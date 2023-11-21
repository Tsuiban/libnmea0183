use crate::base::*;

#[derive(Debug)]
pub struct Mwv {
    base: Nmea0183Base,
}

impl Mwv {
    pub fn new(base: Nmea0183Base) -> Mwv {
        Mwv { base }
    }

    pub fn angle_relative(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[1] == "R" {
                self.base.parameter(0)
            } else {
                Err(NmeaError("Not found.".to_string()))
            }
        } else {
            Err(NmeaError("Data not valid.".to_string()))
        }
    }

    pub fn angle_true(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[1] == "T" {
                self.base.parameter(0)
            } else {
                Err(NmeaError("Not found.".to_string()))
            }
        } else {
            Err(NmeaError("Data not valid.".to_string()))
        }
    }

    pub fn wind_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[4] == "A" {
            match self.base.parameters[3].as_str() {
                "M" => Ok(Speed::from_mph(self.base.parameter(2)?)),
                "N" => Ok(Speed::from_knots(self.base.parameter(2)?)),
                "K" => Ok(Speed::from_kph(self.base.parameter(2)?)),
                _ => Err(NmeaError("Not found".to_string())),
            }
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
}
