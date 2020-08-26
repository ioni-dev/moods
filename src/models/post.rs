use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub positive_reaction_count: i32,
    pub show_comments: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewPost {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub show_comments: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 15))]
    pub title: String,
    #[validate(length(min = 50, max = 1200))]
    pub content: String,
    pub show_comments: bool,
}
