#![allow(non_snake_case)]

mod user;
mod booking;
use crate::user::User;
use crate::booking::Booking;
use std::str::FromStr;
use email_address::*;
use sha256::digest;
use uuid::Uuid;
use chrono::prelude::*;

fn main() {
    let id: Uuid = Uuid::new_v4();
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email = EmailAddress::from_str("johndoe@emailservice.net").unwrap();
    let password = digest("password123");
    let user: User = User::new(id, email, first, last, password);

    let booking_id: Uuid = Uuid::new_v4();
    let user_id: Uuid = user.get_id();
    let from: DateTime<Utc> = Utc::now();
    let to: DateTime<Utc> = Utc.with_ymd_and_hms(
        2023, 
        2, 
        26, 
        0, 
        0, 
        0
    ).unwrap();
    let booking: Booking = Booking::new(
        booking_id,
        user_id,
        from,
        to
    );

    dbg!(&user, &booking);
}

