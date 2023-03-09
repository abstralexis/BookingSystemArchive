#![allow(non_snake_case)]

mod booking;
mod user;
mod database_commands;
use crate::booking::Booking;
use crate::user::User;
use crate::database_commands::*;
use chrono::prelude::*;
use email_address::*;
use postgres::{Client, Error, NoTls};
use sha256::digest;
use std::str::FromStr;
use uuid::Uuid;
use std::fmt;

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

    add_user(&mut client, &user).unwrap();
    add_booking(&mut client, &booking).unwrap();

    dbg!(&user, &booking);

    Ok(())
}