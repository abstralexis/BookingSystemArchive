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

    pub fn get_id(&mut self) -> Uuid {
        return self.uuid;
    }

    pub fn get_email(&mut self) -> EmailAddress {
        return self.email;
    }

    pub fn get_first_name(&mut self) -> String {
        return self.first_name;
    }

    pub fn get_last_name(&mut self) -> String {
        return self.last_name;
    }

    pub fn get_hashed_password(&mut self) -> String {
        return self.hashed_password;
    }
}
