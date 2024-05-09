use crate::base::*;

#[derive(Debug)]
pub struct Vlw {
    base: Nmea0183Base,
}

impl Vlw {
    pub fn new(base: Nmea0183Base) -> Vlw {
        Vlw { base }
    }

    pub fn cumulative_dtw(&self) -> Result<Distance, NmeaError> {
        match self.base.parameters[1].as_str() {
            "M" => Ok(Distance::from_meters(self.base.parameter(0)?)),
            "K" => Ok(Distance::from_kilometers(self.base.parameter(0)?)),
            "N" => Ok(Distance::from_nautical_miles(self.base.parameter(0)?)),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }

    pub fn dtw_since_reset(&self) -> Result<Distance, NmeaError> {
        match self.base.parameters[3].as_str() {
            "M" => Ok(Distance::from_meters(self.base.parameter(2)?)),
            "K" => Ok(Distance::from_kilometers(self.base.parameter(2)?)),
            "N" => Ok(Distance::from_nautical_miles(self.base.parameter(2)?)),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }

    pub fn dog_cumulative(&self) -> Result<Distance, NmeaError> {
        match self.base.parameters[5].as_str() {
            "M" => Ok(Distance::from_meters(self.base.parameter(4)?)),
            "K" => Ok(Distance::from_kilometers(self.base.parameter(4)?)),
            "N" => Ok(Distance::from_nautical_miles(self.base.parameter(4)?)),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }

    pub fn dog_since_reset(&self) -> Result<Distance, NmeaError> {
        match self.base.parameters[7].as_str() {
            "M" => Ok(Distance::from_meters(self.base.parameter(6)?)),
            "K" => Ok(Distance::from_kilometers(self.base.parameter(6)?)),
            "N" => Ok(Distance::from_nautical_miles(self.base.parameter(6)?)),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }
}
