use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc, vec};

use ntex::web::{
    self,
    types::{Json, Query, State},
};

use crate::{
    errors::WebError,
    models::comment::{Comment, CommentView},
    AppState,
};

#[web::get("/api/rest/article/comments/v1")]
pub async fn get_all_comments(
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, u64>>,
) -> Result<Json<Vec<CommentView>>, WebError> {
    let article_id = query.get("article_id").unwrap();

    // 递归查询
    let comments: Vec<Comment> = sqlx::query!(
        "with recursive com as (select comments.*
            from comments
            where article_id = $1
              and p_id is null
            union
            select comments.*
            from comments
                     inner join com on comments.p_id = com.id)
        select *
        from com;",
        *article_id as i64
    )
    .fetch_all(&state.db_pool)
    .await?
    .iter()
    .map(|result| Comment {
        id: result.id.map(|a| a as u64),
        user_id: result.user_id.unwrap() as u64,
        article_id: result.article_id.unwrap() as u64,
        p_id: result.p_id.map(|a| a as u64),
        content: result.content.clone().unwrap(),
        create_time: result.create_time,
    })
    .collect::<Vec<Comment>>();

    let mut result: Vec<CommentView> = vec![];
    let mut map: HashMap<u64, Rc<RefCell<Vec<CommentView>>>> = HashMap::new();

    for ele in comments {
        let comment_id = ele.id.unwrap();
        let p_id = ele.p_id;
        let s_comment: Rc<RefCell<Vec<CommentView>>> = Rc::new(RefCell::new(vec![]));

        map.insert(comment_id, s_comment.clone());

        if p_id.is_none() {
            result.push(CommentView {
                id: ele.id,
                user_id: ele.user_id,
                article_id: ele.article_id,
                p_id: ele.p_id,
                content: ele.content,
                s_comment: s_comment.clone(),
                create_time: ele.create_time,
            });
        } else {
            let pid = p_id.unwrap();
            map.get(&pid).unwrap().borrow_mut().push(CommentView {
                id: ele.id,
                user_id: ele.user_id,
                article_id: ele.article_id,
                p_id: ele.p_id,
                content: ele.content,
                s_comment: Rc::clone(&s_comment),
                create_time: ele.create_time,
            });
        }
    }

    Ok(Json(result))
}
