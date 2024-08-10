use super::{dto::CommentDto, service};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/")]
pub async fn add_comment(comment: web::Json<CommentDto>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let new_comment = service::add_comment(&db, comment.clone()).await?;

    Ok(HttpResponse::Ok().json(new_comment))
}


#[get("/{id}")]
pub async fn get_comment(path: web::Path<(i32,)>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let comment = service::get_comment_by_id(&db, path.into_inner().0).await?;
    Ok(HttpResponse::Ok().json(comment))
}


#[put("/{id}")]
pub async fn update_comment(path: web::Path<(i32,)>, comment:web::Json<CommentDto>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let comment = comment.into_inner();
    let updated_comment = service::update_comment(&db, path.into_inner().0, comment).await?;

    Ok(HttpResponse::Ok().json(updated_comment))
}


#[delete("/{id}")]
pub async fn delete_comment(path: web::Path<(i32,)>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let _ = service::delete_comment(&db, path.into_inner().0).await?;

    Ok(HttpResponse::Ok().finish())
}