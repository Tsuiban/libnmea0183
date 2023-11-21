use crate::base::*;

#[derive(Debug)]
pub struct Mtw {
    base: Nmea0183Base,
}

impl Mtw {
    pub fn new(base: Nmea0183Base) -> Mtw {
        Mtw { base }
    }

    pub fn temperature(&self) -> Result<Temperature, NmeaError> {
        match self.base.parameters[1].as_str() {
            "C" => Ok(Temperature::from_celsius(self.base.parameter::<f32>(0)?)),
            "F" => Ok(Temperature::from_fahrenheit(self.base.parameter::<f32>(0)?)),
            _ => Err(NmeaError("Not found".to_string())),
        }
    }
}
