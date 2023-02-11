use std::sync::{Arc, Mutex};

use ntex::web::{
    self,
    types::{Json, State},
};

use crate::{errors::WebError, models::article::Article, AppState};

#[web::get("/api/rest/articles")]
pub async fn get_articles(
    state: State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Article>>, WebError> {
    let db_pool = &state.lock().unwrap().db_pool;
    let articles = sqlx::query!("select * from articles")
        .fetch_all(db_pool)
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
