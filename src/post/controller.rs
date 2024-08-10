use super::{dto::PostDto, service};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;

#[post("/")]
pub async fn add_post(post: web::Json<PostDto>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let new_post = service::add_post(&db, post.clone()).await?;

    Ok(HttpResponse::Ok().json(new_post))
}


#[get("/{id}")]
pub async fn get_post(path: web::Path<(i32,)>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let post = service::get_post_by_id(&db, path.into_inner().0).await?;
    Ok(HttpResponse::Ok().json(post))
}


#[put("/{id}")]
pub async fn update_post(path: web::Path<(i32,)>, post:web::Json<PostDto>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let post = post.into_inner();
    let updated_post = service::update_post(&db, path.into_inner().0, post).await?;

    Ok(HttpResponse::Ok().json(updated_post))
}


#[delete("/{id}")]
pub async fn delete_post(path: web::Path<(i32,)>, db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    let _ = service::delete_post(&db, path.into_inner().0).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/page/{num}")]
pub async fn get_post_all(db: web::Data<DatabaseConnection>, path: web::Path<(u64,)>) -> Result<HttpResponse, Error> {
    let posts = service::get_post_all(&db, path.into_inner().0).await?;

    Ok(HttpResponse::Ok().json(posts))
}