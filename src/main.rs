#![allow(non_snake_case)]

mod user;
use crate::user::User;

fn main() {
    let id: i32 = 1;
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email: String = String::from("johndoe@emailservice.net");
    let password: String = String::from("password");
    let user: User = User::new(id, first, last, email, password);
    
    dbg!(&user);
}
