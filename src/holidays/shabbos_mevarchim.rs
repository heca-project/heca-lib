use super::Holiday;
use crate::{
    hebrew::{Month, MonthSchedule, Year},
    prelude::{Chol, Day, Molad, Name, YearSchedule},
};
use tinyvec::TinyVec;

pub(crate) fn get<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    let mut v: TinyVec<[Option<Holiday<u8>>; 4]> = TinyVec::new();
    crate::holidays::chol::get(year, &mut v);
    let month_list = if year.is_leap_year() {
        &[
            Some(Month::Tishrei),
            Some(Month::Cheshvan),
            Some(Month::Kislev),
            Some(Month::Teves),
            Some(Month::Shvat),
            None,
            Some(Month::Adar1),
            Some(Month::Adar2),
            Some(Month::Nissan),
            Some(Month::Iyar),
            Some(Month::Sivan),
            Some(Month::Tammuz),
            Some(Month::Elul),
        ]
    } else {
        &[
            Some(Month::Tishrei),
            Some(Month::Cheshvan),
            Some(Month::Kislev),
            Some(Month::Teves),
            Some(Month::Shvat),
            Some(Month::Adar),
            None,
            None,
            Some(Month::Nissan),
            Some(Month::Iyar),
            Some(Month::Sivan),
            Some(Month::Tammuz),
            Some(Month::Elul),
        ]
    };
    return_vec.extend(
        month_list
            .into_iter()
            .filter_map(|x| *x)
            .map(|hebrew_month| {
                let molad = year.get_molad(hebrew_month).unwrap();
                let dow: u8 = offset(year.and_month_day(hebrew_month, 29).day_of_week());

                let month_after = get_month_after(year, hebrew_month);
                let shabbos_mevarchim_day = year.and_month_day(hebrew_month, 29 - dow); //The last day of the month (that's not Rosh Chodesh) is always the 29th. Sometimes Rosh Chodesh is the 30th, sometimes there is no 30th.
                eprintln!("\n");
                dbg!(
                    month_after,
                    dow,
                    shabbos_mevarchim_day.year(),
                    shabbos_mevarchim_day.month(),
                    shabbos_mevarchim_day.day(),
                    shabbos_mevarchim_day.day_of_week()
                );
                Some(Holiday {
                    day: shabbos_mevarchim_day,
                    name: Name::ShabbosMevarchim {
                        hebrew_month: month_after,
                        molad,
                    },
                    candle_lighting: None,
                })
            }),
    );
}

fn get_month_after(year: &Year, month: Month) -> Month {
    match month {
        Month::Tishrei => Month::Cheshvan,
        Month::Cheshvan => Month::Kislev,
        Month::Kislev => Month::Teves,
        Month::Teves => Month::Shvat,
        Month::Shvat => {
            if year.is_leap_year() {
                Month::Adar1
            } else {
                Month::Adar
            }
        }
        Month::Adar => Month::Nissan,
        Month::Adar1 => Month::Adar2,
        Month::Adar2 => Month::Nissan,
        Month::Nissan => Month::Iyar,
        Month::Iyar => Month::Sivan,
        Month::Sivan => Month::Tammuz,
        Month::Tammuz => Month::Av,
        Month::Av => Month::Elul,
        Month::Elul => Month::Tishrei,
    }
}

fn offset(day: Day) -> u8 {
    match day {
        Day::Friday => 6,
        Day::Thursday => 5,
        Day::Wednesday => 4,
        Day::Tuesday => 3,
        Day::Monday => 2,
        Day::Sunday => 1,
        Day::Shabbos => 0,
    }
}

#[test]
fn test_get_molad() {
    for year in 5700..6000 {
        let mut v: TinyVec<[Option<Holiday<u8>>; 4]> = TinyVec::new();
        get(&Year::new(year).unwrap(), &mut v);
        for v in v {
            let shabbos_mevarchim: crate::secular::Date = v.unwrap().day.into();
            assert_eq!(shabbos_mevarchim.weekday(), crate::prelude::Day::Friday);
        }
    }
}

#[test]
fn test_get_leap_year() {
    assert_eq!(Year::new(5701).unwrap().is_leap_year(), false);
}
