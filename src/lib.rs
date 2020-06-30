//! # heca-lib
//! heca-lib is a blazingly fast Hebrew calender library. It's the backend behind the heca program.
//!
//! # Usage:
//!
//! 1. Add to Cargo.toml:
//!
//!```toml
//!     [dependencies]
//!     heca-lib = "1.3"
//!```
//!
//! 2. Add the following to your crate root:
//!
//! ```
//! extern crate heca_lib;
//!
//! ```
//! 3. Import the types:
//!
//!```
//!use heca_lib::prelude::*;
//!use heca_lib::*;
//!```
//!
//! # Overview:
//!
//! ## Convert:
//!
//! This library can convert from Hebrew to Gregorian dates and back. You can get a HebrewDate either from a known Hebrew date or from a Gregorian date:
//!
//! ```
//!
//! use std::num::NonZeroU8;
//! use std::convert::TryInto;
//!
//! use chrono::prelude::*;
//! use heca_lib::prelude::*;
//! use heca_lib::HebrewDate;
//!
//!
//! let hebrew_date: HebrewDate = NaiveDate::from_ymd(2018,9,10).and_hms(17,59,59).try_into().unwrap();
//! assert_eq!(hebrew_date,HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,NonZeroU8::new(1).unwrap()).unwrap());
//! # Ok::<(),ConversionError>(())
//!
//!```
//!
//!You can then get back a Gregorian date from this Hebrew Date.
//!
//!```
//!
//! use std::num::NonZeroU8;
//! use std::convert::TryInto;
//!
//! use chrono::prelude::*;
//! use heca_lib::{HebrewDate};
//! use heca_lib::prelude::*;
//!
//! let eng_day: NaiveDateTime = HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,NonZeroU8::new(10).unwrap())?.into();
//! assert_eq!(eng_day, NaiveDate::from_ymd(2018, 9,18).and_hms(18,00,00));
//! # Ok::<(),ConversionError>(())
//!
//!```
//!
//! ## Get Information on the Hebrew Year
//!
//! This library can also list the major Jewish holidays and Torah readings in a given year (for
//! both Israel and the Diaspora):
//!
//!```
//!
//! use std::num::NonZeroU8;
//!
//! use heca_lib::{HebrewYear,HebrewDate};
//! use heca_lib::prelude::*;
//!
//! assert_eq!(HebrewYear::new(5779)?.get_holidays_vec(Location::Chul, &[HolidayType::Shabbos], &None::<fn(&HebrewDate)>)[0].name(), Name::Shabbos(Parsha::Vayelech));
//! assert_eq!(HebrewYear::new(5779)?.get_holidays_vec(Location::Chul, &[HolidayType::SpecialParsha], &None::<fn(&HebrewDate)> ).iter().find(|x| x.name() == Name::SpecialParsha(SpecialParsha::Zachor)).unwrap().day(),HebrewDate::from_ymd(5779,HebrewMonth::Adar2,NonZeroU8::new(9).unwrap())?);
//! # Ok::<(),ConversionError>(())
//!
//!```
//!
//!
//!# Notes:
//!
//!This library won't work for years before 3764 (4).

pub mod daily_study;
pub mod hebrew;
pub mod holidays;
mod internal;
pub mod prelude;
pub mod secular;
