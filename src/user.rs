#![allow(dead_code)]

use email_address::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    UUID: Uuid,
    email: EmailAddress,
    first_name: String,
    last_name: String,
    hashed_password: String,
}
impl User {
    pub fn new(
        UUID: Uuid,
        email: EmailAddress,
        first_name: String,
        last_name: String,
        hashed_password: String,
    ) -> Self {
        User {
            UUID: UUID,
            email: email,
            first_name: first_name,
            last_name: last_name,
            hashed_password: hashed_password,
        }
    }
}
