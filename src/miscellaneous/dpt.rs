use crate::base::*;

#[derive(Debug)]
pub struct Dpt {
    base: Nmea0183Base,
}

impl Dpt {
    pub fn new(base: Nmea0183Base) -> Dpt {
        Dpt { base }
    }

    pub fn depth(&self) -> Result<Distance, NmeaError> {
        Ok(Distance::from_meters(self.base.parameter(0)?))
    }

    pub fn offset(&self) -> Result<Distance, NmeaError> {
        Ok(Distance::from_meters(self.base.parameter(1)?))
    }

    pub fn maximum_range(&self) -> Result<Distance, NmeaError> {
        Ok(Distance::from_meters(self.base.parameter(2)?))
    }
}
