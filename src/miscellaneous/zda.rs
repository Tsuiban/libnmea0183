use crate::base::*;

use chrono::prelude::*;
use chrono::Duration;

#[derive(Debug)]
pub struct Zda {
    base: Nmea0183Base,
}

impl Zda {
    pub fn new(base: Nmea0183Base) -> Zda {
        Zda { base }
    }

    pub fn timestamp(&self) -> DateTimeError {
        let timeportion = self.base.from_time(0)?.time();
        let date_string = self.base.parameters[1].clone()
            + self.base.parameters[2].clone().as_str()
            + self.base.parameters[3].clone().as_str();
        match NaiveDate::parse_from_str(date_string.as_str(), "%d%m%Y") {
            Ok(dateportion) => {
                let datestamp = NaiveDateTime::new(dateportion, timeportion);
                Ok(DateTime::from_naive_utc_and_offset(datestamp, Utc))
            }
            Err(e) => Err(NmeaError(format!("{}", e))),
        }
    }

    pub fn local_time(&self) -> DateTimeError {
        let utc = self.timestamp()?;
        let hours = self.base.parameter::<i64>(4)?;
        let minutes = self.base.parameter::<i64>(5)? + hours * 60;
        let delta = Duration::minutes(minutes);
        let naive = NaiveDateTime::new(utc.date_naive(), utc.time()) + delta;
        Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
    }
}
