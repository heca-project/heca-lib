use crate::prelude::Day;
use chrono::prelude::*;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
/// The amount of Chalakim in an hour.
pub(crate) const CHALAKIM_PER_HOUR: u16 = 1080;
/// The amount of Chalakim between two Molads.
// See https://www.chabad.org/library/article_cdo/aid/947923/jewish/Kiddush-HaChodesh-Chapter-Six.htm#bartnoteRef8a947923
pub(crate) const CHALAKIM_BETWEEN_MOLAD: u32 =
    29 * 24 * CHALAKIM_PER_HOUR as u32 + 12 * CHALAKIM_PER_HOUR as u32 + 793;

//An array documenting which years are leap years. The Hebrew calendar has a 19 year cycle of leap
//years.
const LEAP_YEARS: [bool; 19] = [
    false, false, true, false, false, true, false, true, false, false, true, false, false, true,
    false, false, true, false, true,
];

// There are three starting dates. Right now, we don't work with negative Gregorian dates, so the
// Epoch period is the first year of the first 19 year cycle after year 0.
//
// 1. Epoch - this is the first day, is on 6:00 PM Shabbos (Saturay) afternoon.
// 2. FIRST_MOLAD - the amount of Chalakim from Epoch to the first Molad -(Tishrei 3673). It was on Monday, September 23rd at 12:16:6 Chalakim
// 3. FIRST_YEAR: Self described - this is the first Hebrew calendar since the epoch.
// 4. FIRST_RH: The first Rosh Hashana since the Epoch.
pub(crate) const FIRST_MOLAD: u16 = 24 * 1080 + 18 * 1080 + (16 * 1080 / 60) + 6;
pub(crate) const FIRST_YEAR: u16 = 3763;
pub(crate) const FIRST_RH: Lazy<chrono::NaiveDateTime> =
    Lazy::new(|| NaiveDate::from_ymd(2, 9, 23).and_hms(18, 0, 0));
pub(crate) const EPOCH: Lazy<chrono::NaiveDateTime> =
    Lazy::new(|| NaiveDate::from_ymd(2, 9, 21).and_hms(18, 0, 0));

// Return the correct schedule for they year. There can be only six possible amount of days, so
// short of a bug on my part, this should never panic.
pub(crate) fn return_year_sched(days: u16) -> usize {
    match days {
        353 => 0,
        354 => 1,
        355 => 2,

        383 => 3,
        384 => 4,
        385 => 5,
        _ => panic!(format!("Wrong amount of days {}", days)),
    }
}
pub(crate) const YEAR_SCHED: [[u8; 14]; 6] = [
    [30, 29, 29, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 29, 0, 0, 30, 29, 30, 29, 30, 29],
    [30, 29, 29, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 29, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
    [30, 30, 30, 29, 30, 0, 30, 29, 30, 29, 30, 29, 30, 29],
];

//This calculates the amount of Chalakim per 19 year cycle.
pub(crate) const AMNT_CHALAKIM_PER_CYCLE: u64 =
    (7 * 13 * CHALAKIM_BETWEEN_MOLAD as u32 + 12 * 12 * CHALAKIM_BETWEEN_MOLAD as u32) as u64;

fn get_molad_for_year(year: i32) -> u64 {
    let amnt_of_cycles: u16 = ((year - i32::from(FIRST_YEAR)) / 19) as u16;

    let mut amnt_chalakim: u64 = (AMNT_CHALAKIM_PER_CYCLE * amnt_of_cycles as u64) as u64;
    let cur_year_in_cycle: u8 = ((year - FIRST_YEAR as i32) % 19) as u8;
    for i in 0..cur_year_in_cycle {
        amnt_chalakim +=
            if LEAP_YEARS[i as usize] { 13 } else { 12 } * CHALAKIM_BETWEEN_MOLAD as u64;
    }

    amnt_chalakim
}

//Does short calculation if this year is a leap year.
pub(crate) fn months_per_year(year: i32) -> u8 {
    let year_in_cycle: usize = ((year - FIRST_YEAR as i32) % 19) as usize;
    if LEAP_YEARS[year_in_cycle] {
        13
    } else {
        12
    }
}

//Calculate how many Chalakim between Epoch and Rosh Hashana, and which day of the week does it
//fall out on.
pub(crate) fn get_rosh_hashana(year: i32) -> (u32, Day, u64) {
    let amnt_chalakim_since_first_molad = get_molad_for_year(year);
    let amnt_chalakim_since_epoch = amnt_chalakim_since_first_molad + u64::from(FIRST_MOLAD);

    let mut amnt_days: u32 = (amnt_chalakim_since_epoch / u64::from(CHALAKIM_PER_HOUR * 24)) as u32;
    let amnt_chalakim: u16 = (amnt_chalakim_since_epoch % u64::from(CHALAKIM_PER_HOUR * 24)) as u16;
    let mut reg_postpone = false;
    //If the Molad is in the afternoon, postpone Rosh Hashana by a day
    if amnt_chalakim > 18 * CHALAKIM_PER_HOUR {
        amnt_days += 1;
        reg_postpone = true;
    }

    //This shouldn't panic, as there are seven options in Day (seven days in week).
    let mut dow = Day::try_from(((amnt_days) % 7) as u8).unwrap();
    // Lo Adu Rosh

    if dow == Day::Sunday || dow == Day::Wednesday || dow == Day::Friday {
        amnt_days += 1;
        reg_postpone = true;
    }

    // See Hilchos Kiddush HaChodesh Halacha 4

    if !reg_postpone
        && dow == Day::Tuesday
        && amnt_chalakim > 9 * CHALAKIM_PER_HOUR + 204
        && months_per_year(year) == 12
    {
        amnt_days += 2;
    }

    if !reg_postpone
        && months_per_year(year - 1) == 13
        && dow == Day::Monday
        && amnt_chalakim > 12 * CHALAKIM_PER_HOUR + 3 * CHALAKIM_PER_HOUR + 589
    {
        amnt_days += 1;
    }

    //This shouldn't panic, as there are seven options in Day (seven days in week).
    dow = Day::try_from(((amnt_days) % 7) as u8).unwrap();

    (amnt_days, dow, amnt_chalakim_since_first_molad)
}

pub(crate) fn day_of_last_rh(days_since_first_rh: u32) -> u32 {
    let mut cur_year = (FIRST_YEAR as i32) + (19 * days_since_first_rh / 6956) as i32;
    debug_assert!(get_rosh_hashana(cur_year).0 <= days_since_first_rh);
    while get_rosh_hashana(cur_year + 1).0 <= days_since_first_rh {
        cur_year += 1;
    }
    cur_year as u32
}
#[cfg(test)]
mod tests {
    use crate::convert::HebrewDate;
    use crate::prelude::*;
    use chrono::Duration;
    use std::convert::TryInto;
    use std::num::NonZeroU8;

    use super::*;
    #[test]
    fn years_correct_sum() {
        assert_eq!(YEAR_SCHED[0].iter().map(|x| (*x) as u64).sum::<u64>(), 353);
        assert_eq!(YEAR_SCHED[1].iter().map(|x| (*x) as u64).sum::<u64>(), 354);
        assert_eq!(YEAR_SCHED[2].iter().map(|x| (*x) as u64).sum::<u64>(), 355);
        assert_eq!(YEAR_SCHED[3].iter().map(|x| (*x) as u64).sum::<u64>(), 383);
        assert_eq!(YEAR_SCHED[4].iter().map(|x| (*x) as u64).sum::<u64>(), 384);
        assert_eq!(YEAR_SCHED[5].iter().map(|x| (*x) as u64).sum::<u64>(), 385);
    }

    #[test]
    fn years_have_right_days() {
        use rayon;
        use rayon::prelude::*;

        (((FIRST_YEAR + 1) as i32)..297000)
            .into_par_iter()
            .map(|i: i32| {
                let amnt_days_between_rh_and_epoch = get_rosh_hashana(i as i32).0;
                let amnt_days_in_year = get_rosh_hashana(i + 1).0 - amnt_days_between_rh_and_epoch;
                return_year_sched(amnt_days_in_year.try_into().unwrap());
            })
            .count();
    }
    #[test]
    fn compare_hebrew_day_elul_sanity_check() {
        let mut orig_date = NaiveDate::from_ymd(1901, 8, 15).and_hms(18, 0, 0);
        for j in 1..=29 {
            let heb_day =
                HebrewDate::from_ymd(5661, HebrewMonth::Elul, NonZeroU8::new(j).unwrap()).unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }

    #[test]
    fn compare_hebrew_day_tishrei_sanity_check() {
        let mut orig_date = NaiveDate::from_ymd(1900, 9, 23).and_hms(18, 0, 0);
        for j in 1..=30 {
            let heb_day =
                HebrewDate::from_ymd(5661, HebrewMonth::Tishrei, NonZeroU8::new(j).unwrap())
                    .unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }
    #[test]
    fn compare_hebrew_day_adar1_sanity_check() {
        let mut orig_date = NaiveDate::from_ymd(1900, 1, 30).and_hms(18, 0, 0);
        for j in 1..=30 {
            let heb_day =
                HebrewDate::from_ymd(5660, HebrewMonth::Adar1, NonZeroU8::new(j).unwrap()).unwrap();
            let back = heb_day.to_gregorian();
            println!("{}", j);
            assert_eq!(orig_date, back);
            orig_date = orig_date + Duration::days(1);
        }
    }

    #[test]
    fn test_rh_against_working_list() {
        test_against_working_list("RoshHashanaList", 1, HebrewMonth::Tishrei);
    }
    #[test]
    fn test_adar1_against_working_list() {
        test_against_working_list("Adar1List", 1, HebrewMonth::Adar1);
    }

    fn test_against_working_list(filename: &str, day: u8, month: HebrewMonth) {
        let file_contents = std::fs::read_to_string(format!("./testing/{}", filename)).unwrap();
        file_contents
            .split("\n")
            .filter(|x| *x != "")
            .for_each(|x| {
                let res = x.split(" ").collect::<Vec<&str>>();
                if res.len() != 1 {
                    let eng_day = HebrewDate::from_ymd(
                        res[0].parse::<i32>().unwrap(),
                        month,
                        NonZeroU8::new(day as u8).unwrap(),
                    )
                    .unwrap()
                    .to_gregorian()
                        + Duration::days(1);
                    println!("{:?}", eng_day);
                    let sp = res[1].split("/").collect::<Vec<&str>>();
                    let (month, day, year) = (sp[0], sp[1], sp[2]);
                    assert_eq!(month.parse::<u64>().unwrap() as u32, eng_day.month());
                    assert_eq!(day.parse::<u64>().unwrap() as u32, eng_day.day());
                    assert_eq!(year.parse::<u64>().unwrap() as i32, eng_day.year());
                }
            });
    }
}
