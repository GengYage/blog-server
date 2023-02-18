use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{
    errors::WebError,
    models::{article::Article, user::auth::User},
    AppState,
};

#[web::post("/api/rest/article/add/v1")]
pub async fn add_article(
    _: User,
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    sqlx::query!(
        "insert into articles(title, content) values ($1, $2)",
        article.title,
        article.content
    )
    .execute(&state.db_pool)
    .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}
