#![allow(non_snake_case)]

mod booking;
mod user;
use crate::booking::Booking;
use crate::user::User;
use chrono::prelude::*;
use email_address::*;
use sha256::digest;
use std::str::FromStr;
use uuid::Uuid;
use postgres::{Client, NoTls, Error};


fn main() -> Result<(), Error> {
    let id: Uuid = Uuid::new_v4();
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email = EmailAddress::from_str("johndoe@emailservice.net").unwrap();
    let password = digest("password123");
    let user: User = User::new(id, email, first, last, password);

    let booking_id: Uuid = Uuid::new_v4();
    let user_id: Uuid = user.uuid;
    let start: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 2, 26, 0, 0, 0).unwrap();
    let booking: Booking = Booking::new(booking_id, user_id, start, end);

    let mut client: Client = connect_and_initialise()?;
    client.execute(
        "INSERT INTO users (uuid, email, first_name, last_name, hashed_password) VALUES ($1, $2, $3, $4, $5)",
        &[&user.uuid, &user.email.to_string(), &user.first_name, &user.last_name, &user.hashed_password]
    )?;

    dbg!(&user, &booking);

    Ok(())
}

fn connect_and_initialise() -> Result<Client, Error> {
    let mut client: Client = Client::connect(
        "postgresql://postgres:AlexisDB@localhost/bookingsystem",
        NoTls
    )?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            uuid UUID NOT NULL,
            email VARCHAR NOT NULL,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            hashed_password VARCHAR NOT NULL
        )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS bookings (
            uuid UUID NOT NULL,
            booker_id UUID NOT NULL,
            start_time TIMESTAMP WITH TIME ZONE NOT NULL,
            end_time TIMESTAMP WITH TIME ZONE NOT NULL
        )
    ")?;

    Ok(client)
}