use std::{env, future::Future, pin::Pin, sync::Arc};

use cookie::Cookie;
use ntex::{
    http::{Client, HttpMessage},
    web::{ErrorRenderer, FromRequest},
};

use crate::{errors::WebError, AppState};

use super::GithubUser;

/// 所有用户类型,用于权限校验
#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
}

/// 管理员类型,用于权限校验
#[derive(Debug, Clone)]
pub struct Admin {
    pub id: u64,
}

impl<E: ErrorRenderer> FromRequest<E> for User {
    type Error = WebError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let fun = async move {
            let access_token = match access_token {
                Some(token) => token,
                None => return Err(WebError::AuthFailed("you must be login first".into())),
            };

            let user_id = get_user_id_by_token(&access_token).await?;

            if sqlx::query!("select * from users where id = $1", user_id as i64)
                .fetch_optional(&db_pool)
                .await?
                .is_none()
            {
                // 查询不到该用户信息
                return Err(WebError::AuthFailed("you must be login first".into()));
            } else {
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fun)
    }
}

impl<E: ErrorRenderer> FromRequest<E> for Admin {
    type Error = WebError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &ntex::web::HttpRequest, _: &mut ntex::http::Payload) -> Self::Future {
        let db_pool = Arc::clone(req.app_state::<Arc<AppState>>().unwrap())
            .db_pool
            .clone();

        let access_token = req.cookie("ACCESS_TOKEN");

        let admin_id = env::var("ADMIN_ID").expect("please set client_secret env");

        let fun = async move {
            let access_token = match access_token {
                Some(token) => token,
                None => return Err(WebError::AuthFailed("you must be login first".into())),
            };

            let user_id = get_user_id_by_token(&access_token).await?;

            // 提前判断是不是管理员id,是的话才会继续判断管理员是否登陆过
            if admin_id != format!("{}", user_id) {
                return Err(WebError::AuthFailed("you must be admin".into()));
            }

            // 查询用户是否登陆过
            if sqlx::query!("select * from users where id = $1", user_id as i64)
                .fetch_optional(&db_pool)
                .await?
                .is_none()
            {
                // 查询不到该用户信息
                return Err(WebError::AuthFailed("you must be login first".into()));
            }

            Ok(Self { id: user_id })
        };

        Box::pin(fun)
    }
}

async fn get_user_id_by_token(access_token: &Cookie<'_>) -> Result<u64, WebError> {
    let client = Client::new();
    let mut user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.value())
        .header("User-Agent", "isbest-blog")
        .send()
        .await?;

    let github_user = user_info.json::<GithubUser>().await.map_err(|_| {
        WebError::AuthFailed(format!(
            "invalid access token, github oauth status:{:#?}",
            user_info.status()
        ))
    })?;

    Ok(github_user.id)
}
