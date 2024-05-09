use chrono::prelude::*;
use liblatlon::Position;
use std::{io, str::FromStr};

pub type F32Error = Result<f32, NmeaError>;
pub type F64Error = Result<f64, NmeaError>;
pub type U8Error = Result<u8, NmeaError>;
pub type UsizeError = Result<usize, NmeaError>;

pub type NaiveDateError = Result<NaiveDate, NmeaError>;
pub type NaiveTimeError = Result<NaiveTime, NmeaError>;
pub type NaiveDateTimeError = Result<NaiveDateTime, NmeaError>;
pub type PositionError = Result<Position, NmeaError>;

pub const INSUFFICIENT_NUMBER_OF_PARAMETERS: &str =
    "Insufficient number of parameters in sentence.";

pub const KPH_TO_KNOTS: f32 = 0.539957;
pub const MPH_TO_KNOTS: f32 = 0.868976;
pub const KPH_TO_MPH: f32 = 1.60934;

#[derive(Debug)]
pub struct NmeaError(pub String);

impl std::fmt::Display for NmeaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for NmeaError {}

#[derive(Debug, Clone)]
pub struct Nmea0183Base {
    pub sender: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub checksum: u8,
}

impl Nmea0183Base {
    pub fn new() -> Nmea0183Base {
        Nmea0183Base {
            sender: String::new(),
            message: String::new(),
            parameters: Vec::new(),
            checksum: 0,
        }
    }

    pub fn from_reader(reader: &mut Box<dyn io::BufRead>) -> Result<Nmea0183Base, io::Error> {
        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer) {
                Ok(n) => {
                    if n > 0 {
                        return Nmea0183Base::from_string(&buffer);
                    } else {
                        return Err(io::Error::new(std::io::ErrorKind::UnexpectedEof, "EOF"));
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub fn from_string(line: &String) -> Result<Nmea0183Base, io::Error> {
        let mut parts = line
            .trim()
            .split('*')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let checksum = parts.pop().unwrap();
        let mut parts = parts[0]
            .trim()
            .split(',')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        if parts.len() == 0 {
            return Err(io::Error::new(
                std::io::ErrorKind::InvalidData,
                line.to_string(),
            ));
        };
        let introducer = parts.remove(0);
        let talker = &introducer[..3];
        let message = &introducer[3..];
        let nmea = Nmea0183Base {
            sender: String::from(talker),
            message: String::from(message),
            parameters: parts,
            checksum: u8::from_str_radix(checksum.as_str(), 16).unwrap(),
        };
        Ok(nmea)
    }

    pub fn is_valid(&self) -> bool {
        self.calculate_checksum() == self.checksum
    }

    pub fn parameter<T: FromStr>(&self, n: usize) -> Result<T, NmeaError> {
        if self.parameters.len() > n {
            match self.parameters[n].parse::<T>() {
                Ok(n) => Ok(n),
                Err(_) => Err(NmeaError(format!("Could not parse {}", self.parameters[n]))),
            }
        } else {
            Err(NmeaError(INSUFFICIENT_NUMBER_OF_PARAMETERS.to_string()))
        }
    }

    pub fn naive_time(&self, n: usize) -> NaiveTimeError {
        match NaiveTime::parse_from_str(self.parameters[n].as_str(), "%H%M%S%.f") {
            Err(e) => return Err(NmeaError(e.to_string())),
            Ok(t) => Ok(t),
        }
    }

    pub fn naive_date(&self, n: usize) -> NaiveDateError {
        match NaiveDate::parse_from_str(self.parameters[n].as_str(), "%d%m%y") {
            Err(e) => return Err(NmeaError(e.to_string())),
            Ok(t) => Ok(t),
        }
    }

    pub fn position(&self, n: usize) -> PositionError {
        let latitude = if self.parameters[n + 1] == "N" {
            self.parameter::<f64>(n)?
        } else {
            -self.parameter::<f64>(n)?
        };
        let longitude = if self.parameters[n + 3] == "E" {
            self.parameter::<f64>(n + 2)?
        } else {
            -self.parameter::<f64>(n + 2)?
        };
        let latitude_degrees = if latitude >= 0.0 {
            (latitude / 100.0).floor()
        } else {
            (latitude / 100.0).ceil()
        };
        let longitude_degrees = if longitude >= 0.0 {
            (longitude / 100.0).floor()
        } else {
            (longitude / 100.0).ceil()
        };
        let latitude_minutes = (latitude - latitude_degrees * 100.0).abs();
        let longitude_minutes = (longitude - longitude_degrees * 100.0).abs();
        Ok(Position::from_degrees_decimal_minutes(
            latitude_degrees as i8,
            latitude_minutes,
            longitude_degrees as i8,
            longitude_minutes,
        ))
    }

    pub fn calculate_checksum(&self) -> u8 {
        let mut checksum = 0;
        for c in self.sender.as_bytes() {
            checksum ^= c;
        }
        for c in self.message.as_bytes() {
            checksum ^= c;
        }
        for p in &self.parameters {
            for c in p.as_bytes() {
                checksum ^= c;
            }
        }
        checksum
    }

    pub fn to_string(&self) -> Option<String> {
        if self.is_valid() {
            let retval = format!(
                "{}{}{}*{:02X}",
                self.sender,
                self.message,
                self.parameters.iter().fold(String::new(), |acc, x| {
                    let mut temp = acc.clone();
                    temp.push_str(format!(",{}", x).as_str());
                    temp
                }),
                self.checksum
            );
            Some(retval)
        } else if self.checksum == 0 {
            let mut retval = (*self).clone();
            retval.checksum = self.calculate_checksum();
            retval.to_string()
        } else {
            None
        }
    }
}

pub struct Temperature {
    celsius: f32,
}

impl Temperature {
    pub fn from_celsius(celsius: f32) -> Temperature {
        Temperature { celsius }
    }

    pub fn from_fahrenheit(fahrenheit: f32) -> Temperature {
        Temperature {
            celsius: (fahrenheit - 32.0) * 5.0 / 9.0,
        }
    }
    pub fn as_celsius(&self) -> f32 {
        self.celsius
    }

    pub fn as_fahrenheit(&self) -> f32 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
}

#[derive(Debug)]
pub struct Distance {
    meters: f32,
}

impl Distance {
    const METERS_2_MILES: f32 = 0.000621371192;
    const METERS_2_NM: f32 = 0.000539956803;

    pub fn from_meters(meters: f32) -> Distance {
        Distance { meters }
    }

    pub fn from_kilometers(km: f32) -> Distance {
        Distance {
            meters: km * 1000.0,
        }
    }

    pub fn from_miles(miles: f32) -> Distance {
        Distance {
            meters: miles / Distance::METERS_2_MILES,
        }
    }

    pub fn from_nautical_miles(nm: f32) -> Distance {
        Distance {
            meters: nm / Distance::METERS_2_NM,
        }
    }

    pub fn as_meters(&self) -> f32 {
        self.meters
    }
    pub fn as_kilometers(&self) -> f32 {
        self.meters / 1000.0
    }
    pub fn as_centimeters(&self) -> f32 {
        self.meters * 100.0
    }
    pub fn as_miles(&self) -> f32 {
        self.meters * Distance::METERS_2_MILES
    }
    pub fn as_feet(&self) -> f32 {
        self.as_miles() / 5280.0
    }
    pub fn as_nautical_mile(&self) -> f32 {
        self.meters * Distance::METERS_2_NM
    }
}

#[derive(Debug)]
pub struct Speed {
    meters_per_second: f32,
}

impl Speed {
    pub fn from_mps(meters_per_second: f32) -> Speed {
        Speed { meters_per_second }
    }
    pub fn from_kph(kph: f32) -> Speed {
        Speed {
            meters_per_second: kph * 1000.0 / 3600.0,
        }
    }
    pub fn from_mph(mph: f32) -> Speed {
        Speed {
            meters_per_second: mph / Distance::METERS_2_MILES / 3600.0,
        }
    }
    pub fn from_knots(kts: f32) -> Speed {
        Speed {
            meters_per_second: kts / Distance::METERS_2_NM / 3600.0,
        }
    }

    pub fn as_mps(&self) -> f32 {
        self.meters_per_second
    }
    pub fn as_kph(&self) -> f32 {
        self.meters_per_second * 3.6
    }
    pub fn as_mph(&self) -> f32 {
        self.meters_per_second * 3600.0 * 0.000621371192
    }
    pub fn as_knots(&self) -> f32 {
        self.meters_per_second * 3600.0 * 0.000539956803
    }
}

pub struct Pressure {
    bar: f32,
}

impl Pressure {
    const INCHES_MERCURY_2_BAR: f32 = 0.03386;
    const PSI_2_BAR: f32 = 0.0689475729;
    const KPA_2_BAR: f32 = 0.01;

    pub fn from_bar(bar: f32) -> Pressure {
        Pressure { bar }
    }

    pub fn from_inches_mercury(inches: f32) -> Pressure {
        Pressure {
            bar: inches * Pressure::INCHES_MERCURY_2_BAR,
        }
    }

    pub fn from_psi(psi: f32) -> Pressure {
        Pressure {
            bar: psi * Pressure::PSI_2_BAR,
        }
    }

    pub fn from_kilo_pascals(kpa: f32) -> Pressure {
        Pressure {
            bar: kpa * Pressure::KPA_2_BAR,
        }
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
