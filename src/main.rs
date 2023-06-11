/* Publish all same-level modules */
pub mod app;
pub mod app_settings;
pub mod data;

use std::fs;

/* Import all necessary packages */
// #[macro_use] extern crate rocket;
use app::modules;
use rocket;
use sqlx::postgres::PgPoolOptions;

/* Global Variables */

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    /* Get the db connection pool */
    let con: sqlx::Pool<sqlx::Postgres> = match db_conn().await {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e.to_string());
            panic!()
        }
    };

    //let con: sqlx::Pool<sqlx::Postgres> = db_conn().await;
    print!("DB connected...");

    /* Launch the server */
    let _rocket: rocket::Rocket<rocket::Ignite> = rocket::build()
        //.mount("/", routes![hello])
        /* Areas */
        /* Home routes */
        .mount("/", modules::home::get_routes())
        .launch()
        .await?;
    print!("Server launched...");

    Ok(())
}

async fn db_conn() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let data = fs::read_to_string("app_settings.json")
        .expect("Unable to read file");

    let db: app_settings::DBSettings = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");


    // let url = "postgres://postgres:Admin12345@localhost:5432/rocket";
    let url = "postgres://postgres:Admin12345@localhost:5432/rocket";
    let url = format!("postgres://{}:{}@{}:{}/{}", 
        db.username,
        db.password,
        db.hostname,
        db.port,
        db.name    
    );
    println!("DB url: {}", url);
    // let url = "host=PostgreSQL 15/localhost port=5432 dbname=rocket user=postgres password=Admin12345 sslmode=prefer connect_timeout=10";

    let p: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    return Ok(p);
}
