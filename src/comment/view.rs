use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc, vec};

use ntex::web::{
    self,
    types::{Json, Query, State},
};

use crate::{errors::WebError, models::comment::CommentView, AppState};

#[web::get("/api/rest/article/comments/v1")]
pub async fn get_all_comments(
    state: State<Arc<AppState>>,
    query: Query<HashMap<String, u64>>,
) -> Result<Json<Vec<CommentView>>, WebError> {
    let article_id = query.get("article_id").unwrap();

    // 递归查询
    let comments: Vec<CommentView> = sqlx::query!(
        "with recursive res as (select c.*, u.name, u.avatar_url
            from comments c
                     left join users u on c.user_id = u.id
            where article_id = $1
              and p_id is null
            union
            select c.*, u.name, u.avatar_url
            from comments c
                     left join users u on u.id = c.user_id
                     inner join res r on c.p_id = r.id)
        select *
        from res;",
        *article_id as i64
    )
    .fetch_all(&state.db_pool)
    .await?
    .iter()
    .map(|result| CommentView {
        id: result.id.map(|a| a as u64),
        user_id: result.user_id.unwrap() as u64,
        article_id: result.article_id.unwrap() as u64,
        p_id: result.p_id.map(|a| a as u64),
        content: result.content.clone().unwrap(),
        user_name: result.name.clone(),
        user_avatar_url: result.avatar_url.clone(),
        s_comment: Rc::new(RefCell::new(vec![])),
        create_time: result.create_time,
    })
    .collect::<Vec<CommentView>>();

    let mut result: Vec<CommentView> = vec![];
    let mut map: HashMap<u64, Rc<RefCell<Vec<CommentView>>>> = HashMap::new();

    for ele in comments {
        let comment_id = ele.id.unwrap();
        let p_id = ele.p_id;
        // 子评论加入到map中暂存
        map.insert(comment_id, ele.s_comment.clone());

        if p_id.is_none() {
            result.push(ele);
        } else {
            let pid = p_id.unwrap();
            // 取出子评论列表,添加子评论
            map.get(&pid).unwrap().borrow_mut().push(ele);
        }
    }

    Ok(Json(result))
}
