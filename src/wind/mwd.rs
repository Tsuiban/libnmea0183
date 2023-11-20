use crate::base::*;

pub struct Mwd {
    base : Nmea0183Base,
}


impl Mwd {
    pub fn new(base : Nmea0183Base) -> Mwd {
        Mwd { base }
    }
    
    pub fn direction_true(&self) -> F32Error {
        if self.base.parameters[1] == "T" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "T" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }

    pub fn direction_magnetic(&self) -> F32Error {
        if self.base.parameters[1] == "M" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "M" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }

    pub fn speed_knots(&self) -> F32Error {
        if self.base.parameters[5] == "N" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "N" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
               * match self.base.parameters[5].as_str() {
                "K" => 0.539957,
                   "M" => 0.868976,
                   _ => 0.0,
               })
        }
    }

    pub fn speed_mph(&self) -> F32Error {
        if self.base.parameters[5] == "M" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "M" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
               * match self.base.parameters[5].as_str() {
                "K" => 0.621371,
                   "N" => 1.15078,
                   _ => 0.0,
               })
        }
    }

    pub fn speed_kph(&self) -> F32Error {
        if self.base.parameters[5] == "K" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "K" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
               * match self.base.parameters[5].as_str() {
                "M" => 1.852,
                   "N" => 1.60934,
                   _ => 0.0,
               })
        }
    }
}
