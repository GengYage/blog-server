use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{errors::WebError, models::article::Article, AppState};

#[web::post("/api/rest/article/update/v1")]
pub async fn update_article(
    article: Json<Article>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let db_pool = &state.db_pool;

    let id = match article.id {
        Some(id) => id,
        None => return Err(WebError::BadRequest("请传入你要修改的文章id".into())),
    };

    sqlx::query!(
        "update articles set title = $1 , content = $2 where id = $3",
        article.title,
        article.content,
        id as i64,
    )
    .execute(db_pool)
    .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}