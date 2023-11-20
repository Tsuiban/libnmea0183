use crate::base::*;

pub struct Mwv {
    base : Nmea0183Base,
}

impl Mwv {
    pub fn new(base : Nmea0183Base) -> Mwv {
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

    pub fn speed_kph(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[3] == "K" {
                self.base.parameter(2)
            } else {
                Err(NmeaError("Not found".to_string()))
            }
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }

    pub fn speed_mph(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[3] == "M" {
                self.base.parameter(2)
            } else {
                Err(NmeaError("Not found".to_string()))
            }
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
}
