use crate::base::*;

#[derive(Debug)]
pub struct Vwr {
    base: Nmea0183Base,
}

impl Vwr {
    pub fn new(base: Nmea0183Base) -> Vwr {
        Vwr { base }
    }

    pub fn wind_direction(&self) -> F32Error {
        match self.base.parameters[1].as_str() {
            "L" => Ok(-(self.base.parameter::<f32>(0)).unwrap()),
            "R" => self.base.parameter(0),
            _ => Err(NmeaError("Invalid data".to_string())),
        }
    }
}
