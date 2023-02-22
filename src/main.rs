#![allow(non_snake_case)]

mod user;
use std::str::FromStr;

use crate::user::User;

use email_address::*;
use sha256::digest;
use uuid::{uuid, Uuid};

fn main() {
    let id: Uuid = Uuid::new_v4();
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email = EmailAddress::from_str("johndoe@emailservice.net").unwrap();
    let password = digest("password123");

    let user: User = User::new(id, email, first, last, password);

    dbg!(&user);
}
