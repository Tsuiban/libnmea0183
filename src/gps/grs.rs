use crate::base::*;

#[derive(Debug)]
pub struct Grs {
    base: Nmea0183Base,
}

impl Grs {
    pub fn new(base: Nmea0183Base) -> Grs {
        Grs { base }
    }
    pub fn timestamp(&self) -> NaiveTimeError {
        self.base.naive_time(0)
    }
}
