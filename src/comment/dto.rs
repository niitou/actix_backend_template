use entity::comment::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDto{
    pub post_id : i32,
    pub comment_title : String,
    pub comment_content : String,
}


impl CommentDto {
    pub fn into_active_model(self) -> CommentActiveModel{
        CommentActiveModel{
            foreign_id : Set(self.post_id.to_owned()),
            title: Set(self.comment_title.to_owned()),
            content : Set(self.comment_content.to_owned()),
            ..Default::default()
        }
    }
}