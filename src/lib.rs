pub mod gps;
pub mod heading;
pub mod miscellaneous;
pub mod wind;
pub mod base;
pub mod ais;

use gps::{gga::Gga, gll::Gll, rmc::Rmc, gsv::Gsv};
use miscellaneous::{zda::Zda, rsa::Rsa, mtw::Mtw, mda::Mda, xdr::Xdr};
use heading::{rot::Rot, hdg::Hdg, hdm::Hdm, hdt::Hdt, vtg::Vtg};
use wind::{mwd::Mwd, mwv::Mwv, vhw::Vhw};
use ais::Ais;

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
}
