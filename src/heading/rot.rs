use crate::base::*;

#[derive(Debug)]
pub struct Rot {
    base: Nmea0183Base,
}

impl Rot {
    pub fn new(base: Nmea0183Base) -> Rot {
        Rot { base }
    }

    pub fn rate_of_turn(&self) -> F32Error {
        self.base.parameter(0)
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[1] == "A"
    }
}
