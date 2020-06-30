use crate::hebrew::{Month, Year};
use crate::{
    holidays::{DailyStudy, Name},
    prelude::Duration,
};
use tinyvec::TinyVec;
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Daf {
    masechta: Masechta,
    daf: u8,
}
pub(crate) fn get(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<DailyStudy>>>,
) {
    let first_day_of_first_daf_yomi_cycle = Year::new(5684).and_month_day(Month::Tishrei, 1);
    let first_day_of_second_daf_yomi_cycle = Year::new(5735).and_month_day(Month::Tammuz, 15);

    if year.year() >= 5684 {
        let amount_of_days_from_first_day_of_daf_yomi_to_rh = year.and_month_day(Month::Tishrei, 1)
            - if year.year() <= 5735 {
                first_day_of_first_daf_yomi_cycle
            } else {
                first_day_of_second_daf_yomi_cycle
            };
        return_vec.extend((0..Year::new(year.year).year_len).into_iter().map(|x| {
            Some(DailyStudy {
                name: Name::DafYomi(get_daf_yomi_for_day_since_day_one_of_daf_yomi(
                    amount_of_days_from_first_day_of_daf_yomi_to_rh
                        .get_days()
                        .days() as u32
                        + x as u32,
                    if year.year() <= 5735 {
                        DafYomiCalendar::First
                    } else {
                        DafYomiCalendar::Second
                    },
                )),
                day: year.and_month_day(Month::Tishrei, 1) + Duration::days(x as i32),
            })
        }))
    }
}

fn get_daf_yomi_for_day_since_day_one_of_daf_yomi(
    day_since_day_one_of_daf_yomi: u32,
    daf_yomi_calendar: DafYomiCalendar,
) -> Daf {
    let mut day = day_since_day_one_of_daf_yomi
        % match daf_yomi_calendar {
            DafYomiCalendar::First => 2702,
            DafYomiCalendar::Second => 2711,
        };
    let mut index = 0;
    let mut masechta: Masechta;

    let daf = loop {
        masechta = MASECHTAS[index];
        if day + 1 < (max_daf(MASECHTAS[index], daf_yomi_calendar) as u32) {
            break day as u8;
        } else {
            day -= max_daf(MASECHTAS[index], daf_yomi_calendar) as u32 - 1;
            index += 1;
        }
    };
    Daf {
        masechta,
        daf: daf + 2,
    }
}

const MASECHTAS: [Masechta; 37] = {
    use Masechta::*;
    [
        Berakhot,
        Shabbat,
        Eruvin,
        Pesachim,
        Shekalim,
        Yoma,
        Sukkah,
        Beitzah,
        RoshHashanah,
        Taanit,
        Megillah,
        MoedKatan,
        Chagigah,
        Yevamot,
        Ketubot,
        Nedarim,
        Nazir,
        Sotah,
        Gittin,
        Kiddushin,
        BavaKamma,
        BavaMetzia,
        BavaBatra,
        Sanhedrin,
        Makkot,
        Shevuot,
        AvodahZarah,
        Horayot,
        Zevachim,
        Menachot,
        Chullin,
        Bekhorot,
        Arakhin,
        Temurah,
        Keritot,
        Meilah,
        Niddah,
    ]
};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Masechta {
    Berakhot,
    Shabbat,
    Eruvin,
    Pesachim,
    Shekalim,
    Yoma,
    Sukkah,
    Beitzah,
    RoshHashanah,
    Taanit,
    Megillah,
    MoedKatan,
    Chagigah,
    Yevamot,
    Ketubot,
    Nedarim,
    Nazir,
    Sotah,
    Gittin,
    Kiddushin,
    BavaKamma,
    BavaMetzia,
    BavaBatra,
    Sanhedrin,
    Makkot,
    Shevuot,
    AvodahZarah,
    Horayot,
    Zevachim,
    Menachot,
    Chullin,
    Bekhorot,
    Arakhin,
    Temurah,
    Keritot,
    Meilah,
    Niddah,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum DafYomiCalendar {
    First,
    Second,
}

fn max_daf(masechta: Masechta, cycle: DafYomiCalendar) -> u8 {
    match masechta {
        Masechta::Berakhot => 64,
        Masechta::Shabbat => 157,
        Masechta::Eruvin => 105,
        Masechta::Pesachim => 121,
        Masechta::Shekalim => {
            if cycle == DafYomiCalendar::First {
                13
            } else {
                22
            }
        }
        Masechta::Yoma => 88,
        Masechta::Sukkah => 56,
        Masechta::Beitzah => 40,
        Masechta::RoshHashanah => 35,
        Masechta::Taanit => 31,
        Masechta::Megillah => 32,
        Masechta::MoedKatan => 29,
        Masechta::Chagigah => 27,
        Masechta::Yevamot => 122,
        Masechta::Ketubot => 112,
        Masechta::Nedarim => 91,
        Masechta::Nazir => 66,
        Masechta::Sotah => 49,
        Masechta::Gittin => 90,
        Masechta::Kiddushin => 82,
        Masechta::BavaKamma => 119,
        Masechta::BavaMetzia => 119,
        Masechta::BavaBatra => 176,
        Masechta::Sanhedrin => 113,
        Masechta::Makkot => 24,
        Masechta::Shevuot => 49,
        Masechta::AvodahZarah => 76,
        Masechta::Horayot => 14,
        Masechta::Zevachim => 120,
        Masechta::Menachot => 110,
        Masechta::Chullin => 142,
        Masechta::Bekhorot => 61,
        Masechta::Arakhin => 34,
        Masechta::Temurah => 34,
        Masechta::Keritot => 28,
        Masechta::Meilah => 37,
        Masechta::Niddah => 73,
    }
}

#[test]
fn test_daf_yomi_day_1() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5684), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::Berakhot,
                        daf: 2,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5684).and_month_day(Month::Tishrei, 1)
    );
}
#[test]
fn test_daf_yomi_year_1() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5684), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::Pesachim,
                        daf: 62,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5684).and_month_day(Month::Elul, 29)
    );
}

#[test]
fn test_daf_yomi_year_2() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5685), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::MoedKatan,
                        daf: 9,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5685).and_month_day(Month::Elul, 29)
    );
}

#[test]
fn test_daf_yomi_cycle_2() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5691), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::Eruvin,
                        daf: 3,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5691).and_month_day(Month::Elul, 29)
    );
}

#[test]
fn test_daf_yomi_second_calendar_cycle_1() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5735), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::Shabbat,
                        daf: 12,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5735).and_month_day(Month::Elul, 29)
    );
}

#[test]
fn test_daf_yomi_second_calendar_cycle_later() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();

    get(&Year::new(5780), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::DafYomi(Daf {
                        masechta: Masechta::Eruvin,
                        daf: 40,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5780).and_month_day(Month::Elul, 29)
    );
}
