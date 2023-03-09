use crate::user::User;
use crate::booking::Booking;
use postgres::{Client, Error, NoTls};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotUniqueError;
impl fmt::Display for NotUniqueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item is supposed to be unique but is not")
    }
}

pub fn add_user(client: &mut Client, user: &User) -> Result<(), NotUniqueError> {
    match client
        .query("SELECT * FROM users WHERE uuid = $1", &[&user.uuid]).unwrap()
        .is_empty()
    {
        true => {
            match client
                .query(
                    "SELECT * FROM users WHERE email = $1",
                    &[&user.email.to_string()],
                ).unwrap()
                .is_empty()
            {
                true => {
                    client.execute(
                        "INSERT INTO users (uuid, email, first_name, last_name, hashed_password) VALUES ($1, $2, $3, $4, $5)",
                        &[&user.uuid, &user.email.to_string(), &user.first_name, &user.last_name, &user.hashed_password]
                    ).unwrap();
                    Ok(())
                }
                false => Err(NotUniqueError),
            }
        }
        false => Err(NotUniqueError),
    }
}

pub fn add_booking(client: &mut Client, booking: &Booking) -> Result<(), NotUniqueError> {
    match client
    .query("SELECT * FROM bookings WHERE uuid = $1", &[&booking.uuid]).unwrap()
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

pub fn connect_and_initialise() -> Result<Client, Error> {
    let mut client: Client = Client::connect(
        "postgresql://postgres:AlexisDB@localhost/bookingsystem",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            uuid UUID UNIQUE NOT NULL,
            email VARCHAR UNIQUE NOT NULL,
            first_name VARCHAR NOT NULL,
            last_name VARCHAR NOT NULL,
            hashed_password VARCHAR NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bookings (
            uuid UUID NOT NULL,
            booker_id UUID NOT NULL,
            start_time TIMESTAMP WITH TIME ZONE NOT NULL,
            end_time TIMESTAMP WITH TIME ZONE NOT NULL
        );
    ",
    )?;

    Ok(client)
}

// --- ---------------------------- ---
