use crate::base::*;

#[derive(Debug)]
pub struct Bwc {
    base: Nmea0183Base,
}

impl Bwc {
    pub fn new(base : Nmea0183Base) -> Bwc { Bwc { base } }
    pub fn timestamp(&self) -> NaiveTimeError { self.base.naive_time(0) }
}