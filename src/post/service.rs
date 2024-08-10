use super::dto::PostDto;
use crate::error::MyError;
use entity::post::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, Set};

pub async fn add_post(db: &DatabaseConnection, post_info: PostDto) -> Result<PostModel, MyError> {
    let post = post_info.into_active_model();

    let post = post.insert(db).await?;

    Ok(post)
}

pub async fn get_post_by_id(db: &DatabaseConnection, id: i32) -> Result<PostModel, MyError> {
    Post::find_by_id(id)
        .one(db)
        .await
        .map_err(MyError::DbErr)?
        .ok_or("Post not found !".into())
        .map_err(MyError::NotFound)
}


pub async fn update_post(db: &DatabaseConnection, id: i32, post_info: PostDto) -> Result<PostModel, MyError> {
    let mut post = post_info.into_active_model();
    post.id = Set(id);

    let post = post.update(db).await?;

    Ok(post)
}


pub async fn delete_post(db: &DatabaseConnection, id: i32) -> Result<bool, MyError>{
    let post = get_post_by_id(db, id).await?;
    let res = post.delete(db).await?;

    Ok(res.rows_affected > 0)
}

pub async fn get_post_all(db: &DatabaseConnection, page: u64) -> Result<Vec<PostModel>, MyError> {
    let paginator = Post::find().paginate(db, 50); 

    let data = paginator.fetch_page(page).await.map_err(MyError::DbErr);

    return data;
}