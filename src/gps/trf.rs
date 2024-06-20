use crate::base::*;

#[derive(Debug)]
pub struct Trf {
    base: Nmea0183Base,
}

impl Trf {
    pub fn new(base: Nmea0183Base) -> Trf {
        Trf { base }
    }
    pub fn timestamp(&self) -> DateTimeError {
        self.base.from_time(0)
    }
}
