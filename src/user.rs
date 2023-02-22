#![allow(dead_code)]

pub use uuid::{uuid, Uuid};

pub use email_address::*;
pub use std::str::FromStr;  // Use ::from_str() to gen email

#[derive(Debug)]
pub struct User {
    UUID: Uuid,
    email: EmailAddress,
    first_name: String,
    last_name: String,
}