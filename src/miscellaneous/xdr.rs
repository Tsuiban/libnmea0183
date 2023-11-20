use std::str::FromStr;

use crate::base::*;

pub struct Xdr {
    base : Nmea0183Base,
}

pub struct Transducer<T> {
    pub xdr_type: char,
    pub xdr_data: T,
    pub xdr_units: char,
    pub xdr_name: String,
}

impl Xdr {
    pub fn new(base : Nmea0183Base) -> Xdr {
        Xdr { base }
    }
    
    pub fn measurements<T : FromStr> (&self, index: usize) -> Result<Transducer<T>, NmeaError> {
        if index >= self.base.parameters.len() / 4 {
            Err(NmeaError("Index out of range".to_string()))
        } else {
            Ok(Transducer::<T> {
                xdr_type : self.base.parameters[index * 4].chars().nth(0).unwrap(),
                xdr_data : self.base.parameter::<T>(index * 4 + 1)?,
                xdr_units : self.base.parameters[index * 4 + 2].chars().nth(0).unwrap(),
                xdr_name : self.base.parameters[index * 4 + 3].clone(),
            })
        }
    }
}
