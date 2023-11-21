use crate::base::*;

#[derive(Debug)]
pub struct Vhw {
    base: Nmea0183Base,
}

impl Vhw {
    pub fn new(base: Nmea0183Base) -> Vhw {
        Vhw { base }
    }

    pub fn heading_true(&self) -> F32Error {
        if self.base.parameters[1] == "T" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "T" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn heading_magnetic(&self) -> F32Error {
        if self.base.parameters[1] == "M" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "M" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[5] == "K" {
            Ok(Speed::from_kph(self.base.parameter(4)?))
        } else if self.base.parameters[7] == "K" {
            Ok(Speed::from_kph(self.base.parameter(6)?))
        } else {
            match self.base.parameters[5].as_str() {
                "M" => Ok(Speed::from_mph(self.base.parameter(4)?)),
                "N" => Ok(Speed::from_knots(self.base.parameter(4)?)),
                _ => match self.base.parameters[7].as_str() {
                    "M" => Ok(Speed::from_mph(self.base.parameter(6)?)),
                    "N" => Ok(Speed::from_knots(self.base.parameter(6)?)),
                    _ => Err(NmeaError("Not found".to_string())),
                },
            }
        }
    }
}
