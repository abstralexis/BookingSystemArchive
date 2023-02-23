#![allow(non_snake_case)]

mod booking;
mod user;
use crate::booking::Booking;
use crate::user::User;
use chrono::prelude::*;
use email_address::*;
use postgres::{Client, Error, NoTls};
use sha256::digest;
use std::str::FromStr;
use uuid::Uuid;

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

    // Eventually make a function for this as it gets messy
    match client
        .query("SELECT * FROM users WHERE uuid = $1", &[&id])?
        .is_empty()
    {
        true => {
            match client
                .query(
                    "SELECT * FROM users WHERE email = $1",
                    &[&user.email.to_string()],
                )?
                .is_empty()
            {
                true => {
                    client.execute(
                        "INSERT INTO users (uuid, email, first_name, last_name, hashed_password) VALUES ($1, $2, $3, $4, $5)",
                        &[&user.uuid, &user.email.to_string(), &user.first_name, &user.last_name, &user.hashed_password]
                    )?;
                }
                false => panic!("User Email not unique."),
            }
        }
        false => panic!("User UUID not unique"),
    }

    // Eventually make a function to do this because it gets messy
    // This will also need to have booking timeframe checking
    match client
        .query("SELECT * FROM bookings WHERE uuid = $1", &[&booking_id])?
        .is_empty()
    {
        true => {
            client.execute(
                "INSERT INTO bookings (uuid, booker_id, start_time, end_time) VALUES ($1, $2, $3, $4)", 
                &[&booking.uuid, &booking.booker_id, &booking.start, &booking.end]
            )?;
        }
        false => panic!("Booking UUID not unique"),
    }

    dbg!(&user, &booking);

    Ok(())
}

fn connect_and_initialise() -> Result<Client, Error> {
    let mut client: Client = Client::connect(
        "postgresql://postgres:AlexisDB@localhost/bookingsystem",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            uuid UUID NOT NULL,
            email VARCHAR NOT NULL,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            hashed_password VARCHAR NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bookings (
            uuid UUID NOT NULL,
            booker_id UUID NOT NULL,
            start_time TIMESTAMP WITH TIME ZONE NOT NULL,
            end_time TIMESTAMP WITH TIME ZONE NOT NULL
        );
    ",
    )?;

    Ok(client)
}
