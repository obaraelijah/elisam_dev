#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    fn get_client() -> Client {
        use crate::server::start_server;
        Client::new(start_server()).expect("Failed to start server!")
    }

    #[test]
    fn static_pages_should_return_200() {
        let routes = vec![
            "/"
        ];
        let client = get_client();
        for route in routes {
            let resp = client.get(route).dispatch();
            assert_eq!(resp.status(), Status::Ok);
        }
    }

    #[test]
    fn unknown_pages_should_return_404() {
        let routes = vec!["/adawdawd", "/ada2wdawd/adawdw", "/ada3dawd/afwf/afwafa"];
        let client = get_client();
        for route in routes {
            let resp = client.get(route).dispatch();
            println!("{}", route);
            assert_eq!(resp.status(), Status::NotFound);
        }
    }

    #[test]
    fn five_hundred_page_should_return_500() {
        let client = get_client();
        let resp = client.get("/500").dispatch();
        assert_eq!(resp.status(), Status::InternalServerError);
    }

}
