use actix_cors::Cors;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use actix_backend_template::{comment, config, post};
use env_logger::Env;
use log::info;
use migration::{Migrator, MigratorTrait};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // init config from environment
    let config = config::Config::from_env().unwrap();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // init database connection
    let db = sea_orm::Database::connect(config.database_url())
        .await
        .unwrap();

    // migration database
    Migrator::up(&db, None).await.unwrap();

    // init state
    let state = web::Data::new(db);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(state.clone())
            .service(hello)
            .service(
                web::scope("/api")
                    .configure(post::config)
                    .configure(comment::config),
            )
    })
    .bind(config.server_addr())?;

    info!("Starting server at http://{}/", config.server_addr());
    server.run().await
}
