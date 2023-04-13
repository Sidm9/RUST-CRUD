#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod errors;
mod handlers;
mod models;
mod schema;

// custom type 
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let port_url = std::env::var("PORT").expect("PORT must be set");
    let host_url = std::env::var("HOST").expect("HOST must be set");

    //? https://docs.rs/r2d2/0.8.5/r2d2/index.html
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::put().to(handlers::update_user)) 
            .route("users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(format!{"{}:{}",host_url,port_url})?
    .run()
    .await
}   
