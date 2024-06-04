pub mod ais;
pub mod base;
pub mod gps;
pub mod heading;
pub mod miscellaneous;
pub mod wind;

use ais::vdm::Vdm;
use base::{Nmea0183Base, NmeaError};
use gps::{
    bwc::Bwc, bwr::Bwr, gbs::Gbs, gga::Gga, gll::Gll, grs::Grs, gsa::Gsa, gst::Gst, gsv::Gsv,
    gxa::Gxa, rmc::Rmc, trf::Trf,
};
use heading::{hdg::Hdg, hdm::Hdm, hdt::Hdt, rot::Rot, vtg::Vtg};
use miscellaneous::{
    dbs::Dbs, dbt::Dbt, dpt::Dpt, mda::Mda, mtw::Mtw, rsa::Rsa, vlw::Vlw, xdr::Xdr, zda::Zda,
};
use wind::{mwd::Mwd, mwv::Mwv, vhw::Vhw, vwr::Vwr, vwt::Vwt};

#[derive(Debug)]
pub enum Nmea0183 {
    BWC(Bwc),
    BWR(Bwr),
    DBS(Dbs),
    DBT(Dbt),
    DPT(Dpt),
    GBS(Gbs),
    GGA(Gga),
    GLL(Gll),
    GRS(Grs),
    GSA(Gsa),
    GST(Gst),
    GSV(Gsv),
    GXA(Gxa),
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
    TRF(Trf),
    VDM(Vdm),
    VHW(Vhw),
    VLW(Vlw),
    VTG(Vtg),
    VWR(Vwr),
    VWT(Vwt),
    XDR(Xdr),
    ZDA(Zda),

    Unknown(Nmea0183Base),
}

impl Nmea0183 {
    pub fn from_string(message: &String) -> Result<Nmea0183, NmeaError> {
        match Nmea0183Base::from_string(message) {
            Ok(b) => Ok(classify(b)),
            Err(e) => Err(NmeaError(format!("{e:?}"))),
        }
    }
}

pub fn classify(b: Nmea0183Base) -> Nmea0183 {
    match b.message.to_uppercase().as_str() {
        "DBS" => Nmea0183::DBS(Dbs::new(b)),
        "DBT" => Nmea0183::DBT(Dbt::new(b)),
        "DPT" => Nmea0183::DPT(Dpt::new(b)),
        "GGA" => Nmea0183::GGA(Gga::new(b)),
        "GLL" => Nmea0183::GLL(Gll::new(b)),
        "GSA" => Nmea0183::GSA(Gsa::new(b)),
        "GSV" => Nmea0183::GSV(Gsv::new(b)),
        "HDG" => Nmea0183::HDG(Hdg::new(b)),
        "HDM" => Nmea0183::HDM(Hdm::new(b)),
        "HDT" => Nmea0183::HDT(Hdt::new(b)),
        "MDA" => Nmea0183::MDA(Mda::new(b)),
        "MTW" => Nmea0183::MTW(Mtw::new(b)),
        "MWD" => Nmea0183::MWD(Mwd::new(b)),
        "MWV" => Nmea0183::MWV(Mwv::new(b)),
        "RMC" => Nmea0183::RMC(Rmc::new(b)),
        "ROT" => Nmea0183::ROT(Rot::new(b)),
        "RSA" => Nmea0183::RSA(Rsa::new(b)),
        "VDM" => Nmea0183::VDM(Vdm::new(b)),
        "VHW" => Nmea0183::VHW(Vhw::new(b)),
        "VLW" => Nmea0183::VLW(Vlw::new(b)),
        "VTG" => Nmea0183::VTG(Vtg::new(b)),
        "VWR" => Nmea0183::VWR(Vwr::new(b)),
        "VWT" => Nmea0183::VWT(Vwt::new(b)),
        "XDR" => Nmea0183::XDR(Xdr::new(b)),
        "ZDA" => Nmea0183::ZDA(Zda::new(b)),
        _ => Nmea0183::Unknown(b),
    }
}

pub fn sender(_nmea0183: &Nmea0183) -> Result<String, String> {
    Err(String::from("Not yet implemented"))
}

pub fn message(_nmea0183: &Nmea0183) -> Result<String, String> {
    Err(String::from("Not yet implemented"))
}
