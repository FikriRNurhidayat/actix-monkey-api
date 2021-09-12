use actix_web::{App, HttpServer, web};
use sqlx::{Pool, Error};
use sqlx::postgres::{PgPoolOptions as PoolOptions, Postgres};
use dotenv::dotenv;
use std::env;

mod controllers;
mod entities;
mod services;
mod repositories;

async fn connect_to_database(url: &String) -> Result<Pool<Postgres>, Error> {
    Ok(PoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await?
    )
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port: String = env::var("PORT").unwrap();
    let database_url: String = env::var("DATABASE_URL").unwrap();

    match connect_to_database(&database_url).await {
        Ok(_) => println!("Connected!"),
        Err(_) => panic!("Duh")
    };

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(controllers::main_controller::root))
            .route("/monkeys/register", web::post().to(controllers::monkey_controller::register))
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
