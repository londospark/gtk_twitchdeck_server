extern crate actix_web;

use self::actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn start_server() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8087")
        .unwrap()
        .run();
}
