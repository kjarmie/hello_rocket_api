/* Publish all same-level modules */
pub mod app;
pub mod app_settings;
pub mod data;

use std::fs;

/* Import all necessary packages */
// #[macro_use] extern crate rocket;
use app::modules;
use rocket::{Rocket, Build};
use sqlx::postgres::PgPoolOptions;

/* Global Variables */

fn configure(rocket:rocket::Rocket<rocket::Build>) -> rocket::Rocket<rocket::Build> {
    /* ROUTES */    
    rocket.mount("/", modules::home::get_routes());/* Home routes */


    return rocket;
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    /* CREATE ROCKET */
    let rocket: rocket::Rocket<rocket::Build> = rocket::build();

    /* CONFIGURE */    
    rocket = configure(rocket);

    /* DB  */


    let rocket: rocket::Rocket<rocket::Ignite> = rocket()
        .ignite().await?
        .launch().await?;
        
    Ok(());




// let _rocket: rocket::Rocket<rocket::Ignite>

    /* Launch the server */
    if let Err(e) = rocket().launch().await {
        println!("Server failed to launched: {}", e.to_string());
    }
   
    
    /* Get the db connection pool */
    let con: sqlx::Pool<sqlx::Postgres> = match db_conn().await {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err.to_string());
            panic!();
            // return Err(err);
        }
    };

    //let con: sqlx::Pool<sqlx::Postgres> = db_conn().await;
    print!("DB connected...");

    

    Ok(())
}

async fn db_conn() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    println!("{}",std::env::current_dir().unwrap().to_str().unwrap());


    let data = fs::read_to_string("./src/app_settings.json")
        .expect("Unable to read file");

    let settings: app_settings::AppSettings = serde_json::from_str(&data)
        .expect("JSON does not have correct format.");

    let db: app_settings::DBSettings = settings.db;

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

