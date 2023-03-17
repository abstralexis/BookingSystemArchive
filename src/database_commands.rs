use crate::booking::Booking;
use crate::user::User;
use postgrest::Postgrest;
use std::fmt::{self, Error};

#[derive(Debug, Clone)]
pub struct NotUniqueError;
impl fmt::Display for NotUniqueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item is supposed to be unique but is not")
    }
}

pub fn add_user(client: &mut Postgrest, user: &User) -> Result<(), Error> {
    let query = format!(
        "{{
        \"uuid\": \"{}\",
        \"email\": \"{}\",
        \"first_name\": \"{}\",
        \"last_name\": \"{}\", 
        \"hashed_password\": \"{}\"
    }}",
        &user.uuid,
        &user.email.to_string(),
        &user.first_name,
        &user.last_name,
        &user.hashed_password
    );

    client.from("users").insert(query).execute();

    Ok(())
}

pub fn add_booking(client: &mut Postgrest, booking: &Booking) -> Result<(), NotUniqueError> {
    match client
        .query("SELECT * FROM bookings WHERE uuid = $1", &[&booking.uuid])
        .unwrap()
        .is_empty()
    {
        true => {
            client.execute(
            "INSERT INTO bookings (uuid, booker_id, start_time, end_time) VALUES ($1, $2, $3, $4)", 
            &[&booking.uuid, &booking.booker_id, &booking.start, &booking.end]
        ).unwrap();
            Ok(())
        }
        false => Err(NotUniqueError),
    }
}
