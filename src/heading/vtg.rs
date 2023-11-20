use crate::base::*;

pub struct Vtg {
    base : Nmea0183Base,
}

impl Vtg {
    pub fn new(base: Nmea0183Base) -> Vtg {
        Vtg { base }
    }

    pub fn cog_true(&self) -> F32Error {
        if self.base.parameters[1] == "T" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "T" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }
    
    pub fn cog_mag(&self) -> F32Error {
        if self.base.parameters[1] == "M" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "M" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }
    
    pub fn sog(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[5] == "N" {
            Ok(Speed::from_knots(self.base.parameter(4)?))
        } else if self.base.parameters[7] == "N" {
            Ok(Speed::from_knots(self.base.parameter(6)?))
        } else {
            match self.base.parameters[5].as_str() {
                "M" => Ok(Speed::from_mph(self.base.parameter(4)?)),
                "K" => Ok(Speed::from_kph(self.base.parameter(4)?)),
                _ => match self.base.parameters[7].as_str() {
                    "M" => Ok(Speed::from_mph(self.base.parameter(6)?)),
                    "K" => Ok(Speed::from_kph(self.base.parameter(6)?)),
                    _ => Err(NmeaError("Not found".to_string())),
                }
            }
        }
    }
    
    pub fn faa_mode(&self) -> Option<char> {
        if self.base.parameters.len() > 8 && self.base.parameters[8].len() > 0 {
            Some(self.base.parameters[8].chars().nth(0).unwrap())
        } else { None }
    }
}
