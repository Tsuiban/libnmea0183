use crate::base::*;

pub struct Ais {
    base : Nmea0183Base,
}

impl Ais {
    pub fn new(base : Nmea0183Base) -> Ais {
        Ais { base }
    }
    
    pub fn sentence_total(&self) -> UsizeError {
        self.base.parameter(0)
    }
    
    pub fn sentence_number(&self) -> UsizeError {
        self.base.parameter(1)
    }
    
    pub fn message_identifier(&self) -> UsizeError {
        self.base.parameter(2)
    }
    
    pub fn channel(&self) -> Result<char, NmeaError> {
        Ok(self.base.parameters[4].chars().nth(0).unwrap())
    }
}