pub mod ais;
pub mod base;
pub mod gps;
pub mod heading;
pub mod miscellaneous;
pub mod wind;

use ais::Ais;
use base::{Nmea0183Base, NmeaError};
use gps::{gga::Gga, gll::Gll, gsv::Gsv, rmc::Rmc};
use heading::{hdg::Hdg, hdm::Hdm, hdt::Hdt, rot::Rot, vtg::Vtg};
use miscellaneous::{mda::Mda, mtw::Mtw, rsa::Rsa, xdr::Xdr, zda::Zda};
use wind::{mwd::Mwd, mwv::Mwv, vhw::Vhw};

#[derive(Debug)]
pub enum Nmea0183 {
    AIS(Ais),
    GGA(Gga),
    GLL(Gll),
    GSV(Gsv),
    HDG(Hdg),
    HDM(Hdm),
    HDT(Hdt),
    MDA(Mda),
    MTW(Mtw),
    MWD(Mwd),
    MWV(Mwv),
    RMC(Rmc),
    ROT(Rot),
    RSA(Rsa),
    VHW(Vhw),
    VTG(Vtg),
    XDE(Xdr),
    ZDA(Zda),
    Unkown(Nmea0183Base),
}


impl Nmea0183 {
    pub fn from_string(message : &String) -> Result<Nmea0183, NmeaError> {
        match Nmea0183Base::from_string(message) {
            Ok(b) => Ok(Nmea0183::Unkown(b)),
            Err(e) => Err(NmeaError(format!("{e:?}"))),
        }
    }
}