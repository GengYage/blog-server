use std::{collections::HashMap, sync::Arc};

use ntex::web::{
    self,
    types::{Query, State},
    HttpResponse, Responder,
};

use crate::{
    errors::WebError,
    models::user::auth::{Admin, User},
    AppState,
};

#[web::delete("/api/rest/comment/delete/v1")]
pub async fn delete_comment(
    user: User,
    admin: Option<Admin>,
    query: Query<HashMap<String, u64>>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let user_id = user.id;

    let comment_id = query.get("comment_id");
    // 非管理员智能删除自己的评论
    let affected_rows = if admin.is_none() {
        sqlx::query!(
            "delete from comments where id = $1 and user_id = $2",
            comment_id.map(|a| *a as i64),
            user_id as i64
        )
        .execute(&state.db_pool)
        .await?
    } else {
        sqlx::query!(
            "delete from comments where id = $1",
            comment_id.map(|a| *a as i64)
        )
        .execute(&state.db_pool)
        .await?
    }
    .rows_affected();

    if affected_rows > 0 {
        Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
    } else {
        Err(WebError::BadRequest(
            "you can't delete other user's comment or comment is not exits".into(),
        ))
    }
}
