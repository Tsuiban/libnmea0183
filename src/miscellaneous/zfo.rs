use crate::base::*;

use chrono::prelude::*;
use chrono::{Duration};

#[derive(Debug)]
pub struct Zfo {
    base: Nmea0183Base,
}

impl Zfo {
    pub fn new(base: Nmea0183Base) -> Zfo { Zfo { base } }
    pub fn timestamp(&self) -> DateTimeError {
        self.base.from_time(0)
    }

    pub fn elapsed_time(&self) -> Result<Duration, NmeaError> {
        if let Ok(t) = self.base.from_time(1) {
            let o = DateTime::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    t.date_naive(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()), Utc);
            Ok(t - o)
        } else {
            Err(NmeaError("Invalid elapsed time.".to_string()))
        }
    }

    pub fn waypoint_id(&self) -> String {
        self.base.parameters[2].clone()
    }
}
