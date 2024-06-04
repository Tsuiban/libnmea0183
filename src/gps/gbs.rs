use crate::base::*;

#[derive(Debug)]
pub struct Gbs {
    base: Nmea0183Base,
}

impl Gbs {
    pub fn new(base: Nmea0183Base) -> Gbs {
        Gbs { base }
    }
    pub fn timestamp(&self) -> NaiveTimeError {
        self.base.naive_time(0)
    }
}
