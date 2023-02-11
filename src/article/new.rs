use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{errors::WebError, models::article::Article, AppState};

#[web::post("/api/rest/article/add/v1")]
pub async fn add_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let db_pool = &state.db_pool;
    sqlx::query!(
        "insert into articles(title, content) values ($1, $2)",
        article.title,
        article.content
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}
