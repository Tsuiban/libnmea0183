use chrono::prelude::*;
use chrono::Duration;
use std::io;
use std::str::FromStr;

type F32Error = Result<f32, NmeaError>;
type F64Error = Result<f64, NmeaError>;
type U8Error = Result<u8, NmeaError>;
type UsizeError = Result<usize, NmeaError>;

type NaiveDateError = Result<NaiveDate, NmeaError>;
type NaiveTimeError = Result<NaiveTime, NmeaError>;
type NaiveDateTimeError = Result<NaiveDateTime, NmeaError>;
type PositionError = Result<Position, NmeaError>;

const INSUFFICIENT_NUMBER_OF_PARAMETERS: &str = "Insufficient number of parameters in sentence.";

#[derive(Debug)]
pub struct NmeaError(String);

#[derive(Debug)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

impl std::fmt::Display for NmeaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for NmeaError {}

pub enum Nmea0183 {
    GGA(Gga),
    GLL(Gll),
    RMC(Rmc),
    ZDA(Zda),
    ROT(Rot),
    HDG(Hdg),
    HDM(Hdm),
    HDT(Hdt),
    MWD(Mwd),
    MWV(Mwv),
    VHW(Vhw),
}

macro_rules! make_sentence {
    ($x : ident) => {
        #[derive(Debug, Clone)]
        pub struct $x {
            base: Nmea0183Base,
        }

        impl $x {
            pub fn new(base: Nmea0183Base) -> $x {
                $x { base }
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct Nmea0183Base {
    pub sender: String,
    pub message: String,
    pub parameters: Vec<String>,
    pub checksum: u8,
}

make_sentence! {Gga}
make_sentence! {Gll}
make_sentence! {Rmc}
make_sentence! {Zda}
make_sentence! {Rot}
make_sentence! {Hdg}
make_sentence! {Hdm}
make_sentence! {Hdt}
make_sentence! {Mwd}
make_sentence! {Mwv}
make_sentence! {Vhw}

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

impl Gga {
    pub fn timestamp(&self) -> NaiveTimeError {
        self.base.naive_time(0)
    }

    pub fn position(&self) -> PositionError {
        self.base.position(0)
    }

    pub fn quality(&self) -> U8Error {
        self.base.parameter(5)
    }

    pub fn number_of_satellites(&self) -> U8Error {
        self.base.parameter(6)
    }

    pub fn hdop(&self) -> F32Error {
        self.base.parameter(7)
    }

    pub fn height(&self) -> F64Error {
        self.base.parameter(8)
    }

    pub fn geoid_separation(&self) -> F64Error {
        self.base.parameter(10)
    }

    pub fn differential_age(&self) -> F64Error {
        self.base.parameter(12)
    }

    pub fn reference_station(&self) -> UsizeError {
        self.base.parameter(13)
    }
}

impl Gll {
    pub fn position(&self) -> PositionError {
        self.base.position(0)
    }

    pub fn timestamp(&self) -> NaiveTimeError {
        self.base.naive_time(4)
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[5] == "A"
    }

    pub fn faa_mode(&self) -> Option<char> {
        self.base.parameters[6].chars().nth(0)
    }
}

impl Rmc {
    pub fn timestamp(&self) -> Result<DateTime<Utc>, NmeaError> {
        let timeportion: NaiveTime = self.base.naive_time(0)?;
        let dateportion: NaiveDate = self.base.naive_date(8)?;
        let naivedatetime = NaiveDateTime::new(dateportion, timeportion);
        Ok(DateTime::from_naive_utc_and_offset(naivedatetime, Utc))
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[1] == "A"
    }

    pub fn position(&self) -> PositionError {
        self.base.position(2)
    }

    pub fn sog(&self) -> F32Error {
        self.base.parameter(6)
    }

    pub fn track_made_good(&self) -> F32Error {
        self.base.parameter(7)
    }

    pub fn magnetic_variation(&self) -> F32Error {
        if self.base.parameters[10] == "E" {
            self.base.parameter(9)
        } else {
            Ok(-(self.base.parameter(9)?))
        }
    }

    pub fn faa_mode(&self) -> Option<char> {
        self.base.parameters[11].chars().nth(0)
    }

    pub fn nav_status(&self) -> Option<char> {
        self.base.parameters[12].chars().nth(0)
    }
}

impl Zda {
    pub fn timestamp(&self) -> Result<DateTime<Utc>, NmeaError> {
        let timeportion = self.base.naive_time(0)?;
        let date_string = self.base.parameters[1].clone()
            + self.base.parameters[2].clone().as_str()
            + self.base.parameters[3].clone().as_str();
        match NaiveDate::parse_from_str(date_string.as_str(), "%d%m%Y") {
            Ok(dateportion) => {
                let datestamp = NaiveDateTime::new(dateportion, timeportion);
                Ok(DateTime::from_naive_utc_and_offset(datestamp, Utc))
            }
            Err(e) => Err(NmeaError(format!("{}", e))),
        }
    }

    pub fn local_time(&self) -> NaiveDateTimeError {
        let utc = self.timestamp()?;
        let hours = self.base.parameter::<i64>(4)?;
        let minutes = self.base.parameter::<i64>(5)? + hours * 60;
        let delta = Duration::minutes(minutes);
        let naive = NaiveDateTime::new(utc.date_naive(), utc.time()) + delta;
        Ok(naive)
    }
}

impl Rot {
    pub fn rate_of_turn(&self) -> F32Error {
        self.base.parameter(0)
    }

    pub fn is_valid(&self) -> bool {
        self.base.parameters[1] == "A"
    }
}

impl Hdg {
    pub fn magnetic_heading(&self) -> F32Error {
        self.base.parameter(0)
    }

    pub fn magnetic_deviation(&self) -> F32Error {
        if self.base.parameters[2] == "E" {
            self.base.parameter(1)
        } else {
            Ok(-(self.base.parameter(1)?))
        }
    }

    pub fn magnetic_variation(&self) -> F32Error {
        if self.base.parameters[4] == "E" {
            self.base.parameter(3)
        } else {
            Ok(-(self.base.parameter(3)?))
        }
    }
}

impl Hdm {
    pub fn heading(&self) -> F32Error {
        self.base.parameter(0)
    }
}

impl Hdt {
    pub fn heading(&self) -> F32Error {
        self.base.parameter(0)
    }
}

impl Mwd {
    pub fn direction_true(&self) -> F32Error {
        if self.base.parameters[1] == "T" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "T" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }

    pub fn direction_magnetic(&self) -> F32Error {
        if self.base.parameters[1] == "M" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "M" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found".to_string()))
        }
    }

    pub fn speed_knots(&self) -> F32Error {
        if self.base.parameters[5] == "N" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "N" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
                * match self.base.parameters[5].as_str() {
                    "K" => 0.539957,
                    "M" => 0.868976,
                    _ => 0.0,
                })
        }
    }

    pub fn speed_mph(&self) -> F32Error {
        if self.base.parameters[5] == "M" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "M" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
                * match self.base.parameters[5].as_str() {
                    "K" => 0.621371,
                    "N" => 1.15078,
                    _ => 0.0,
                })
        }
    }

    pub fn speed_kph(&self) -> F32Error {
        if self.base.parameters[5] == "K" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "K" {
            self.base.parameter(6)
        } else {
            Ok(self.base.parameter::<f32>(4)?
                * match self.base.parameters[5].as_str() {
                    "M" => 1.852,
                    "N" => 1.60934,
                    _ => 0.0,
                })
        }
    }
}

impl Mwv {
    pub fn angle_relative(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[1] == "R" {
                self.base.parameter(0)
            } else {
                Err(NmeaError("Not found.".to_string()))
            }
        } else {
            Err(NmeaError("Data not valid.".to_string()))
        }
    }

    pub fn angle_true(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[1] == "T" {
                self.base.parameter(0)
            } else {
                Err(NmeaError("Not found.".to_string()))
            }
        } else {
            Err(NmeaError("Data not valid.".to_string()))
        }
    }

    pub fn speed_kph(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[3] == "K" {
                self.base.parameter(2)
            } else {
                Err(NmeaError("Not found".to_string()))
            }
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }

    pub fn speed_mph(&self) -> F32Error {
        if self.base.parameters[4] == "A" {
            if self.base.parameters[3] == "M" {
                self.base.parameter(2)
            } else {
                Err(NmeaError("Not found".to_string()))
            }
        } else {
            Err(NmeaError("Invalid data".to_string()))
        }
    }
}

impl Vhw {
    pub fn heading_true(&self) -> F32Error {
        if self.base.parameters[1] == "T" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "T" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn heading_magnetic(&self) -> F32Error {
        if self.base.parameters[1] == "M" {
            self.base.parameter(0)
        } else if self.base.parameters[3] == "M" {
            self.base.parameter(2)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed_knots(&self) -> F32Error {
        if self.base.parameters[5] == "N" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "N" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed_mph(&self) -> F32Error {
        if self.base.parameters[5] == "M" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "M" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }

    pub fn water_speed_kph(&self) -> F32Error {
        if self.base.parameters[5] == "K" {
            self.base.parameter(4)
        } else if self.base.parameters[7] == "K" {
            self.base.parameter(6)
        } else {
            Err(NmeaError("Not found.".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
