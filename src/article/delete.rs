use std::sync::Arc;

use ntex::{
    util::HashMap,
    web::{
        self,
        types::{Query, State},
        HttpResponse, Responder,
    },
};

use crate::{errors::WebError, models::user::auth::User, AppState};

#[web::delete("/api/rest/article/delete/v1")]
pub async fn delete_article(
    _: User,
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, i64>>,
) -> Result<impl Responder, WebError> {
    let db_pool = &state.db_pool;
    let id = query.get("id").unwrap_or(&0);

    sqlx::query!("delete from articles where id = $1", *id)
        .execute(db_pool)
        .await?;

    Ok(HttpResponse::Created().body(r#"{"result": "ok"}"#))
}
