#![allow(non_snake_case)]

mod booking;
mod database_commands;
mod user;
use crate::booking::Booking;
use crate::database_commands::*;
use crate::user::User;
use chrono::prelude::*;
use email_address::*;
use postgres::{Client, Error};
use postgrest::Postgrest;
use sha256::digest;
use std::str::FromStr;
use uuid::Uuid;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello world!" }</h1>
        </div>
    }
}

fn main() {
    let id: Uuid = Uuid::new_v4();
    let first: String = String::from("John");
    let last: String = String::from("Doe");
    let email = EmailAddress::from_str("johndoe@emailservice.net").unwrap();
    let password = digest("password123");
    let user: User = User::new(id, email, first, last, password);

    let booking_id: Uuid = Uuid::new_v4();
    let user_id: Uuid = user.uuid;
    let start: DateTime<Utc> = Utc::now();
    let end: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 3, 11, 0, 0, 0).unwrap();
    let booking: Booking = Booking::new(booking_id, user_id, start, end).unwrap();

    // let mut client: Client = connect_and_initialise().unwrap();

    // add_user(&mut client, &user).unwrap();
    // add_booking(&mut client, &booking).unwrap();

    // dbg!(&user, &booking);

    yew::Renderer::<App>::new().render();
}
