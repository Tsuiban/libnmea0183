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
    
    pub fn sog_knots(&self) -> F32Error {
        if self.base.parameters[5] == "N" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "N" {
            self.base.parameter(6)
        } else if self.base.parameters[5] == "K" {
            Ok(self.base.parameter::<f32>(4)? * KPH_TO_KNOTS)
        } else if self.base.parameters[7] == "K" {
            Ok(self.base.parameter::<f32>(6)? * KPH_TO_KNOTS)
        } else if self.base.parameters[5] == "M" {
            Ok(self.base.parameter::<f32>(4)? * MPH_TO_KNOTS)
        } else if self.base.parameters[7] == "M" {
            Ok(self.base.parameter::<f32>(6)? * MPH_TO_KNOTS)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
    
    pub fn sog_kph(&self) -> F32Error {
        if self.base.parameters[5] == "K" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "K" {
            self.base.parameter(6)
        } else if self.base.parameters[5] == "N" {
            Ok(self.base.parameter::<f32>(4)? / KPH_TO_KNOTS)
        } else if self.base.parameters[7] == "N" {
            Ok(self.base.parameter::<f32>(6)? / KPH_TO_KNOTS)
        } else if self.base.parameters[5] == "M" {
            Ok(self.base.parameter::<f32>(4)? / MPH_TO_KNOTS)
        } else if self.base.parameters[7] == "M" {
            Ok(self.base.parameter::<f32>(6)? / MPH_TO_KNOTS)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }

    pub fn sog_mph(&self) -> F32Error {
        if self.base.parameters[5] == "M" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "M" {
            self.base.parameter(6)
        } else if self.base.parameters[5] == "K" {
            Ok(self.base.parameter::<f32>(4)? * KPH_TO_MPH)
        } else if self.base.parameters[7] == "K" {
            Ok(self.base.parameter::<f32>(6)? * KPH_TO_MPH)
        } else if self.base.parameters[5] == "N" {
            Ok(self.base.parameter::<f32>(4)? / MPH_TO_KNOTS)
        } else if self.base.parameters[7] == "N" {
            Ok(self.base.parameter::<f32>(6)? / MPH_TO_KNOTS)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
    
    pub fn faa_mode(&self) -> Option<char> {
        if self.base.parameters.len() > 8 && self.base.parameters[8].len() > 0 {
            Some(self.base.parameters[8].chars().nth(0).unwrap())
        } else { None }
    }
}
