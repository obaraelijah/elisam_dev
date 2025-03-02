#[macro_use]
extern crate rocket;
extern crate log;
#[cfg(test)]
mod tests;

mod blog;
mod context;
mod routes;
mod server;

use context::init_context;
use server::start_server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    init_context();
    let server = start_server();
    server.launch().await?;
    Ok(())
}