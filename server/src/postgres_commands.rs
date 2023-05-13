use http::{Request, Response, StatusCode};
use uuid::Uuid;
use chrono::prelude::*;
use email_address::*;
use tokio;
use postgres::{Client, Error, NoTls};

pub async fn connect_and_initialise() -> Result<Client, Error> {
    let mut client: Client = Client::connect(
        "postgresql://postgres:AlexisDB@localhost/bookingsystem",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            uuid UUID UNIQUE NOT NULL,
            email VARCHAR UNIQUE NOT NULL,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            hashed_password VARCHAR NOT NULL,
        );

        CREATE TABLE IF NOT EXISTS bookings (
            uuid UUID NOT NULL,
            booker_id UUID NOT NULL,
            start_time TIMESTAMP WITH TIME ZONE NOT NULL,
            end_time TIMESTAMP WITH TIME ZONE NOT NULL,
            FOREIGN KEY (booker_id) REFERENCES users(uuid),
        );
    ",
    )?;

    Ok(client)
}