#![feature(proc_macro_hygiene, decl_macro)]
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

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    init_context();
    start_server().launch();
}
