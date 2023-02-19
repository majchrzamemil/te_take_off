mod endpoints;
mod errors;
mod repository;

use rocket::routes;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    use crate::endpoints::*;
    use crate::repository::*;

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL needs to be set");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .map_err(|e| {
            eprintln!("Error while creating db pool: {}", e);
            std::process::exit(-1);
        })
        .unwrap(); //This will never error out on unwrap so I am going to leave it like this

    match sqlx::migrate!().run(&db).await {
        Ok(_) => println!("Migrated successfully"),
        Err(e) => eprintln!("Migration failed, reason: {}", e),
    }

    let repository = Repository::new(&db);
    let _rocket = rocket::build()
        .mount("/", routes![index, add_opinion, get_opinions])
        .manage(repository)
        .launch()
        .await?;

    Ok(())
}
