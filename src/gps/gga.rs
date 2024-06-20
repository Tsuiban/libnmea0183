use crate::base::*;

#[derive(Debug)]
pub struct Gga {
    base: Nmea0183Base,
}

impl Gga {
    pub fn new(base: Nmea0183Base) -> Gga {
        Gga { base }
    }

    pub fn timestamp(&self) -> DateTimeError {
        self.base.by_time(0)
    }

    pub fn position(&self) -> PositionError {
        self.base.position(1)
    }

    pub fn quality(&self) -> U8Error {
        self.base.parameter(5)
    }

    pub fn number_of_satellites(&self) -> U8Error {
        self.base.parameter(6)
    }

    pub fn hdop(&self) -> F32Error {
        self.base.parameter(7)
    }

    pub fn height(&self) -> F64Error {
        self.base.parameter(8)
    }

    pub fn geoid_separation(&self) -> F64Error {
        self.base.parameter(10)
    }

    pub fn differential_age(&self) -> F64Error {
        self.base.parameter(12)
    }

    pub fn reference_station(&self) -> UsizeError {
        self.base.parameter(13)
    }
}
