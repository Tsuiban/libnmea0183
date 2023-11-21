use crate::base::*;

#[derive(Debug)]
pub struct SatelliteData {
    pub id: usize,
    pub elevation: usize,
    pub azimuth: usize,
    pub snr: usize,
}

#[derive(Debug)]
pub struct Gsv {
    base: Nmea0183Base,
}

impl Gsv {
    pub fn new(base: Nmea0183Base) -> Gsv {
        Gsv { base }
    }

    pub fn sentence_total(&self) -> UsizeError {
        self.base.parameter(0)
    }

    pub fn sentence_number(&self) -> UsizeError {
        self.base.parameter(1)
    }

    pub fn number_of_satellites(&self) -> UsizeError {
        self.base.parameter(2)
    }

    pub fn satellite(&self, n: usize) -> Result<SatelliteData, NmeaError> {
        if n * 4 + 6 < self.base.parameters.len() {
            Ok(SatelliteData {
                id: self.base.parameter(n * 4 + 3)?,
                elevation: self.base.parameter(n * 4 + 1 + 3)?,
                azimuth: self.base.parameter(n * 4 + 2 + 3)?,
                snr: self.base.parameter(n * 4 + 3 + 3)?,
            })
        } else {
            Err(NmeaError("Invalid index".to_string()))
        }
    }
}
