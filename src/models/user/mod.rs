use core::str;
use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubAuthResponse {
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubUser {
    // github user id
    pub id: u64,
    /// github user name
    pub login: String,
    /// api url
    pub url: String,
    /// github home page
    pub html_url: String,
    /// avatar url
    pub avatar_url: String,
}
