use crate::base::*;

#[derive(Debug)]
pub struct Gst {
    base: Nmea0183Base,
}

impl Gst {
    pub fn new(base : Nmea0183Base) -> Gst { Gst { base } }
    pub fn timestamp(&self) -> NaiveTimeError { self.base.naive_time(0) }
}