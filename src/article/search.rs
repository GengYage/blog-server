use std::{collections::HashMap, sync::Arc};

use ntex::web::{
    self,
    types::{Json, Query, State},
};

use crate::{errors::WebError, models::article::Article, AppState};

#[web::get("/api/rest/article/search/v1")]
pub async fn search_by_title_or_content(
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, String>>,
) -> Result<Json<Vec<Article>>, WebError> {
    let keyword = query.get("keyword".into()).unwrap();

    let all_match_article = sqlx::query!(
        "select * from articles where title like $1 or content like $1 order by id",
        format!("%{}%", keyword)
    )
    .fetch_all(&state.db_pool)
    .await?
    .iter()
    .map(|result| Article {
        id: Some(result.id as u64),
        title: result.title.clone(),
        content: result.content.clone(),
        create_time: result.create_time,
        update_time: result.update_time,
    })
    .collect::<Vec<Article>>();

    Ok(Json(all_match_article))
}

#[web::get("/api/rest/article/get/v1")]
pub async fn get_one(
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, i64>>,
) -> Result<Json<Article>, WebError> {
    let id = query.get("id".into()).unwrap_or(&0);

    if *id == 0 {
        return Err(WebError::NotFound("article not found".into()));
    }

    let result = sqlx::query!("select * from articles where id = $1", id)
        .fetch_one(&state.db_pool)
        .await?;

    Ok(Json(Article {
        id: Some(result.id as u64),
        title: result.title.clone(),
        content: result.content.clone(),
        create_time: result.create_time,
        update_time: result.update_time,
    }))
}
