use crate::base::*;

#[derive(Debug)]
pub struct Grs {
    base: Nmea0183Base,
}

impl Grs {
    pub fn new(base: Nmea0183Base) -> Grs {
        Grs { base }
    }
    pub fn timestamp(&self) -> DateTimeError {
        self.base.from_time(0)
    }
}
