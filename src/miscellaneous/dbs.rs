use crate::base::*;

#[derive(Debug)]
pub struct Dbs {
    base: Nmea0183Base,
}

impl Dbs {
    pub fn new(base: Nmea0183Base) -> Dbs {
        Dbs { base }
    }

    pub fn depth(&self) -> Result<Distance, NmeaError> {
        if self.base.parameters[1] == "M" {
            Ok(Distance::from_meters(self.base.parameter(0)?))
        } else if self.base.parameters[3] == "M" {
            Ok(Distance::from_meters(self.base.parameter(2)?))
        } else if self.base.parameters[5] == "M" {
            Ok(Distance::from_meters(self.base.parameter(4)?))
        } else if self.base.parameters[1] == "f" {
            Ok(Distance::from_miles(
                self.base.parameter::<f32>(0)? / 5280.0,
            ))
        } else if self.base.parameters[3] == "f" {
            Ok(Distance::from_miles(
                self.base.parameter::<f32>(2)? / 5280.0,
            ))
        } else if self.base.parameters[5] == "f" {
            Ok(Distance::from_miles(
                self.base.parameter::<f32>(4)? / 5280.0,
            ))
        } else {
            Err(NmeaError("Invalid data.".to_string()))
        }
    }

    pub fn offset(&self) -> Result<Distance, NmeaError> {
        Ok(Distance::from_meters(self.base.parameter(1)?))
    }

    pub fn maximum_range(&self) -> Result<Distance, NmeaError> {
        Ok(Distance::from_meters(self.base.parameter(2)?))
    }
}
