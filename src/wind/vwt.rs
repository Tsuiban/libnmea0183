use crate::base::*;

#[derive(Debug)]
pub struct Vwt {
    base: Nmea0183Base,
}

impl Vwt {
    pub fn new(base: Nmea0183Base) -> Vwt {
        Vwt { base }
    }

    pub fn wind_direction(&self) -> F32Error {
        match self.base.parameters[1].as_str() {
            "L" => Ok(-(self.base.parameter::<f32>(0)).unwrap()),
            "R" => self.base.parameter(0),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }

    pub fn wind_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[3] == "M" {
            Ok(Speed::from_mps(self.base.parameter::<f32>(2)?))
        } else if self.base.parameters[5] == "M" {
            Ok(Speed::from_mps(self.base.parameter::<f32>(4)?))
        } else if self.base.parameters[7] == "M" {
            Ok(Speed::from_mps(self.base.parameter::<f32>(6)?))
        } else if self.base.parameters[3] == "K" {
            Ok(Speed::from_kph(self.base.parameter(2)?))
        } else if self.base.parameters[5] == "K" {
            Ok(Speed::from_kph(self.base.parameter::<f32>(4)?))
        } else if self.base.parameters[7] == "K" {
            Ok(Speed::from_kph(self.base.parameter::<f32>(6)?))
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
}
