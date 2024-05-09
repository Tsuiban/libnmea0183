use crate::base::*;

#[derive(Debug)]
pub struct Gsa {
    base: Nmea0183Base,
}

impl Gsa {
    pub fn new(base: Nmea0183Base) -> Gsa {
        Gsa { base }
    }

    pub fn mode(&self) -> Result<char, NmeaError> {
        Ok(self.base.parameters[0].chars().nth(0).unwrap())
    }

    pub fn fix_type(&self) -> Result<char, NmeaError> {
        Ok(self.base.parameters[1].chars().nth(0).unwrap())
    }

    pub fn prn_number(&self) -> U8Error {
        self.base.parameter(2)
    }

    pub fn pdop(&self) -> F32Error {
        self.base.parameter(3)
    }

    pub fn hdop(&self) -> F32Error {
        self.base.parameter(4)
    }

    pub fn vdop(&self) -> F32Error {
        self.base.parameter(5)
    }
}
