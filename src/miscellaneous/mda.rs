use crate::base::*;

pub struct Mda {
    base : Nmea0183Base,
}

pub struct Pressure {
    bar : f32,
}

impl Pressure {
    const INCHES_MERCURY_2_BAR : f32 = 0.03386;
    const PSI_2_BAR : f32 = 0.0689475729;
    const KPA_2_BAR : f32 = 0.01;
    
    pub fn from_bar(bar : f32) -> Pressure {
        Pressure { bar }
    }
    
    pub fn from_inches_mercury(inches : f32) -> Pressure {
        Pressure { bar : inches * Pressure::INCHES_MERCURY_2_BAR }
    }
    
    pub fn from_psi(psi : f32) -> Pressure {
        Pressure { bar : psi * Pressure::PSI_2_BAR }
    }
    
    pub fn from_kilo_pascals(kpa : f32) -> Pressure {
        Pressure { bar : kpa * Pressure::KPA_2_BAR }
    }
    
    pub fn as_bar(&self) -> f32 {
        self.bar
    }
    
    pub fn as_inches_mercury(&self) -> f32 {
        self.bar / Pressure::INCHES_MERCURY_2_BAR
    }
    
    pub fn as_psi(&self) -> f32 {
        self.bar / Pressure::PSI_2_BAR
    }
    
    pub fn as_kpa(&self) -> f32 {
        self.bar / Pressure::KPA_2_BAR
    }
}

impl Mda {
    pub fn new(base : Nmea0183Base) -> Mda {
        Mda { base }
    }

    pub fn pressure(&self) -> Result<Pressure, NmeaError> {
        if self.base.parameters[1] == "B" {
            Ok(Pressure::from_bar(self.base.parameter(0)?))
        } else if self.base.parameters[3] == "B" {
            Ok(Pressure::from_bar(self.base.parameter(2)?))
        } else if self.base.parameters[1] == "I" {
            Ok(Pressure::from_inches_mercury(self.base.parameter(0)?))
        } else if self.base.parameters[3] == "I" {
            Ok(Pressure::from_inches_mercury(self.base.parameter(3)?))
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
    
    pub fn air_temperature(&self) -> Result<Temperature, NmeaError> {
        if self.base.parameters[5] == "C" {
            Ok(Temperature::from_celsius(self.base.parameter(4)?))
        } else if self.base.parameters[5] == "F" {
            Ok(Temperature::from_fahrenheit(self.base.parameter(4)?))
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
    
    pub fn water_temperature(&self) -> Result<Temperature, NmeaError> {
        if self.base.parameters[7] == "C" {
            Ok(Temperature::from_celsius(self.base.parameter(6)?))
        } else if self.base.parameters[7] == "F" {
            Ok(Temperature::from_fahrenheit(self.base.parameter(6)?))
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
    
    pub fn relative_humidity(&self) -> F32Error {
        self.base.parameter(8)
    }
    
    pub fn absolute_humidity(&self) -> F32Error {
        self.base.parameter(9)
    }
    
    pub fn dew_point(&self) -> Result<Temperature, NmeaError> {
        if self.base.parameters[11] == "C" {
            Ok(Temperature::from_celsius(self.base.parameter(10)?))
        } else if self.base.parameters[11] == "F" {
            Ok(Temperature::from_fahrenheit(self.base.parameter(10)?))
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
    
    pub fn wind_direction_true(&self) -> F32Error {
        if self.base.parameters[13] == "T" {
            self.base.parameter(12)
        } else if self.base.parameters[15] == "T" {
            self.base.parameter(14)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
    
    pub fn wind_direction_magnetic(&self) -> F32Error {
        if self.base.parameters[13] == "M" {
            self.base.parameter(12)
        } else if self.base.parameters[15] == "M" {
            self.base.parameter(14)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }
    
    pub fn wind_speed(&self) -> Result<Speed, NmeaError> {
        if self.base.parameters[17] == "M" {
            Ok(Speed::from_mps(self.base.parameter(16)?))
        } else if self.base.parameters[19] == "M" {
            Ok(Speed::from_mps(self.base.parameter(18)?))
        } else {
            match self.base.parameters[17].as_str() {
                "N" => Ok(Speed::from_knots(self.base.parameter(16)?)),
                "K" => Ok(Speed::from_kph(self.base.parameter(16)?)),
                _ => {
                    match self.base.parameters[19].as_str() {
                        "N" => Ok(Speed::from_knots(self.base.parameter(18)?)),
                        "K" => Ok(Speed::from_kph(self.base.parameter(18)?)),
                        _ => Err(NmeaError("Invalid data".to_string())),
                    }
                }
            }
        }
    }

}
