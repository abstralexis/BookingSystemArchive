#![allow(dead_code)]

#[derive(Debug)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    password: String,   
}
impl User {
    pub fn new(id: i32, first_name: String, last_name: String, email: String, password: String,) -> User {
        User {
            id: id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            password:password
        }
    }
}