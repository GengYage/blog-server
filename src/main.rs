use std::env;

use errors::WebError;
use ntex::web::{self, middleware, App, HttpServer};

mod errors;

#[ntex::main]
async fn main() {
    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(error)
    })
    .bind("0.0.0.0:8081")
    .unwrap()
    .run()
    .await
    .unwrap();

    println!("Hello, world!");
}

#[web::get("/")]
async fn index() -> String {
    "Hello world".into()
}

#[web::get("/error")]
async fn error() -> Result<String, WebError> {
    Err(WebError::NotFound("Not found".into()))
}
