use std::{env, sync::Arc};

use ntex::web::{middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod article;
mod comment;
mod errors;
mod models;
mod user;

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
    // 校验是否设置了相关环境变量,提前报错
    let _ = env::var("CLIENT_ID").expect("please set client_id env");
    let _ = env::var("CLIENT_SECRET").expect("please set client_secret env");

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
            .service(article::view::get_articles)
            .service(article::new::add_article)
            .service(article::edit::update_article)
            .service(article::search::search_by_title_or_content)
            .service(article::search::get_one)
            .service(article::delete::delete_article)
            .service(user::login::github_login)
            .service(comment::new::new_comment)
            .service(comment::view::get_all_comments)
    })
    .bind("0.0.0.0:8081")
    .unwrap()
    .run()
    .await
    .unwrap();

    println!("Hello, world!");
}
