#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate log;

#[cfg(test)]
mod test;

mod server;
mod routes;
mod context;
mod blog;

use server::start_server;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    start_server().launch();
} 