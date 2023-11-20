use crate::base::*;

pub struct Rsa {
    base : Nmea0183Base,
}

impl Rsa {
    pub fn new(base : Nmea0183Base) -> Rsa {
        Rsa { base }
    }

    pub fn angle(&self) -> F32Error {
        if self.base.parameters[1] == "A" {
            self.base.parameter(0)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
    
    pub fn starboard_rudder(&self) -> F32Error {
        self.angle()
    }
    
    pub fn port_rudder(&self) -> F32Error {
        if self.base.parameters[3] == "A" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
}
