use crate::prelude::Day;
use std::convert::TryInto;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, Copy, Clone, PartialOrd, Ord)]
pub struct Date {
    days_since_epoch: i32,
    year: i32,
    month: u8,
    day: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
    chalakim: u8,
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.days_since_epoch == other.days_since_epoch
            && self.minutes == other.minutes
            && self.seconds == other.seconds
            && self.chalakim == other.chalakim
    }
}

#[derive(Debug)]
pub enum Error {
    TooManyDays,
    TooManyMonths,
}
fn last_day_of_month_common_year(m: u8) -> u8 {
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    months[m as usize - 1]
}
fn last_day_of_month(y: i32, m: u8) -> u8 {
    if m != 2 || !is_leap(y) {
        last_day_of_month_common_year(m)
    } else {
        29
    }
}
fn is_leap(y: i32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

#[cfg(features = "chrono-enabled")]
impl From<Date> for chrono::NaiveDate {
    fn from(orig: Date) -> Self {
        chrono::NaiveDate::from_ymd(orig.year, orig.month as u32, orig.day as u32)
    }
}
impl Date {
    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    ///source is http://howardhinnant.github.io/date_algorithms.html#days_from_civil
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Date {
        Self::from_ymd_opt(year, month, day).unwrap()
    }

    pub fn and_hms(&self, hours: u8, minutes: u8, seconds: u8) -> Date {
        Self {
            year: self.year,
            days_since_epoch: self.days_since_epoch,
            month: self.month,
            day: self.day,
            hours: 0,
            minutes: 0,
            seconds: 0,
            chalakim: 0,
        } + Duration::hours(hours as i32)
            + Duration::minutes(minutes as i32)
            + Duration::seconds(seconds as i32)
    }

    pub fn and_chalakim(&self, chalakim: u8) -> Date {
        Self {
            year: self.year,
            days_since_epoch: self.days_since_epoch,
            month: self.month,
            day: self.day,
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
            chalakim: 0,
        } + Duration::chalakim(chalakim as i32)
    }

    pub fn from_ymd_opt(year: i32, month: u8, day: u8) -> Result<Date, Error> {
        if month == 0 || month > 12 {
            return Err(Error::TooManyMonths);
        } else if day > last_day_of_month(year, month) {
            return Err(Error::TooManyDays);
        }
        let new_year = year - if month <= 2 { 1 } else { 0 };
        let era = (if new_year >= 0 {
            new_year
        } else {
            new_year - 399
        } / 400) as i32;
        let yoe = (new_year as i32 - era * 400) as u32;
        debug_assert!(yoe <= 399);
        let doy: u32 = ((153 as i64 * (month as i8 + (if month > 2 { -3 } else { 9 })) as i64 + 2)
            / 5
            + (day as i8 - 1) as i64) as u32; // [0, 365]
        debug_assert!(doy <= 365);
        let doe: u32 = yoe * 365 + yoe / 4 - yoe / 100 + doy; // [0, 146096]
        debug_assert!(doe <= 146096);
        let days_since_epoch = era * 146097 + doe as i32 - 719468;

        Ok(Date {
            days_since_epoch,
            year,
            month,
            day,
            hours: 0,
            minutes: 0,
            seconds: 0,
            chalakim: 0,
        })
    }

    pub fn weekday(&self) -> Day {
        use std::convert::TryFrom;

        let z = self.days_since_epoch;
        let day_of_week = if z >= -4 {
            (z + 4) % 7
        } else {
            (z + 5) % 7 + 6
        };

        Day::try_from(day_of_week as u8).unwrap()
    }
}

///source is http://howardhinnant.github.io/date_algorithms.html#days_from_civil
fn num_to_ymd(mut z: i32) -> (i32, u8, u8) {
    z += 719468;
    let era: i32 = (if z >= 0 { z } else { z - 146096 }) / 146097;

    let doe: u64 = (z - era * 146097).try_into().unwrap();
    debug_assert!(doe <= 146096);
    let yoe: u64 = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    debug_assert!(yoe <= 399);

    let y: i32 = yoe as i32 + era * 400;
    let doy: u64 = doe - (365 * yoe + yoe / 4 - yoe / 100);
    debug_assert!(doy <= 365);
    let mp: u64 = (5 * doy + 2) / 153;
    debug_assert!(mp <= 11);
    let d: u8 = (doy - (153 * mp + 2) / 5 + 1) as u8;
    debug_assert!(d <= 31);
    let m: u8 = (mp as i64 + (if mp < 10 { 3 } else { -9 })) as u8;
    let y = y + if m <= 2 { 1 } else { 0 };
    debug_assert!(m <= 12); // [1, 12]
    (y, m, d)
}

impl Add<Days> for Date {
    type Output = Date;
    fn add(self, rhs: Days) -> Self::Output {
        let days_since_epoch = self.days_since_epoch + rhs.days as i32;
        let (year, month, day) = num_to_ymd(days_since_epoch);
        Date {
            days_since_epoch,
            year,
            month,
            day,
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
            chalakim: self.chalakim,
        }
    }
}

impl Sub<Date> for Date {
    type Output = Days;
    fn sub(self, rhs: Date) -> Self::Output {
        let days_since_epoch = self.days_since_epoch - rhs.days_since_epoch;
        Days {
            days: days_since_epoch,
        }
    }
}

impl Sub<Days> for Date {
    type Output = Date;
    fn sub(self, rhs: Days) -> Self::Output {
        let days_since_epoch = self.days_since_epoch - rhs.days as i32;
        let (year, month, day) = num_to_ymd(days_since_epoch);
        Date {
            days_since_epoch,
            year,
            month,
            day,
            hours: self.hours,
            minutes: self.minutes,
            seconds: self.seconds,
            chalakim: 0,
        }
    }
}

impl Add<Chalakim> for Date {
    type Output = Date;
    fn add(self, rhs: Chalakim) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_chalakim = new_english_date.chalakim as i32;
        let amnt_seconds = (cur_chalakim + rhs.chalakim) / 60;
        let amnt_chalakim = (cur_chalakim + rhs.chalakim) % 60;
        new_english_date.chalakim = amnt_chalakim as u8;
        new_english_date + Duration::seconds(amnt_seconds)
    }
}

impl Sub<Chalakim> for Date {
    type Output = Date;
    fn sub(self, rhs: Chalakim) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_chalakim = new_english_date.chalakim as i32;
        let amnt_seconds = (cur_chalakim - rhs.chalakim) / 60;
        let amnt_chalakim = (cur_chalakim - rhs.chalakim) % 60;
        new_english_date.chalakim = amnt_chalakim as u8;
        new_english_date + Duration::seconds(amnt_seconds)
    }
}

impl Add<Seconds> for Date {
    type Output = Date;
    fn add(self, rhs: Seconds) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_seconds = new_english_date.seconds as i32;
        let amnt_minutes = (cur_seconds + rhs.seconds) / 60;
        let amnt_seconds = (cur_seconds + rhs.seconds) % 60;
        new_english_date.seconds = amnt_seconds as u8;
        new_english_date + Duration::minutes(amnt_minutes)
    }
}

impl Sub<Seconds> for Date {
    type Output = Date;
    fn sub(self, rhs: Seconds) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_seconds = new_english_date.seconds as i32;
        let amnt_minutes = (cur_seconds - rhs.seconds) / 60;
        let amnt_seconds = (cur_seconds - rhs.seconds) % 60;
        new_english_date.seconds = amnt_seconds as u8;
        new_english_date - Duration::minutes(amnt_minutes)
    }
}

impl Add<Minutes> for Date {
    type Output = Date;
    fn add(self, rhs: Minutes) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_minutes = new_english_date.minutes as i32;
        let amnt_hours = (cur_minutes + rhs.minutes) / 60;
        let amnt_minutes = (cur_minutes + rhs.minutes) % 60;
        new_english_date.minutes = amnt_minutes as u8;
        new_english_date + Duration::hours(amnt_hours)
    }
}

impl Sub<Minutes> for Date {
    type Output = Date;
    fn sub(self, rhs: Minutes) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_minutes = new_english_date.minutes as i32;
        let amnt_hours = (cur_minutes - rhs.minutes) / 60;
        let amnt_minutes = (cur_minutes - rhs.minutes) % 60;
        new_english_date.minutes = amnt_minutes as u8;
        new_english_date - Duration::hours(amnt_hours)
    }
}

impl Add<Hours> for Date {
    type Output = Date;
    fn add(self, rhs: Hours) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_hours = new_english_date.hours as i32;
        let amnt_days = (cur_hours + rhs.hours) / 24;
        let amnt_hours = (cur_hours + rhs.hours) % 24;
        new_english_date.hours = amnt_hours as u8;
        new_english_date + Duration::days(amnt_days)
    }
}
impl Sub<Hours> for Date {
    type Output = Date;
    fn sub(self, rhs: Hours) -> Self::Output {
        let mut new_english_date = self.clone();
        let cur_hours = new_english_date.hours as i32;
        let amnt_days = (cur_hours - rhs.hours) / 24;
        let amnt_hours = (cur_hours - rhs.hours) % 24;
        new_english_date.hours = amnt_hours as u8;
        new_english_date - Duration::days(amnt_days)
    }
}

impl From<crate::hebrew::Date> for Date {
    fn from(orig: crate::hebrew::Date) -> Self {
        orig.to_gregorian()
    }
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Duration {
    days: i32,
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Days {
    days: i32,
}

impl Days {
    pub fn days(&self) -> i32 {
        self.days
    }
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Hours {
    hours: i32,
}

impl Hours {
    pub fn hours(&self) -> i32 {
        self.hours
    }
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Minutes {
    minutes: i32,
}

impl Minutes {
    pub fn minutes(&self) -> i32 {
        self.minutes
    }
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Seconds {
    seconds: i32,
}

impl Seconds {
    pub fn seconds(&self) -> i32 {
        self.seconds
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Chalakim {
    chalakim: i32,
}

impl Chalakim {
    pub fn chalakim(&self) -> i32 {
        self.chalakim
    }
}

impl Duration {
    pub fn days(days: i32) -> Days {
        Days { days }
    }
    pub fn minutes(minutes: i32) -> Minutes {
        Minutes { minutes }
    }
    pub fn seconds(seconds: i32) -> Seconds {
        Seconds { seconds }
    }
    pub fn hours(hours: i32) -> Hours {
        Hours { hours }
    }
    pub fn chalakim(chalakim: i32) -> Chalakim {
        Chalakim { chalakim }
    }
}

#[test]

fn test_add_ymd_not_crash() {
    let original_date = Date::from_ymd(-1000, 1, 1);
    let mut amount_days = 0;
    for i in -1000..1000 {
        for j in 1..=12 {
            for k in 1..=last_day_of_month(i, j) {
                assert_eq!(
                    Date::from_ymd(i, j, k),
                    original_date + Duration::days(amount_days)
                );
                amount_days += 1;
            }
        }
    }
}

#[test]
#[cfg(features = "chrono-enabled")]
fn test_chrono_add() {
    use chrono::NaiveDate;
    let original_chrono_date = NaiveDate::from_ymd(-100, 1, 1);
    let original_date = Date::from_ymd(-100, 1, 1);
    let mut amount_days = 0;
    for _ in -100_000..100_000 {
        let new_naive_date: NaiveDate = (original_date + Duration::days(amount_days)).into();

        assert_eq!(
            new_naive_date,
            original_chrono_date + chrono::Duration::days(amount_days as i64)
        );
        amount_days += 1;
    }
}

#[test]
#[cfg(features = "chrono-enabled")]
fn test_chrono_sub() {
    use chrono::NaiveDate;
    let original_chrono_date = NaiveDate::from_ymd(2020, 1, 1);
    let original_date = Date::from_ymd(2020, 1, 1);
    let mut amount_days = 0;
    for _ in -100_000..100_000 {
        let new_naive_date: NaiveDate = (original_date - Duration::days(amount_days)).into();

        assert_eq!(
            new_naive_date,
            original_chrono_date - chrono::Duration::days(amount_days as i64)
        );
        amount_days += 1;
    }
}

#[test]
#[cfg(features = "chrono-enabled")]
fn test_weekday() {
    use chrono::{Datelike, NaiveDate, Weekday};
    let original_chrono_date = NaiveDate::from_ymd(2020, 1, 1);
    let original_date = Date::from_ymd(2020, 1, 1);
    let mut amount_days = 0;
    for i in -100_000..(100_000 as i64) {
        let new_naive_date: NaiveDate =
            NaiveDate::from_ymd(-100_000, 1, 1) + chrono::Duration::days(i);
        let new_secular_date: Date = Date::from_ymd(-100_000, 1, 1) + Duration::days(i as i32);
        let secular_weekday = new_secular_date.weekday();
        let chrono_weekday = match new_naive_date.weekday() {
            Weekday::Sun => Day::Sunday,
            Weekday::Mon => Day::Monday,
            Weekday::Tue => Day::Tuesday,
            Weekday::Wed => Day::Wednesday,
            Weekday::Thu => Day::Thursday,
            Weekday::Fri => Day::Friday,
            Weekday::Sat => Day::Shabbos,
        };
        assert_eq!(chrono_weekday, secular_weekday);
        amount_days += 1;
    }
}

#[test]
fn test_to_from_ymd_not_crash() {
    for i in -100..1000 {
        for j in 1..=12 {
            for k in 1..=last_day_of_month(i, j) {
                let date = Date::from_ymd(i, j, k);
                eprintln!("original: {} {} {}; date: {:?}", i, j, k, date);
                assert_eq!(i, date.year);
                assert_eq!(j, date.month);
                assert_eq!(k, date.day);
            }
        }
    }
}

#[test]
#[cfg(features = "chrono-enabled")]
fn test_add_seconds() {
    let original_date = Date::from_ymd(-1000, 1, 1).unwrap();
    for i in 0..10_000 {
        assert_eq!(
            original_date + Duration::days(i),
            original_date + Duration::seconds(i * 24 * 60 * 60)
        );
    }
}

#[test]
fn test_from_ymd() {
    assert_eq!(Date::from_ymd(0, 1, 1).days_since_epoch, -719528);
}

#[test]
#[cfg(features = "chrono-enabled")]
fn test_chrono_negative() {
    let original_chrono_date = NaiveDate::from_ymd(2020, 1, 1);
    let original_date = Date::from_ymd(2020, 1, 1).unwrap();
    let mut amount_days = -1000;
    for _ in -100_000..100_000 {
        let new_naive_date: NaiveDate = (original_date - Duration::days(amount_days)).into();

        assert_eq!(
            new_naive_date,
            original_chrono_date - chrono::Duration::days(amount_days as i64)
        );
        amount_days += 1;
    }
}

#[test]
fn test_create_secular_date() {
    assert_eq!(
        Date::from_ymd(2020, 1, 2).and_hms(3, 4, 5).and_chalakim(6),
        Date {
            days_since_epoch: 18263,
            year: 2020,
            month: 1,
            day: 2,
            hours: 3,
            minutes: 4,
            seconds: 5,
            chalakim: 6
        }
    );
}

#[test]
fn test_create_secular_date_2() {
    assert_eq!(
        Date::from_ymd(2020, 12, 31)
            .and_hms(23, 59, 59)
            .and_chalakim(255),
        Date {
            days_since_epoch: 18628,
            year: 2021,
            month: 1,
            day: 1,
            hours: 0,
            minutes: 0,
            seconds: 3,
            chalakim: 15
        }
    );
}
