pub mod gps;
pub mod heading;
pub mod miscellaneous;
pub mod wind;
pub mod base;

use gps::{gga::Gga, gll::Gll, rmc::Rmc};
use miscellaneous::{zda::Zda, rsa::Rsa};
use heading::{rot::Rot, hdg::Hdg, hdm::Hdm, hdt::Hdt, vtg::Vtg};
use wind::{mwd::Mwd, mwv::Mwv, vhw::Vhw};

pub enum Nmea0183 {
    GGA(Gga),
    GLL(Gll),
    HDG(Hdg),
    HDM(Hdm),
    HDT(Hdt),
    MWD(Mwd),
    MWV(Mwv),
    RMC(Rmc),
    ROT(Rot),
    RSA(Rsa),
    VHW(Vhw),
    VTG(Vtg),
    ZDA(Zda),
}
