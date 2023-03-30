#![allow(dead_code)]

use chrono::prelude::*;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct StartsAfterEndError;
impl fmt::Display for StartsAfterEndError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Email starts with a date that is after or equal to the end date."
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Booking {
    pub uuid: Uuid,
    pub booker_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
impl Booking {
    pub fn new(
        uuid: Uuid,
        booker_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Booking, StartsAfterEndError> {
        if end <= start {
            return Err(StartsAfterEndError);
        }

        Ok(Booking {
            uuid,
            booker_id,
            start,
            end,
        })
    }

    pub fn update_start_time(&mut self, start: DateTime<Utc>) {
        self.start = start;
    }

    pub fn update_end_time(&mut self, end: DateTime<Utc>) {
        self.end = end;
    }
}
