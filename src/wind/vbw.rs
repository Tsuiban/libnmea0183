use crate::base::*;

#[derive(Debug)]
pub struct Vbw {
    base: Nmea0183Base,
}

impl Vbw {
    pub fn new(base: Nmea0183Base) -> Vbw {
        Vbw { base }
    }

    pub fn water_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[2] == "A" {
            Ok(Speed::from_knots(self.base.parameter(0)?))
        } else {
            Err(NmeaError("Invalid speed".to_string()))
        }
    }

    pub fn transverse_water_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[2] == "A" {
            Ok(Speed::from_knots(self.base.parameter(1)?))
        } else {
            Err(NmeaError("Invalid speed.".to_string()))
        }
    }

    pub fn ground_speed(&self) -> F32Error {
        if self.base.parameters[5] == "A" {
            self.base.parameter(3)
        } else {
            Ok (0.)
        }
    }

    pub fn transverse_ground_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[5] == "A" {
            Ok(Speed::from_knots(self.base.parameter(4)?))
        } else {
            Err(NmeaError("No speed present".to_string()))
        }
    }
}
