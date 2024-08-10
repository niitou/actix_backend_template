use entity::post::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDto{
    pub post_title : String,
    pub post_content : String,
    pub post_description : Option<String>
}


impl PostDto {
    pub fn into_active_model(self) -> PostActiveModel{
        PostActiveModel{
            title: Set(self.post_title.to_owned()),
            content : Set(self.post_content.to_owned()),
            description : Set(self.post_description.to_owned()),
            ..Default::default()
        }
    }
}