use super::dto::CommentDto;
use crate::error::MyError;
use entity::comment::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

pub async fn add_comment(db: &DatabaseConnection, comment_info: CommentDto) -> Result<CommentModel, MyError> {
    let comment = comment_info.into_active_model();

    let comment = comment.insert(db).await?;

    Ok(comment)
}

pub async fn get_comment_by_id(db: &DatabaseConnection, id: i32) -> Result<CommentModel, MyError> {
    Comment::find_by_id(id)
        .one(db)
        .await
        .map_err(MyError::DbErr)?
        .ok_or("Comment not found !".into())
        .map_err(MyError::NotFound)
}


pub async fn update_comment(db: &DatabaseConnection, id: i32, comment_info: CommentDto) -> Result<CommentModel, MyError> {
    let mut comment = comment_info.into_active_model();
    comment.id = Set(id);

    let comment = comment.update(db).await?;

    Ok(comment)
}


pub async fn delete_comment(db: &DatabaseConnection, id: i32) -> Result<bool, MyError>{
    let comment = get_comment_by_id(db, id).await?;
    let res = comment.delete(db).await?;

    Ok(res.rows_affected > 0)
}
