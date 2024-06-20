use crate::base::*;

#[derive(Debug)]
pub struct Bwr {
    base: Nmea0183Base,
}

impl Bwr {
    pub fn new(base: Nmea0183Base) -> Bwr {
        Bwr { base }
    }
    pub fn timestamp(&self) -> DateTimeError {
        self.base.by_time(0)
    }
}
