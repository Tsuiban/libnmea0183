use crate::base::*;

#[derive(Debug)]
pub struct Hdt {
    base: Nmea0183Base,
}

impl Hdt {
    pub fn new(base: Nmea0183Base) -> Hdt {
        Hdt { base }
    }

    pub fn heading(&self) -> F32Error {
        self.base.parameter(0)
    }
}
