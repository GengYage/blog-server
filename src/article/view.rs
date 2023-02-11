use std::sync::Arc;

use ntex::web::{
    self,
    types::{Json, State},
};

use crate::{errors::WebError, models::article::Article, AppState};

#[web::get("/api/rest/articles/v1")]
pub async fn get_articles(state: State<Arc<AppState>>) -> Result<Json<Vec<Article>>, WebError> {
    let articles = sqlx::query!("select * from articles order by id")
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

    Ok(Json(articles))
}
