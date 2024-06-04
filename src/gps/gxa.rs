use crate::base::*;

#[derive(Debug)]
pub struct Gxa {
    base: Nmea0183Base,
}

impl Gxa {
    pub fn new(base: Nmea0183Base) -> Gxa {
        Gxa { base }
    }
    pub fn timestamp(&self) -> NaiveTimeError {
        self.base.naive_time(0)
    }
}
