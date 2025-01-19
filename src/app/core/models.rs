use serde::{Deserialize, Serialize};

pub type UserIdT = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsContent {
    pub id: Option<UserIdT>,
    pub title: String,
    pub content: Option<String>,
    pub authors: String,
    pub publication_date: Option<String>,
    pub url: String,
    pub summary: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserHistoryT(pub Option<Vec<NewsContent>>);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PublicUserWithId {
    pub id: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUserCred {
    pub email: String,
    pub password: String,
}
