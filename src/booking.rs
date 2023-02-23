#![allow(dead_code)]

use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Booking {
    uuid: Uuid,
    booker_id: Uuid,
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}
impl Booking {
    pub fn new(
        uuid: Uuid,
        booker_id: Uuid,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Self {
        Booking {
            uuid,
            booker_id,
            from,
            to,
        }
    }
}