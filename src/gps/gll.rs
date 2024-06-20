use crate::base::*;

#[derive(Debug)]
pub struct Gll {
    base: Nmea0183Base,
}

impl Gll {
    pub fn new(base: Nmea0183Base) -> Gll {
        Gll { base }
    }

    pub fn position(&self) -> PositionError {
        self.base.position(0)
    }

    pub fn timestamp(&self) -> DateTimeError {
        self.base.by_time(4)
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[5] == "A"
    }

    pub fn faa_mode(&self) -> Option<char> {
        self.base.parameters[6].chars().nth(0)
    }
}
