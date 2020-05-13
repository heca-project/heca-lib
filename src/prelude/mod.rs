pub use crate::holidays::chol::Chol;
pub use crate::holidays::shabbos::Parsha;
pub use crate::holidays::special_parsha::SpecialParsha;
pub use crate::holidays::yom_tov::YomTov;
pub use crate::holidays::{Holiday, Name};
#[doc(inline)]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::num::NonZeroU8;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Serialize, Deserialize)]

pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Shabbos,
}

/// Notes: This panics if input is larger than 6, so this will be converted to a TryFrom in a future release.
impl TryFrom<u8> for Day {
    type Error = ConversionError;
    fn try_from(input: u8) -> Result<Self, ConversionError> {
        match input {
            0 => Ok(Day::Sunday),
            1 => Ok(Day::Monday),
            2 => Ok(Day::Tuesday),
            3 => Ok(Day::Wednesday),
            4 => Ok(Day::Thursday),
            5 => Ok(Day::Friday),
            6 => Ok(Day::Shabbos),
            _ => Err(ConversionError::TooManyDaysInWeek),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Molad {
    pub(crate) day: chrono::NaiveDateTime,
    pub(crate) remainder: u16,
}

impl Molad {
    pub fn get_day_utc(&self) -> chrono::NaiveDateTime {
        self.day
    }
    pub fn get_chalakim(&self) -> u16 {
        self.remainder
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Location {
    Israel,
    Chul,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub enum HebrewMonth {
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
impl TryFrom<u8> for HebrewMonth {
    type Error = ConversionError;
    fn try_from(input: u8) -> Result<Self, Self::Error> {
        match input {
            0 => Ok(HebrewMonth::Tishrei),
            1 => Ok(HebrewMonth::Cheshvan),
            2 => Ok(HebrewMonth::Kislev),
            3 => Ok(HebrewMonth::Teves),
            4 => Ok(HebrewMonth::Shvat),
            5 => Ok(HebrewMonth::Adar),
            6 => Ok(HebrewMonth::Adar1),
            7 => Ok(HebrewMonth::Adar2),
            8 => Ok(HebrewMonth::Nissan),
            9 => Ok(HebrewMonth::Iyar),
            10 => Ok(HebrewMonth::Sivan),
            11 => Ok(HebrewMonth::Tammuz),
            12 => Ok(HebrewMonth::Av),
            13 => Ok(HebrewMonth::Elul),
            _ => Err(ConversionError::TooManyHebrewMonths),
        }
    }
}

///Occurs when failing to get a Hebrew Date.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum ConversionError {
    /// Occurs when attempting to get an Adar 1 or Adar 2 in a non-leap year.
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroU8;
    /// #
    /// let result = HebrewDate::from_ymd(5778,HebrewMonth::Adar1,NonZeroU8::new(1).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::IsNotLeapYear);
    /// ```
    IsNotLeapYear,

    /// Occurs when trying to get a Hebrew Date who's day is out of range
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroU8;
    /// #
    /// let result = HebrewDate::from_ymd(5778,HebrewMonth::Adar,NonZeroU8::new(40).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::TooManyDaysInMonth(NonZeroU8::new(29).unwrap()));
    /// ```
    TooManyDaysInMonth(NonZeroU8),

    TooManyDaysInWeek,
    TooManyHebrewMonths,

    /// Occurs when attempting to get a regular Adar in a leap year.
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroU8;
    /// #
    /// let result = HebrewDate::from_ymd(5779,HebrewMonth::Adar,NonZeroU8::new(1).unwrap());
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::IsLeapYear);
    /// ```
    IsLeapYear,
    /// Occurs when attempting to get a year that is before the epoch (currently: year 3764/4).
    ///
    /// # Example:
    /// ```
    /// # use heca_lib::prelude::*;
    /// # use heca_lib::HebrewDate;
    /// # use std::num::NonZeroU8;
    /// #
    /// let result = HebrewDate::from_ymd(2448,HebrewMonth::Nissan,NonZeroU8::new(15).unwrap()); // What was the English day of the Exodus?
    /// assert!(!result.is_ok());
    /// assert_eq!(result.unwrap_err(),ConversionError::YearTooSmall);
    /// ```
    YearTooSmall,
}

impl std::error::Error for ConversionError {}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::IsNotLeapYear => write!(
                f,
                "Can't convert an Adar 1 or Adar 2 of a year which isn't a leap year"
            ),
            ConversionError::TooManyDaysInMonth(d) => {
                write!(f, "Too many days in this month. Month only has {} days", d)
            }
            ConversionError::IsLeapYear => write!(
                f,
                "Can't convert an Adar of a year which is a leap year. Specify Adar1 or Adar2"
            ),
            ConversionError::YearTooSmall => write!(
                f,
                "Cannot build calendar for years below 3764 (After Creation)"
            ),
            ConversionError::TooManyDaysInWeek => write!(f, "Too many days in the week"),
            ConversionError::TooManyHebrewMonths => write!(f, "Wrong Hebrew month number"),
        }
    }
}
/// What Torah Readings are we looking for
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum TorahReadingType {
    /// Yom Tov - Pesach, Shavuos, Sukkos, Shmini Atzeres/Simchas Torah, Rosh Hashana, Yom Kippur and Chol HaMoed.
    YomTov,
    /// Weekday Torah reading - Rosh Chodesh, Chanuka and Purim
    Chol,
    /// Weekly Parsha Torah reading
    Shabbos,
    /// One of the four special Torah portions read every winter (Shekalim, Zachor, Parah and HaChodesh).
    SpecialParsha,
}

/// A Hebrew year can be defined by three variables:
///
/// 1. The first day of Rosh Hashana - Monday (the second day of the week, represented by Beis - **Ba**), Tuesday (the third day of the week, represented by Gimmel - **Ga**), Thursday (the fifth day of the week, represented by Hei - **Ha**) and Shabbos (the seventh day of the week, represented by Zayin - **Za**).
/// 2. The length of the year, specifically, if Cheshvan and Kislev are both full (**She**leima - 30 days long), empty (**Chaseir** - 29 days long), or in regular order ("Kesidra", Cheshvan is 29 days long and Kislev is 30. So the year goes 30,29,30,29 etc.).
/// 3. The day Pesach starts, defined as on Rosh Hashana above.
///
/// So, for example, 5779 is a BaShaZ year - that is, the first day of Rosh Hashana was on a Monday (Beis - **Ba**), Bosh Cheshvan and Kislev are full (Shleimah - **Sh**in),
/// and the first night of Pesach was on Friday night (Zain - **Z** for Shabbos).
///
/// # Examples
///
///
///
/// ~~~
///
/// use heca_lib::HebrewYear;
/// use heca_lib::prelude::*;
/// assert_eq!(HebrewYear::new(5779)?.year_type(),MonthSchedule::BaShaZ);
/// # Ok::<(),ConversionError>(())
/// ~~~
///
/// ## Find out how often does Pesach start on which days:
///
/// ~~~
///
/// use heca_lib::HebrewYear;
/// use heca_lib::prelude::*;
/// let (mut thu, mut tue, mut sun, mut sat) = (0,0,0,0);
/// for year in 3765..9999 {
///     let t = HebrewYear::new(year)?.year_type();
///     match t {
///         MonthSchedule::GaChaH
///         | MonthSchedule::BaShaH
///         | MonthSchedule::BaChaH
///         | MonthSchedule::ZaShaH => thu += 1,
///
///         MonthSchedule::HaShaG
///         | MonthSchedule::ZaShaG
///         | MonthSchedule::ZaChaG
///         | MonthSchedule::BaChaG => tue += 1,
///
///         MonthSchedule::HaShA
///         | MonthSchedule::ZaChA
///         | MonthSchedule::HaChA => sun += 1,
///         
///         MonthSchedule::HaKaZ
///         | MonthSchedule::BaShaZ
///         | MonthSchedule::GaKaZ => sat += 1,
///     }
/// }
/// assert_eq!(thu, 1782);
/// assert_eq!(tue, 1988);
/// assert_eq!(sun, 718); // <-- Note, that Pesach falls out on a Motzei Shabbos only 10% of the time.
/// assert_eq!(sat, 1746);
/// # Ok::<(),ConversionError>(())
///
///
/// ~~~
///
/// ## Find out when will Pesach start on Motzei Shabbos:
///
/// ~~~
/// use heca_lib::HebrewYear;
/// use heca_lib::prelude::*;
/// let mut years: Vec<i32> = Vec::new();
/// for year in 5780..5880 {
///     let t = HebrewYear::new(year).unwrap().year_type();
///     match t {
///         MonthSchedule::HaShA
///         | MonthSchedule::ZaChA
///         | MonthSchedule::HaChA => years.push(year),
///
///         _ => { }
///         
///     }
/// }
/// assert_eq!(years, vec![5781, 5785, 5805, 5808, 5812, 5832, 5835, 5839, 5859, 5863] ); // <-- We'll have two of them over the next few years, and then Pesach won't fall out on Motzei Shabbos for twenty years!
///
/// ~~~
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub enum MonthSchedule {
    BaChaG,
    BaShaH,
    GaChaH,
    HaKaZ,
    HaShA,
    ZaChA,
    ZaShaG,

    BaChaH,
    BaShaZ,
    GaKaZ,
    HaChA,
    HaShaG,
    ZaChaG,
    ZaShaH,
}
