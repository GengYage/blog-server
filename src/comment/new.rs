use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
    HttpResponse, Responder,
};

use crate::{
    errors::WebError,
    models::{comment::Comment, user::auth::User},
    AppState,
};

// 无需再次校验用户是否存在,在权限校验出已经能确保用户必然存在
#[web::post("/api/rest/comment/add/v1")]
pub async fn new_comment(
    _: User,
    state: State<Arc<AppState>>,
    comment: Json<Comment>,
) -> Result<impl Responder, WebError> {
    let article_id = comment.article_id;

    if sqlx::query!("select * from articles where id = $1", article_id as i64)
        .fetch_optional(&state.db_pool)
        .await?
        .is_none()
    {
        return Err(WebError::BadRequest(format!(
            "article is not exits, id:{}",
            article_id
        )));
    }

    // 保存评论
    sqlx::query!(
        "insert into comments (article_id, user_id, p_id, content) values ($1,$2,$3,$4)",
        comment.article_id as i64,
        comment.user_id as i64,
        comment.p_id.map(|a| a as i64), // Option<u64> -> Option<i64>
        comment.content
    )
    .execute(&state.db_pool)
    .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}
