use crate::hebrew::Month;
use crate::{internal::calendar, prelude::*};
use std::convert::{TryFrom, TryInto};
use std::num::NonZeroU8;

#[derive(Debug, Copy, Clone)]
/// Date holds a specific Hebrew Date. It can be constructed individually or through hebrew::Year.
pub struct Date {
    pub(crate) day: NonZeroU8,
    pub(crate) month: Month,
    pub(crate) year: Year,
}

impl TryFrom<crate::secular::Date> for Date {
    type Error = ConversionError;
    fn try_from(orig: crate::secular::Date) -> Result<Self, Self::Error> {
        Date::from_gregorian(orig)
    }
}

impl Eq for Date {}
impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.day == other.day
            && self.month == other.month
            && self.year().year() == other.year().year()
    }
}

use super::Year;
use calendar::{day_of_last_rh, CHALAKIM_PER_DAY, DAYS_BETWEEN_RH_AND_EPOCH, EPOCH, FIRST_RH};
use std::{cmp::Ordering, ops::Sub};
impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        if self.year().year() < other.year().year() {
            Ordering::Less
        } else if self.year().year() > other.year().year() {
            Ordering::Greater
        } else if (self.month as i32) < (other.month as i32) {
            Ordering::Less
        } else if (self.month as i32) > (other.month as i32) {
            Ordering::Greater
        } else if self.day < other.day {
            Ordering::Less
        } else if self.day > other.day {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[test]
fn test_cmp_hebrew_date() {
    assert_eq!(
        Year::new(5740).and_month_day(Month::Shvat, 15)
            < Year::new(5741).and_month_day(Month::Tishrei, 1),
        true
    );
    assert_eq!(
        Year::new(5760).and_month_day(Month::Tishrei, 1)
            < Year::new(5760).and_month_day(Month::Tishrei, 2),
        true
    );
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Date {
    /// Returns a HebrewDate on success or a ConversionError on failure.
    ///
    /// # Arguments
    /// * `year` - The Hebrew year since creation.
    /// * `month` - The Hebrew month.
    /// * `day` - The Hebrew day of month.
    ///
    /// # Error Values
    /// * `YearTooSmall` - This algorithm won't work if the year is before 3764.
    /// * `IsLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so if you want to
    /// convert a day in Adar 1 or Adar 2 of a leap year, specify which one.
    ///  * `IsNotLeapYear` - I treat Adar, Adar 1 and Adar 2 as three seperate months, so it won't
    ///  make sense to get the English date of the first of Adar 1 or Adar 2 if the year isn't a
    ///  leap year.
    ///  * `TooManyDaysInMonth` - There are either 29 or 30 days in a month, so it doesn't make sense
    ///  to find the 50th day of Nissan.
    ///
    /// # Notes:
    ///
    /// Day must be above zero. If it's below zero, the function returns TooManyDaysInMonth. In a future release, day will be a NonZeroU8 so that it will be impossible to supply a negative number.
    #[must_use]
    pub fn from_ymd(year: u32, month: Month, day: u8) -> Date {
        Year::new(year).and_month_day(month, day)
    }

    #[must_use]
    pub(crate) fn from_ymd_internal(
        month: Month,
        day: u8,
        hebrew_year: Year,
    ) -> Result<Date, ConversionError> {
        //Get a HebrewDate object from the Hebrew Year, Month, and Day. Can fail if the year is too
        //small or the day is less than one.
        if !hebrew_year.is_leap_year() && (month == Month::Adar1 || month == Month::Adar2) {
            return Err(ConversionError::IsNotLeapYear);
        }

        if hebrew_year.is_leap_year() && month == Month::Adar {
            return Err(ConversionError::IsLeapYear);
        }

        if day as u8 > hebrew_year.sched[month as usize] {
            return Err(ConversionError::TooManyDaysInMonth(
                NonZeroU8::new(hebrew_year.sched[month as usize]).unwrap(),
            ));
        }

        Ok(Date {
            month,
            day: NonZeroU8::new(day).unwrap(),
            year: hebrew_year,
        })
    }

    #[must_use]
    pub fn day_of_week(&self) -> Day {
        let amnt_days_between_rh_and_epoch = self.year.days_since_epoch;
        let sched = self.year.sched;
        let mut amnt_days_in_month: u16 = 0;
        if self.month != Month::Tishrei {
            for item in sched.iter().take(self.month as usize) {
                amnt_days_in_month += u16::from(*item);
            }
        }

        let amnt_days = amnt_days_between_rh_and_epoch as u32
            + u64::from(amnt_days_in_month) as u32
            + self.day.get() as u32
            - 1;
        ((amnt_days % 7) as u8).try_into().unwrap()
    }

    #[must_use]
    pub(crate) fn from_gregorian(date: crate::secular::Date) -> Result<Date, ConversionError> {
        if date < (*FIRST_RH + Duration::days(2 + 365)) {
            return Err(ConversionError::YearTooSmall);
        }
        let days_since_first_rh = ((date - *FIRST_RH).days() + 2) as u32;

        let hebrew_year = Year::new(day_of_last_rh(days_since_first_rh).try_into().unwrap());
        Ok(hebrew_year.get_hebrewdate_from_days_after_rh(days_since_first_rh))
    }
    pub(crate) fn from_days_since_epoch(days_since_epoch: u32) -> Date {
        let days_since_first_rh: u32 = days_since_epoch - DAYS_BETWEEN_RH_AND_EPOCH as u32;
        let hebrew_year = Year::new(day_of_last_rh(days_since_first_rh).try_into().unwrap());
        hebrew_year.get_hebrewdate_from_days_after_rh(days_since_first_rh)
    }
    pub(crate) fn to_gregorian(&self) -> crate::secular::Date {
        let amnt_days_between_rh_and_epoch = self.year.days_since_epoch;
        let sched = self.year.sched;
        let mut amnt_days_in_month: u16 = 0;
        if self.month != Month::Tishrei {
            for item in sched.iter().take(self.month as usize) {
                amnt_days_in_month += u16::from(*item);
            }
        }

        let amnt_days = amnt_days_between_rh_and_epoch as u32
            + u64::from(amnt_days_in_month) as u32
            + self.day.get() as u32
            - 1;
        *EPOCH + Duration::days(amnt_days as i32)
    }
    ///Get the Hebrew day of month.
    #[inline]
    #[must_use]
    pub fn day(&self) -> NonZeroU8 {
        self.day
    }

    ///Get the Hebrew month of year
    #[inline]
    #[must_use]
    pub fn month(&self) -> Month {
        self.month
    }

    ///Get the Hebrew year.

    #[inline]
    #[must_use]
    pub fn year(&self) -> Year {
        self.year
    }

    pub(crate) fn to_chalakim(&self) -> u64 {
        let month_num: u8 = self.month.into();

        (self.year().days_since_epoch as u64 * CHALAKIM_PER_DAY as u64)
            + self
                .year()
                .sched
                .into_iter()
                .enumerate()
                .filter(|x| x.0 < month_num as usize)
                .fold(0 as u64, |old, new| old + *new.1 as u64)
                * (CHALAKIM_PER_DAY as u64)
            + (self.day().get() as u64 * CHALAKIM_PER_DAY as u64)
    }
}

impl Sub<Date> for Date {
    type Output = Duration;
    fn sub(self, rhs: Date) -> Self::Output {
        Duration {
            chalakim: self.to_chalakim() as i64 - rhs.to_chalakim() as i64,
        }
    }
}

mod tests {
    #[test]
    fn get_year() {
        use super::*;
        for j in 0..100 {
            let mut original_day = crate::secular::Date::from_ymd(16 + j, 10, 4).and_hms(18, 0, 0);
            for _i in 1..366 {
                let h_day = Date::from_gregorian(original_day).unwrap();
                let ne_day = h_day.to_gregorian();
                assert_eq!(original_day, ne_day);
                original_day = original_day + Duration::days(1);
            }
        }
    }
    #[test]
    fn test_day_of_week() {
        use crate::hebrew::{Month, Year};
        use crate::prelude::*;
        assert_eq!(
            Year::new(5780).and_month_day(Month::Iyar, 18).day_of_week(),
            Day::Tuesday
        );

        assert_eq!(
            Year::new(5700)
                .and_month_day(Month::Adar1, 29)
                .day_of_week(),
            Day::Shabbos
        );
    }
}

#[test]
fn test_subtract_day() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 1);
    let date2 = Year::new(5751).and_month_day(Month::Tishrei, 2);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        CHALAKIM_PER_DAY as i32
    );
}

#[test]
fn test_subtract_month() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 29);
    let date2 = Year::new(5751).and_month_day(Month::Tishrei, 30);
    eprintln!("{}", (date2 - date1).get_chalakim().chalakim() / 1080);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        (CHALAKIM_PER_DAY as i32)
    );
}

#[test]
fn test_subtract_month_1() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 30);
    let date2 = Year::new(5751).and_month_day(Month::Cheshvan, 1);
    eprintln!("{}", (date2 - date1).get_chalakim().chalakim() / 1080);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        (CHALAKIM_PER_DAY as i32)
    );
}

#[test]
fn test_subtract_month_2() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 1);
    let date2 = Year::new(5751).and_month_day(Month::Cheshvan, 1);
    eprintln!("{}", (date2 - date1).get_chalakim().chalakim() / 1080);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        (CHALAKIM_PER_DAY as i32) * 30
    );
}

#[test]
fn test_subtract_year() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 1);
    let date2 = Year::new(5752).and_month_day(Month::Tishrei, 1);
    eprintln!("{}", (date2 - date1).get_chalakim().chalakim() / 1080);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        (CHALAKIM_PER_DAY as i32) * 354
    );
}

#[test]
fn test_subtract_full() {
    let date1 = Year::new(5751).and_month_day(Month::Tishrei, 1);
    let date2 = Year::new(5751).and_month_day(Month::Elul, 29);
    eprintln!("{}", (date2 - date1).get_chalakim().chalakim() / 1080);
    assert_eq!(
        (date2 - date1).get_chalakim().chalakim(),
        (CHALAKIM_PER_DAY as i32) * 353
    );
}

#[test]
fn check_day_of_week() {
    assert_eq!(
        Year::new(5700)
            .and_month_day(Month::Tishrei, 1)
            .day_of_week(),
        Day::Thursday
    );
    assert_eq!(
        Year::new(5700)
            .and_month_day(Month::Nissan, 15)
            .day_of_week(),
        Day::Tuesday
    );
    assert_eq!(
        Year::new(5701)
            .and_month_day(Month::Tishrei, 1)
            .day_of_week(),
        Day::Thursday
    );
}
