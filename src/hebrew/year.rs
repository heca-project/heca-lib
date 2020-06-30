use std::convert::{TryFrom, TryInto};

use crate::hebrew::Date;
use crate::holidays::{
    chabad_holidays::{self},
    chol::{self},
    israeli_holidays::{self},
    minor_holiday::{self},
    omer::{self},
    shabbos::{self},
    shabbos_mevarchim,
    special_parsha::{self},
    yom_tov::{self},
    DailyStudy, Holiday,
};

use crate::daily_study::{
    daf_yomi::{self},
    rambam, yerushalmi_yomi,
};

use crate::internal::calendar::{CHALAKIM_BETWEEN_MOLAD, FIRST_MOLAD};
use crate::prelude::*;
use std::{num::NonZeroU8, ops::Add};
use tinyvec::TinyVec;

use crate::hebrew::Month;
use crate::internal::calendar::{
    get_rosh_hashana, months_per_year, return_year_sched, CHALAKIM_PER_DAY, CHALAKIM_PER_HOUR,
    CHALAKIM_PER_MINUTE, EPOCH, FIRST_YEAR, YEAR_SCHED,
};
use crate::{
    prelude::{ConversionError, Molad},
    secular::{Days, Hours},
};

/// HebrewYear holds data on a given year. It's faster to get multiple HebrewDates from
/// an existing HebrewYear rather than generating each one on its own.
#[derive(Copy, Clone, Debug)]
pub struct Year {
    pub(crate) year: u32,
    pub(crate) day_of_rh: Day,
    pub(crate) day_of_next_rh: Day,
    pub(crate) months_per_year: u8,
    pub(crate) sched: &'static [u8; 14],
    pub(crate) year_len: u16,
    pub(crate) days_since_epoch: u32,
    pub(crate) chalakim_since_epoch: u64,
}

impl Year {
    #[inline]
    pub fn month_schedule(&self) -> MonthSchedule {
        if self.sched[1] == 30 && self.sched[2] == 30 {
            MonthSchedule::Malei
        } else if self.sched[1] == 29 && self.sched[2] == 30 {
            MonthSchedule::Kesidra
        } else if self.sched[1] == 29 && self.sched[2] == 29 {
            MonthSchedule::Chaseir
        } else {
            panic!("Cheshvan has {} days and Kislev has {} days");
        }
    }

    #[inline]
    pub(crate) fn new_unchecked(year: u32) -> Year {
        let cur_rh = get_rosh_hashana(year);
        let next_rh = get_rosh_hashana(year + 1);
        let days_since_epoch = cur_rh.0 as u32;
        let chalakim_since_epoch = cur_rh.2;
        let year_len = (next_rh.0 - cur_rh.0) as u16;
        let months_per_year = months_per_year(year);
        let sched = &YEAR_SCHED[return_year_sched(year_len as u16)];

        Year {
            day_of_rh: get_rosh_hashana(year).1,
            year,
            day_of_next_rh: get_rosh_hashana(year + 1).1,
            months_per_year,
            sched: sched,
            days_since_epoch,
            year_len,
            chalakim_since_epoch,
        }
    }

    #[inline]
    pub fn new_opt(year: u32) -> Result<Year, ConversionError> {
        //! Returns a new HebrewYear on success or a ConversionError on failure.
        //!
        //! # Arguments
        //!
        //! `year` - The Hebrew year
        //!
        if year < (FIRST_YEAR + 1) as u32 {
            Err(ConversionError::YearTooSmall)
        } else {
            Ok(Year::new_unchecked(year))
        }
    }

    #[inline]
    pub fn new(year: u32) -> Year {
        //! Returns a new HebrewYear on success or a ConversionError on failure.
        //!
        //! # Arguments
        //!
        //! `year` - The Hebrew year
        //!
        if year < (FIRST_YEAR + 1) as u32 {
            panic!("{:?}", ConversionError::YearTooSmall)
        } else {
            Year::new_unchecked(year)
        }
    }

    #[inline]
    /// Returns if this year is a leap year.
    ///
    /// ```
    /// use heca_lib::prelude::*;
    /// use heca_lib::HebrewYear;
    /// assert_eq!(HebrewYear::new(5779)?.is_leap_year(),true);
    /// # Ok::<(),ConversionError>(())
    /// ```
    pub fn is_leap_year(&self) -> bool {
        self.months_per_year == 13
    }

    /// Returns the type of year.
    ///
    /// A Hebrew year can be one of 14 combinations of length and starting day.
    ///
    /// # Returns
    ///
    /// A [MonthSchedule](../heca_lib/prelude/enum.MonthSchedule.html)
    #[inline]
    pub fn year_type(&self) -> YearSchedule {
        if self.months_per_year == 12 {
            match self.day_of_rh {
                Day::Monday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::BaShaH
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        YearSchedule::BaChaG
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, stars on Monday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Tuesday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        YearSchedule::GaChaH
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, starts on Tuesday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Thursday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        YearSchedule::HaKaZ
                    } else if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::HaShA
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, starts on Thursday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Shabbos => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::ZaShaG
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        YearSchedule::ZaChA
                    } else {
                        panic!(format!(
                            "Year {} is 12 months, stars on Shabbos, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                x => panic!(format!("Rosh Hashana should never fall out on {:?}", x)),
            }
        } else {
            match self.day_of_rh {
                Day::Monday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::BaShaZ
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        YearSchedule::BaChaH
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, stars on Monday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Tuesday => {
                    if self.sched[1] == 29 && self.sched[2] == 30 {
                        YearSchedule::GaKaZ
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, starts on Tuesday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Thursday => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::HaShaG
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        YearSchedule::HaChA
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, starts on Thursday, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                Day::Shabbos => {
                    if self.sched[1] == 30 && self.sched[2] == 30 {
                        YearSchedule::ZaShaH
                    } else if self.sched[1] == 29 && self.sched[2] == 29 {
                        YearSchedule::ZaChaG
                    } else {
                        panic!(format!(
                            "Year {} is 13 months, stars on Shabbos, yet has Cheshvan {} days and Kislev {} days",
                            self.year, self.sched[1], self.sched[2]
                        ))
                    }
                }
                x => panic!(format!("Rosh Hashana should never fall out on {:?}", x)),
            }
        }
    }

    #[inline]
    pub fn first_day(&self) -> Day {
        self.day_of_rh
    }
    /// Returns the year.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::num:: NonZeroU8;
    /// use heca_lib::prelude::*;
    /// use heca_lib::{HebrewDate, HebrewYear};
    /// let year = HebrewYear::new(5779)?;
    /// assert_eq!(year.year(), 5779);
    /// # Ok::<(),ConversionError>(())
    /// ```
    #[inline]
    pub fn year(&self) -> u32 {
        self.year
    }
    /// Returns a HebrewDate from the current year and a supplied month and day.
    ///
    /// # Arguments:
    ///
    /// `month` - The Hebrew month.
    ///
    /// `day` - The day of the Hebrew month.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::num:: NonZeroU8;
    /// use heca_lib::prelude::*;
    /// use heca_lib::{HebrewDate, HebrewYear};
    /// let year = HebrewYear::new(5779)?;
    /// assert_eq!(
    ///        year.get_hebrew_date(Month::Tishrei,  10)?,
    ///        HebrewDate::from_ymd(5779, Month::Tishrei,  10)?
    ///  );
    /// # Ok::<(),ConversionError>(())
    /// ```
    ///
    /// # Notes:
    ///
    /// Day must be above zero. If it's below zero, the function returns TooManyDaysInMonth. In a future release, day will be a NonZeroU8 so that it will be impossible to supply a negative number.
    #[inline]
    pub fn and_month_day(self, month: Month, day: u8) -> Date {
        self.and_month_day_opt(month, day).unwrap()
    }
    #[inline]
    pub fn and_month_day_opt(self, month: Month, day: u8) -> Result<Date, ConversionError> {
        Date::from_ymd_internal(month, day, self)
    }

    pub(crate) fn get_hebrewdate_from_days_after_rh(self, amnt_days: u32) -> Date {
        let mut remainder = amnt_days - self.days_since_epoch as u32;
        let mut month: u64 = 0;
        for days_in_month in self.sched.iter() {
            if remainder < u32::from(*days_in_month) {
                break;
            }
            month += 1;
            remainder -= u32::from(*days_in_month);
        }
        Date {
            year: self,
            month: Month::try_from(month as u8).unwrap(),
            day: NonZeroU8::new((remainder + 1) as u8).unwrap(),
        }
    }
    /// Returns all the days when the Torah is read.
    ///
    /// # Arguments
    ///
    /// `location` - Specify if you're looking for the calendar in Israel or in the Diaspora. Is
    /// relevent as there's only one day of Yom Tov in Israel while there are two day of Yom Tov outside.
    /// Since we don't read the Weekly Parsha on Yom Tov, in a year when the 8th day of Pesach is on a Shabbos,
    /// Israelis read the next Parsha while the Diaspora reads the Yom Tov Parsha, catching up in the summer.
    ///
    /// `yt_types` - An array containing `HolidayType`. This should be used as a flag to
    /// specify which types of Torah readings you want to list.
    ///
    /// # Returns
    ///
    /// Returns an array (or a vec) of days.
    ///
    /// # Note
    ///
    /// This may unsorted, and is returned under no defined order.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num:: NonZeroU8;
    /// use heca_lib::prelude::*;
    /// use heca_lib::{HebrewDate, HebrewYear};
    /// let year = HebrewYear::new(5779)?;
    /// let shabbosim = year.get_holidays_vec(Location::Chul, &[HolidayType::Shabbos, HolidayType::SpecialParsha, HolidayType::Chol, HolidayType::YomTov], &None::<fn(&HebrewDate)>);
    /// let mut count = 0;
    /// for s in shabbosim.into_iter() {
    ///   if s.name() == Name::Shabbos(Parsha::Bereishis) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,Month::Tishrei,  27)?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == Name::SpecialParsha(SpecialParsha::Zachor) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,Month::Adar2,  9)?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == Name::Chol(Chol::Chanukah1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,Month::Kislev,  25)?);
    ///     count += 1;
    ///   }
    ///   else if s.name() == Name::YomTov(YomTov::Shavuos1) {
    ///     assert_eq!(s.day(), HebrewDate::from_ymd(5779,Month::Sivan,  6)?);
    ///     count += 1;
    ///   }
    /// }
    /// assert_eq!(count,4);
    /// # Ok::<(),ConversionError>(())
    /// ```
    pub fn get_holidays<S: Clone, T: Clone + Fn(Date) -> S, U: Clone + Fn(Date) -> S>(
        &self,
        location: Location,
        yt_types: &[HolidayType],
        array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<S>>>>,
        shkiya_func: Option<T>,
        tzeis_func: Option<U>,
    ) {
        if yt_types.contains(&HolidayType::YomTov) {
            yom_tov::get(
                &self,
                location,
                array_vec,
                shkiya_func.clone(),
                tzeis_func.clone(),
            )
        }
        if yt_types.contains(&HolidayType::Chol) {
            chol::get(self, array_vec);
        }
        if yt_types.contains(&HolidayType::Shabbos) {
            let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<S>>; 32]>::new();
            yom_tov::get(
                &self,
                location,
                &mut ignore_vec,
                None::<fn(Date) -> S>,
                None::<fn(Date) -> S>,
            );
            shabbos::get(
                self,
                location,
                &ignore_vec,
                array_vec,
                shkiya_func,
                tzeis_func,
            );
        }
        if yt_types.contains(&HolidayType::SpecialParsha) {
            special_parsha::get(self, array_vec)
        }
        if yt_types.contains(&HolidayType::Omer) {
            omer::get(self, array_vec)
        }
        if yt_types.contains(&HolidayType::MinorHolidays) {
            minor_holiday::get(self, array_vec)
        }
        if yt_types.contains(&HolidayType::ShabbosMevarchim) {
            shabbos_mevarchim::get(self, array_vec)
        }
        if yt_types.contains(&HolidayType::IsraeliHolidays(true)) {
            israeli_holidays::get(self, array_vec, true)
        } else if yt_types.contains(&HolidayType::IsraeliHolidays(false)) {
            israeli_holidays::get(self, array_vec, false)
        }
        if yt_types.contains(&HolidayType::ChabadHolidays) {
            chabad_holidays::get(self, array_vec)
        }
    }

    pub fn get_holidays_vec<S: Clone, T: Clone + Fn(Date) -> S, U: Clone + Fn(Date) -> S>(
        &self,
        location: Location,
        yt_types: &[HolidayType],
        shkiya_func: Option<T>,
        tzeis_func: Option<U>,
    ) -> Vec<Holiday<S>> {
        let mut collect_vec = tinyvec::TinyVec::<[Option<Holiday<S>>; 32]>::new();
        self.get_holidays(
            location,
            yt_types,
            &mut collect_vec,
            shkiya_func,
            tzeis_func,
        );
        collect_vec
            .into_iter()
            .filter_map(|x| if x.is_some() { x } else { None })
            .collect()
    }

    pub fn get_daily_study<S: Clone, T: Fn(Date) -> S>(
        &self,
        daily_study_type: &[DailyStudyType],
        array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<DailyStudy>>>,
    ) {
        if daily_study_type.contains(&DailyStudyType::DafYomi) {
            daf_yomi::get(&self, array_vec)
        }
        if daily_study_type.contains(&DailyStudyType::YerushalmiYomi) {
            yerushalmi_yomi::get(&self, array_vec)
        }
        if daily_study_type.contains(&DailyStudyType::RambamOneChapter) {
            rambam::one_chapter::get(&self, array_vec)
        }
        if daily_study_type.contains(&DailyStudyType::RambamThreeChapters) {
            rambam::three_chapters::get(&self, array_vec)
        }
    }
    /// Returns the Molad of a given month, or a ConversionError if trying to get Molad of a month which is does not exist in that year.
    ///
    /// # Note:
    /// The Molad has no modern Halachic significance since Rosh Chodesh isn't derived from the Molad. However, it is useful to know as some say that one should know the Molad during the Birkas HaChodesh.
    pub fn get_molad(&self, month: Month) -> Result<Molad, ConversionError> {
        let chalakim_since_epoch = if self.is_leap_year() {
            match month {
                Month::Tishrei => 0,
                Month::Cheshvan => 1,
                Month::Kislev => 2,
                Month::Teves => 3,
                Month::Shvat => 4,
                Month::Adar1 => 5,
                Month::Adar2 => 6,
                Month::Nissan => 7,
                Month::Iyar => 8,
                Month::Sivan => 9,
                Month::Tammuz => 10,
                Month::Av => 11,
                Month::Elul => 12,
                Month::Adar => {
                    return Err(ConversionError::IsLeapYear);
                }
            }
        } else {
            match month {
                Month::Tishrei => 0,
                Month::Cheshvan => 1,
                Month::Kislev => 2,
                Month::Teves => 3,
                Month::Shvat => 4,
                Month::Adar => 5,
                Month::Nissan => 6,
                Month::Iyar => 7,
                Month::Sivan => 8,
                Month::Tammuz => 9,
                Month::Av => 10,
                Month::Elul => 11,
                Month::Adar1 => return Err(ConversionError::IsNotLeapYear),
                Month::Adar2 => return Err(ConversionError::IsNotLeapYear),
            }
        } * CHALAKIM_BETWEEN_MOLAD as u64
            + self.chalakim_since_epoch
            + FIRST_MOLAD as u64;

        Ok(Molad {
            chalakim_since_epoch,
        })
    }
}

impl Add<Days> for Date {
    type Output = Date;
    fn add(self, rhs: Days) -> Self::Output {
        let amount_days_in_year: u16 = self.year().sched.iter().map(|x| *x as u16).sum();
        let mut remainder: u16 = self.day().get() as u16
            + (0..u8::from(self.month()))
                .map(|x| self.year().sched[x as usize] as u16)
                .sum::<u16>()
            + rhs.days() as u16;

        if amount_days_in_year < remainder {
            remainder -= amount_days_in_year;
            Date {
                day: NonZeroU8::new(1).unwrap(),
                month: Month::Tishrei,
                year: Year::new(self.year().year + 1),
            } + Duration::days(remainder as i32 - 1)
        } else {
            let year = self.year;

            let mut month: u64 = 0;
            for days_in_month in self.year().sched.iter() {
                if remainder <= u16::from(*days_in_month) {
                    break;
                }
                month += 1;
                remainder -= u16::from(*days_in_month);
            }
            Date {
                year,
                month: Month::try_from(month as u8).unwrap(),
                day: NonZeroU8::new((remainder) as u8).unwrap(),
            }
        }
    }
}

#[test]
fn test_add_date_1() {
    let date: Date = Year::new(5770).and_month_day(Month::Elul, 29);
    assert_eq!(
        date + Duration::days(1),
        Year::new(5771).and_month_day(Month::Tishrei, 1)
    );
}

#[test]
fn test_add_date_2() {
    let date: Date = Year::new(5770).and_month_day(Month::Elul, 28);
    assert_eq!(
        date + Duration::days(1),
        Year::new(5770).and_month_day(Month::Elul, 29)
    );
}

#[test]
fn test_add_date_3() {
    let date: Date = Year::new(5770).and_month_day(Month::Tishrei, 1);
    assert_eq!(
        date + Duration::days(1),
        Year::new(5770).and_month_day(Month::Tishrei, 2)
    );
}
#[test]
fn test_get_molad() {
    let year = crate::hebrew::Year::new(5780);
    let molad: crate::secular::Date = year.get_molad(Month::Cheshvan).unwrap().into();
    assert_eq!(
        molad,
        crate::secular::Date::from_ymd(2019, 10, 28)
            .and_hms(18, 34, 0)
            .and_chalakim(6)
    );
}

/// Returns a HebrewDate on success, or a ConversionError on failure.
///
/// # Arguments
/// * `date` - The Gregorian date.
///
/// # Note:
/// Hebrew days start at sundown, not midnight, so there isn't a full 1:1 mapping between
/// Gregorian days and Hebrew. So when you look up the date of Rosh Hashana 5779, most calendars will say that it's on Monday the 10th of September, 2018, while Rosh Hashana really started at sundown on the 9th of September.
///
/// I'm trying to be a _bit_ more precise, so I made the date cutoff at 6:00 PM. So for example:
///
/// ```
/// use std::num:: NonZeroU8;
/// use std::convert::TryInto;
///
/// use chrono::prelude::*;
/// use chrono::offset::TimeZone;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
/// let hebrew_date: HebrewDate = NaiveDate::from_ymd(2018,9,10).and_hms(17,59,59).try_into()?;
/// assert_eq!(hebrew_date,HebrewDate::from_ymd(5779,Month::Tishrei, 1)?);
/// # Ok::<(),ConversionError>(())
/// ```
///
/// while
///
/// ```
/// use std::num:: NonZeroU8;
/// use std::convert::TryInto;
///
/// use chrono::prelude::*;
/// use chrono::offset::TimeZone;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
///
/// let hebrew_date: HebrewDate = NaiveDate::from_ymd(2018,9,10).and_hms(18,0,0).try_into()?;
/// assert_eq!(hebrew_date, HebrewDate::from_ymd(5779,Month::Tishrei, 2)?);
/// # Ok::<(),ConversionError>(())
/// ```
/// # Error Values:
/// * YearTooSmall - This algorithm won't work if the year is before year 4.
///
#[cfg(features = "chrono-enabled")]
impl TryFrom<NaiveDateTime> for Date {
    type Error = ConversionError;
    fn try_from(original_day: NaiveDateTime) -> Result<Date, ConversionError> {
        Date::from_gregorian(original_day)
    }
}

/// Gets the Gregorian date for the current Hebrew date.
///
/// # Notes
///
/// This function returns the DateTime of the given HebrewDate at nightfall.
///
/// For example, Yom Kippur 5779 started at sunset of September 18, 2018. So
/// ```
/// use std::num:: NonZeroU8;
///
/// use chrono::prelude::*;
/// use heca_lib::prelude::*;
/// use heca_lib::HebrewDate;
///
/// let gregorian_date: NaiveDateTime = HebrewDate::from_ymd(5779,Month::Tishrei, 10)?.into();
/// assert_eq!(gregorian_date ,NaiveDate::from_ymd(2018, 9,18).and_hms(18,00,00));
/// # Ok::<(),ConversionError>(())
/// ```
/// ## Algorithm:
/// The conversion is done (at the moment) according to the calculation of the Rambam (Maimonidies), as is documented in [Hilchos Kiddush Ha'chodesh](https://www.sefaria.org/Mishneh_Torah%2C_Sanctification_of_the_New_Month.6.1?lang=bi&with=all&lang2=en).
///
/// The algorithm is as follows:
///
/// 1. There are exactly 1080 Chalakim (parts) in an hour.
/// 2. There are exactly (well, not really. But it's close enough that we use that number as exact.) 29 days, 12 hours, and 793 Chalakim between new moons.
///
///  So that's the basic numbers. Regarding the calendar itself:
///
/// 3. All months are either 29 or 30 days long.
/// 4. There are either 12 or 13 months in the Hebrew calendar, depending if it's a leap year. When it's a leap year, Adar (which generally is in the late winter or early spring) is doubled into a "first Adar" (Adar1) and a "second Adar" (Adar2).
/// 5. There is a 19 year cycle of leap years. So the first two years of the cycle are regular years, the one after that's a leap year. Then another two are regular, then a leap year. Then it's regular, leap, regular, regular, leap, regular, regular, leap.
/// 6. Year 3763 was the first year of its 19 year cycle.
/// 7. Now you can calculate when's the New Moon before a given Rosh Hashana.
///
///  So how to calculate Rosh Hashana:
///
/// 8. If the New Moon is in the afternoon, Rosh Hashana is postponed to the next day.
/// 9. If Rosh Hashana's starting on a Sunday (Saturday night), Wednesday (Tuesday night), or Friday (Thursday night) - postpone it by a day.
///
///  If any of the above two conditions were fulfilled. Good. You just found Rosh Hashana. If not:
///
/// 10. If the New Moon is on a Tuesday after 3am+204 Chalakim and the coming year is not a leap year, Rosh Hashana is postponed to that upcoming Thursday instead.
/// 11. If the New Moon is on a Monday after 9am+589 Chalakim, and the previous year was a leap year, then Rosh Hashana is postponed to Tuesday.
///
///
///  Now you have all the Rosh Hashanas.
///
/// 12. In general, months alternate between “Full” (30 days long) and “Empty” (29 days long) months. So Tishrei is full, Teves is empty, Shvat is full, Adar is empty, Nissan is full.
/// 13. When the year is a leap year, Adar 1 is full and Adar 2 is empty. (So a full Shvat is followed by a full Adar1).
///
///  Knowing this, you can calculate any other date of the year.
///
///  But wait! We're playing with the date when Rosh Hashana will start, so not every year will be the same length! How do we make up these days?
///
///  So there's a last little bit:
///
/// 14. Cheshvan and Kislev are variable length months – some years both are full, some years both are empty, and some years Cheshvan is full and Kislev is empty - depending on the day Rosh Hashana starts (and the day _the next Rosh Hashana starts_) and how many days are in the year.
#[cfg(features = "chrono-enabled")]
impl From<Date> for NaiveDateTime {
    fn from(h: Date) -> Self {
        let result = h.to_gregorian();
        NaiveDate::from_ymd(result.year(), result.month(), result.day()).and_hms(18, 0, 0)
    }
}

mod test {
    #[test]
    fn make_new_year() {
        use super::*;

        for i in 4000..5000 {
            println!("{}", i);
            Year::new(i);
        }
    }

    #[test]
    fn check_year_type() {
        use super::*;
        for i in 3765..9999 {
            println!("{}", i);
            let y = Year::new(i);
            let t = y.year_type();
            match t {
                YearSchedule::GaChaH
                | YearSchedule::BaShaH
                | YearSchedule::BaChaH
                | YearSchedule::ZaShaH => assert_eq!(
                    y.and_month_day(Month::Nissan, 16).to_gregorian().weekday(),
                    Day::Thursday
                ),

                YearSchedule::HaShaG
                | YearSchedule::ZaShaG
                | YearSchedule::ZaChaG
                | YearSchedule::BaChaG => assert_eq!(
                    y.and_month_day(Month::Nissan, 16).to_gregorian().weekday(),
                    Day::Tuesday
                ),
                YearSchedule::HaShA | YearSchedule::ZaChA | YearSchedule::HaChA => assert_eq!(
                    y.and_month_day(Month::Nissan, 16).to_gregorian().weekday(),
                    Day::Sunday
                ),
                YearSchedule::HaKaZ | YearSchedule::BaShaZ | YearSchedule::GaKaZ => assert_eq!(
                    y.and_month_day(Month::Nissan, 16).to_gregorian().weekday(),
                    Day::Shabbos
                ),
            }
        }
    }
}

#[test]
fn test_month_schedule() {
    for i in 2..6000 {
        let y = Year::new(i);
        y.month_schedule();
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MonthSchedule {
    Chaseir,
    Kesidra,
    Malei,
}

#[test]
fn ensure_no_panic() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[
                HolidayType::IsraeliHolidays(false),
                HolidayType::ChabadHolidays,
                HolidayType::MinorHolidays,
                HolidayType::YomTov,
                HolidayType::Shabbos,
                HolidayType::SpecialParsha,
                HolidayType::Chol,
            ],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
    }
}

#[test]
fn ensure_shabbos_on_shabbos() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[
                HolidayType::Shabbos,
                HolidayType::ShabbosMevarchim,
                HolidayType::SpecialParsha,
            ],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
        result.iter().for_each(|x| {
            if x.day().day_of_week() != Day::Shabbos {
                panic!("{:?}", x);
            }
        });
    }
}

#[test]
fn ensure_first_day_of_selichos_on_sunday() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::MinorHolidays],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
        result.iter().for_each(|x| {
            if x.name() == Name::MinorHoliday(minor_holiday::MinorHoliday::LeilSlichos)
                && x.day().day_of_week() != Day::Sunday
            {
                panic!("{:?}", x.day().day_of_week());
            }
        });
    }
}

#[test]
fn ensure_shabbos_chazon_is_on_shabbos() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::MinorHolidays],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
        result.iter().for_each(|x| {
            if x.name() == Name::MinorHoliday(minor_holiday::MinorHoliday::ShabbosChazon)
                && x.day().day_of_week() != Day::Shabbos
            {
                panic!("{:?}", x);
            }
        });
    }
}

#[test]
fn ensure_shabbos_nachamu_is_on_shabbos() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::MinorHolidays],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
        result.iter().for_each(|x| {
            if x.name() == Name::MinorHoliday(minor_holiday::MinorHoliday::ShabbosNachamu)
                && x.day().day_of_week() != Day::Shabbos
            {
                panic!("{:?}", x);
            }
        });
    }
}

#[test]
fn ensure_shabbos_nachamu_is_one_week_before_chazon() {
    for i in 5700..10_000 {
        let result = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::MinorHolidays],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );
        let shabbos_chazon = result
            .iter()
            .find(|x| x.name() == Name::MinorHoliday(minor_holiday::MinorHoliday::ShabbosChazon));
        let shabbos_nachamu = result
            .iter()
            .find(|x| x.name() == Name::MinorHoliday(minor_holiday::MinorHoliday::ShabbosNachamu));
        assert_eq!(
            (shabbos_chazon.unwrap().day().clone() - shabbos_nachamu.unwrap().day()).get_days(),
            Duration::days(-7)
        );
    }
}

#[test]
fn ensure_all_days_of_sukkos_come_after_another() {
    for i in 10..10_000 {
        let yom_tovs = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::YomTov],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );

        let days_of_sukkos = [
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos1))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos2))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos3))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos4))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos5))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos6))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Sukkos7))
                .unwrap(),
        ];
        days_of_sukkos
            .iter()
            .take(days_of_sukkos.len() - 1)
            .enumerate()
            .for_each(|(index, holiday)| {
                assert_eq!(
                    (days_of_sukkos[index + 1].day() - days_of_sukkos[index].day()).get_days(),
                    Duration::days(1)
                );
            })
    }
}

#[test]
fn ensure_all_days_of_pesach_come_after_another() {
    for i in 10..10_000 {
        let yom_tovs = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::YomTov],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );

        let days_of_pesach = [
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach1))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach2))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach3))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach4))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach5))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach6))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Pesach7))
                .unwrap(),
        ];
        days_of_pesach
            .iter()
            .take(days_of_pesach.len() - 1)
            .enumerate()
            .for_each(|(index, holiday)| {
                assert_eq!(
                    (days_of_pesach[index + 1].day() - days_of_pesach[index].day()).get_days(),
                    Duration::days(1)
                );
            })
    }
}

#[test]
fn ensure_all_days_of_shavuos_come_after_another() {
    for i in 10..10_000 {
        let yom_tovs = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::YomTov],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );

        let days_of_shavuos = [
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Shavuos1))
                .unwrap(),
            yom_tovs
                .iter()
                .find(|x| x.name() == Name::YomTov(YomTov::Shavuos2))
                .unwrap(),
        ];
        days_of_shavuos
            .iter()
            .take(days_of_shavuos.len() - 1)
            .enumerate()
            .for_each(|(index, holiday)| {
                assert_eq!(
                    (days_of_shavuos[index + 1].day() - days_of_shavuos[index].day()).get_days(),
                    Duration::days(1)
                );
            })
    }
}

#[test]
fn ensure_all_days_of_chanuka_come_after_another() {
    for i in 10..10_000 {
        let weekday_holidays = Year::new(i).get_holidays_vec(
            Location::Chul,
            &[HolidayType::Chol],
            Some(|x| Some(5)),
            Some(|x| Some(6)),
        );

        let days_of_chanuka = [
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah1))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah2))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah3))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah4))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah5))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah6))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah7))
                .unwrap(),
            weekday_holidays
                .iter()
                .find(|x| x.name() == Name::Chol(Chol::Chanukah8))
                .unwrap(),
        ];
        days_of_chanuka
            .iter()
            .take(days_of_chanuka.len() - 1)
            .enumerate()
            .for_each(|(index, holiday)| {
                assert_eq!(
                    (days_of_chanuka[index + 1].day() - days_of_chanuka[index].day()).get_days(),
                    Duration::days(1)
                );
            })
    }
}
