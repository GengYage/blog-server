use std::{env, sync::Arc};

use ntex::web::{middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::article::{edit, new, view};

mod article;
mod errors;
mod models;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let url = env::var("DATABASE_URL").expect("please set database_url env");

    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&url[..])
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .service(view::get_articles)
            .service(new::add_article)
            .service(edit::update_article)
    })
    .bind("0.0.0.0:8081")
    .unwrap()
    .run()
    .await
    .unwrap();

    println!("Hello, world!");
}
