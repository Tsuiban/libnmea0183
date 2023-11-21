use crate::base::*;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Rmc {
    base: Nmea0183Base,
}

impl Rmc {
    pub fn new(base: Nmea0183Base) -> Rmc {
        Rmc { base }
    }

    pub fn timestamp(&self) -> Result<DateTime<Utc>, NmeaError> {
        let timeportion: NaiveTime = self.base.naive_time(0)?;
        let dateportion: NaiveDate = self.base.naive_date(8)?;
        let naivedatetime = NaiveDateTime::new(dateportion, timeportion);
        Ok(DateTime::from_naive_utc_and_offset(naivedatetime, Utc))
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[1] == "A"
    }

    pub fn position(&self) -> PositionError {
        self.base.position(2)
    }

    pub fn sog(&self) -> Result<Speed, NmeaError> {
        Ok(Speed::from_knots(self.base.parameter(6)?))
    }

    pub fn track_made_good(&self) -> F32Error {
        self.base.parameter(7)
    }

    pub fn magnetic_variation(&self) -> F32Error {
        if self.base.parameters[10] == "E" {
            self.base.parameter(9)
        } else {
            Ok(-(self.base.parameter(9)?))
        }
    }

    pub fn faa_mode(&self) -> Option<char> {
        self.base.parameters[11].chars().nth(0)
    }

    pub fn nav_status(&self) -> Option<char> {
        self.base.parameters[12].chars().nth(0)
    }
}
