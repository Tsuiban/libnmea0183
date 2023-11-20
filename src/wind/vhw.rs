use crate::base::*;

pub struct Vhw {
    base : Nmea0183Base,
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

    pub fn water_speed_knots(&self) -> F32Error {
        if self.base.parameters[5] == "N" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "N" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed_mph(&self) -> F32Error {
        if self.base.parameters[5] == "M" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "M" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed_kph(&self) -> F32Error {
        if self.base.parameters[5] == "K" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "K" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }
}
