#![allow(dead_code)]

use email_address::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub uuid: Uuid,
    pub email: EmailAddress,
    pub first_name: String,
    pub last_name: String,
    pub hashed_password: String,
}
impl User {
    pub fn new(
        uuid: Uuid,
        email: EmailAddress,
        first_name: String,
        last_name: String,
        hashed_password: String,
    ) -> Self {
        User {
            uuid,
            email,
            first_name,
            last_name,
            hashed_password,
        }
    }
}
