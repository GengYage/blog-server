use std::{env, sync::Arc};

use cookie::{time::Duration, Cookie};
use ntex::{
    http::Client,
    http::Response,
    web::{
        self,
        types::{Json, State},
        Responder,
    },
};

use crate::{
    errors::WebError,
    models::user::{GithubAuthResponse, GithubUser, Login},
    AppState,
};

#[web::post("/api/rest/auth/login/v1")]
pub async fn github_login(
    code: Json<Login>,
    state: State<Arc<AppState>>,
) -> Result<impl Responder, WebError> {
    let code = &code.code;
    let client = Client::new();

    // 从环境变量读取client_id和client_secret
    let client_id = env::var("CLIENT_ID").expect("please set client_id env");
    let client_secret = env::var("CLIENT_SECRET").expect("please set client_secret env");

    let mut github_response = client
        .post(format!(
            "https://github.com/login/oauth/access_token?client_id={client_id}&client_secret={client_secret}&code={code}"
        ))
        .header("Accept", "application/json")
        .send()
        .await?;

    let access_token = match github_response.json::<GithubAuthResponse>().await {
        Ok(r) => r.access_token,
        Err(_) => {
            return Err(WebError::AuthFailed(
                "invalid code,please login again use github".into(),
            ))
        }
    };

    let mut user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        .header("User-Agent", "isbest-blog")
        .send()
        .await?;

    let github_user = user_info.json::<GithubUser>().await.unwrap();

    sqlx::query!(
        "insert into users (id, name,avatar_url, url, html_url)
    values($1,$2,$3,$4,$5) 
    on conflict (id) do 
    update set name= $2,avatar_url=$3,url=$4,html_url=$5 ",
        github_user.id as i64,
        github_user.login,
        github_user.avatar_url,
        github_user.url,
        github_user.html_url
    )
    .execute(&state.db_pool)
    .await?;

    let mut response = Response::Ok().body(format!(
        r#"{{"result": "ok", "name":"{}"}}"#,
        github_user.login
    ));

    // 设置cookie
    let mut cookie = Cookie::new("ACCESS_TOKEN", access_token);
    cookie.set_path("/");
    cookie.set_max_age(Duration::days(7));
    // cookie.set_http_only(true);
    let _ = response.add_cookie(&cookie);

    Ok(response)
}
