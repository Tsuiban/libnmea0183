use crate::base::*;

#[derive(Debug)]
pub struct Hdm {
    base: Nmea0183Base,
}

impl Hdm {
    pub fn new(base: Nmea0183Base) -> Hdm {
        Hdm { base }
    }

    pub fn heading(&self) -> F32Error {
        self.base.parameter(0)
    }
}
