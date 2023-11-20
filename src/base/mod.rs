use std::{io, str::FromStr};
use chrono::prelude::*;

pub type F32Error = Result<f32, NmeaError>;
pub type F64Error = Result<f64, NmeaError>;
pub type U8Error = Result<u8, NmeaError>;
pub type UsizeError = Result<usize, NmeaError>;

pub type NaiveDateError = Result<NaiveDate, NmeaError>;
pub type NaiveTimeError = Result<NaiveTime, NmeaError>;
pub type NaiveDateTimeError = Result<NaiveDateTime, NmeaError>;
pub type PositionError = Result<Position, NmeaError>;

#[derive(Debug)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

pub const INSUFFICIENT_NUMBER_OF_PARAMETERS: &str = "Insufficient number of parameters in sentence.";

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
            checksum: checksum.parse::<u8>().unwrap(),
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
        Ok(Position {
            latitude: if self.parameters[n + 1] == "N" {
                self.parameter(n)?
            } else {
                -self.parameter(n)?
            },

            longitude: if self.parameters[n + 3] == "E" {
                self.parameter(n + 2)?
            } else {
                -self.parameter(n + 2)?
            },
        })
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

