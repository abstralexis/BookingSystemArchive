#![allow(dead_code)]

use email_address::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    uuid: Uuid,
    email: EmailAddress,
    first_name: String,
    last_name: String,
    hashed_password: String,
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

    pub fn get_id(&self) -> Uuid {
        return self.uuid;
    }
}
