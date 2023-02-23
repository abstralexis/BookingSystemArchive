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
    let mut user: User = User::new(id, email, first, last, password);

    let booking_id: Uuid = Uuid::new_v4();
    let user_id: Uuid = user.get_id();
    let start: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 2, 26, 0, 0, 0).unwrap();
    let booking: Booking = Booking::new(booking_id, user_id, start, end);

    let mut client: Client = connect_and_initialise()?;
    client.execute(
        "INSERT INTO users (uuid, email, first_name, last_name, hashed_password) VALUES ($1, $2, $3, $4, $5)",
        &[
            &user.get_id(), 
            &user.get_email().to_string(), 
            &user.get_first_name(), 
            &user.get_last_name(), 
            &user.get_hashed_password()
        ]
    )?;

    dbg!(&user, &booking);

    Ok(())
}

fn connect_and_initialise() -> Result<Client, Error> {
    let mut client: Client = Client::connect(
        "postgresql://postgres:alexisdb@localhost/bookingsystem",
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
            from TIMESTAMP WITH TIMEZONE NOT NULL,
            to TIMESTAMP WITH TIMEZONE NOT NULL
        )
    ")?;

    Ok(client)
}