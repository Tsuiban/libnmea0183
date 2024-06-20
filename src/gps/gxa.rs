use crate::base::*;

#[derive(Debug)]
pub struct Gxa {
    base: Nmea0183Base,
}

impl Gxa {
    pub fn new(base: Nmea0183Base) -> Gxa {
        Gxa { base }
    }
    pub fn timestamp(&self) -> DateTimeError {
        self.base.by_time(0)
    }
}
