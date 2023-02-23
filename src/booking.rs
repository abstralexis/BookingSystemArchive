#![allow(dead_code)]

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Booking {
    pub uuid: Uuid,
    pub booker_id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
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

    pub fn update_start_time(&mut self, start: DateTime<Utc>) {
        self.start = start;
    }

    pub fn update_end_time(&mut self, end: DateTime<Utc>) {
        self.end = end;
    }
}
