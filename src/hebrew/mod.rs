mod date;
mod year;
use crate::prelude::ConversionError;
#[doc(inline)]
pub use date::*;
use std::convert::TryFrom;
#[doc(inline)]
pub use year::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum Month {
    Tishrei,
    Cheshvan,
    Kislev,
    Teves,
    Shvat,
    Adar,
    Adar1,
    Adar2,
    Nissan,
    Iyar,
    Sivan,
    Tammuz,
    Av,
    Elul,
}
impl TryFrom<u8> for Month {
    type Error = ConversionError;
    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            0 => Ok(Month::Tishrei),
            1 => Ok(Month::Cheshvan),
            2 => Ok(Month::Kislev),
            3 => Ok(Month::Teves),
            4 => Ok(Month::Shvat),
            5 => Ok(Month::Adar),
            6 => Ok(Month::Adar1),
            7 => Ok(Month::Adar2),
            8 => Ok(Month::Nissan),
            9 => Ok(Month::Iyar),
            10 => Ok(Month::Sivan),
            11 => Ok(Month::Tammuz),
            12 => Ok(Month::Av),
            13 => Ok(Month::Elul),
            _ => Err(ConversionError::TooManyHebrewMonths),
        }
    }
}

impl From<Month> for u8 {
    fn from(input: Month) -> Self {
        match input {
            Month::Tishrei => 0,
            Month::Cheshvan => 1,
            Month::Kislev => 2,
            Month::Teves => 3,
            Month::Shvat => 4,
            Month::Adar => 5,
            Month::Adar1 => 6,
            Month::Adar2 => 7,
            Month::Nissan => 8,
            Month::Iyar => 9,
            Month::Sivan => 10,
            Month::Tammuz => 11,
            Month::Av => 12,
            Month::Elul => 13,
        }
    }
}
