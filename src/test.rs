#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    fn get_client() -> Client {
        use crate::server::start_server;
        Client::new(start_server()).expect("Failed to start server!")
    }

    fn static_pages_should_return_200() {
        let routes = vec![
            "/"
        ];
        let client = get_client();
        for route in routes {
            let resp = client.get(route).dispatch();
            assert_eq!(resp.Status(), Status.Ok());
        }
    }

}