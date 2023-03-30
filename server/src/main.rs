mod postgres_commands;
use postgres_commands::*;
use http::{Request, Response, StatusCode};
use uuid::Uuid;
use chrono::prelude::*;
use email_address::*;
use tokio;
use postgres::{Client, Error, NoTls};
use rocket::prelude::*;

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), Error> {
    let mut pg_client: Client = connect_and_initialise().await?; 

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    build().mount(routes![index])
}
