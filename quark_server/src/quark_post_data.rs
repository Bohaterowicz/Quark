use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QuarkPost {
    pub id: i64,
    pub username: String,
    pub post_time: String,
    pub post_content: String,
    pub post_attachments: String,
}

#[derive(Deserialize)]
pub struct QuarkPostPrototype {
    pub username: String,
    pub post_content: String,
    pub post_attachments: String,
}
