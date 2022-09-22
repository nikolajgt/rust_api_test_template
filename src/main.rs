#[macro_use]
use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, get_user, update_user, delete_user};
use repository::mongodb_repo::MongoRepo;

mod api;
mod repository;
mod models;



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await



}



