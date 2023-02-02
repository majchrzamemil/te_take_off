mod endpoints;
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
        .unwrap();

    let repository = Repository::new(&db);
    let _rocket = rocket::build()
        .mount("/", routes![index, add_opinion])
        .manage(repository)
        .launch()
        .await?;

    Ok(())
}
