#![allow(dead_code)]

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Booking {
    uuid: Uuid,
    booker_id: Uuid,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}
impl Booking {
    pub fn new(uuid: Uuid, booker_id: Uuid, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Booking {
            uuid,
            booker_id,
            start,
            end,
        }
    }

    pub fn get_id(&self) -> Uuid {
        return self.uuid;
    }

    pub fn get_booker_id(&self) -> Uuid {
        return self.booker_id;
    }

    pub fn get_timestamps(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        /*
            Returns a timestamp pair of `DateTime<Utc>`, the first
            indicating the timestamp of the start of the booking and
            the second indicates the timestamp of the end of the booking.
        */
        return (self.start, self.end);
    }

    pub fn update_start_time(&mut self, start: DateTime<Utc>) {
        self.start = start;
    }

    pub fn update_end_time(&mut self, end: DateTime<Utc>) {
        self.end = end;
    }
}
