use crate::base::*;

#[derive(Debug)]
pub struct Hdg {
    base: Nmea0183Base,
}

impl Hdg {
    pub fn new(base: Nmea0183Base) -> Hdg {
        Hdg { base }
    }

    pub fn magnetic_heading(&self) -> F32Error {
        self.base.parameter(0)
    }

    pub fn magnetic_deviation(&self) -> F32Error {
        if self.base.parameters[2] == "E" {
            self.base.parameter(1)
        } else {
            Ok(-(self.base.parameter(1)?))
        }
    }

    pub fn magnetic_variation(&self) -> F32Error {
        if self.base.parameters[4] == "E" {
            self.base.parameter(3)
        } else {
            Ok(-(self.base.parameter(3)?))
        }
    }
}
