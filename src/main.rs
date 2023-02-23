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

fn main() {
    let id: Uuid = Uuid::new_v4();
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email = EmailAddress::from_str("johndoe@emailservice.net").unwrap();
    let password = digest("password123");
    let user: User = User::new(id, email, first, last, password);

    let booking_id: Uuid = Uuid::new_v4();
    let user_id: Uuid = user.get_id();
    let start: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 2, 26, 0, 0, 0).unwrap();
    let booking: Booking = Booking::new(booking_id, user_id, start, end);

    dbg!(&user, &booking);
}
