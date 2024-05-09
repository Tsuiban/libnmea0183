use crate::base::*;

#[derive(Debug)]
pub struct Vdm {
    base: Nmea0183Base,
}

impl Vdm {
    pub fn new(base: Nmea0183Base) -> Vdm {
        Vdm { base }
    }

    pub fn total_sentences(&self) -> UsizeError {
        self.base.parameter(0)
    }

    pub fn sentence_number(&self) -> UsizeError {
        self.base.parameter(1)
    }

    pub fn sentence_id(&self) -> UsizeError {
        self.base.parameter(2)
    }

    pub fn channel(&self) -> Result<char, NmeaError> {
        if let Some(c) = self.base.parameters[3].chars().nth(0) {
            Ok(c)
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }

    pub fn message(&self) -> Result<String, NmeaError> {
        Ok(self.base.parameters[4].clone())
    }
}
