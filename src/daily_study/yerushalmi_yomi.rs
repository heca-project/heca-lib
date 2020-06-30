use crate::{
    hebrew::{Date, Month, Year},
    holidays::DailyStudy,
    prelude::Name,
    secular::Duration,
};
use tinyvec::TinyVec;

const MAX_DAF: u16 = 1554;

pub(crate) fn get(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<DailyStudy>>>,
) {
    if year.year() >= 5741 {
        let first_day_of_yerushalmi_yomi = Year::new(5740).and_month_day(Month::Shvat, 16);
        let amount_of_days_between_day_one_of_yerushalmi_yomi_and_rosh_hashana =
            year.and_month_day(Month::Tishrei, 1) - first_day_of_yerushalmi_yomi;
        let amount_of_yom_kippurs_and_tisha_beavs_between_day_one_and_current_rosh_hashana =
            1 + (year.year() - 5741) * 2;
        let amount_of_learning_days_between_first_day_of_yerushalmi_yomi_and_rosh_hashana =
            amount_of_days_between_day_one_of_yerushalmi_yomi_and_rosh_hashana
                .get_days()
                .days() as u32
                - amount_of_yom_kippurs_and_tisha_beavs_between_day_one_and_current_rosh_hashana;

        let mut current_day = year.and_month_day(Month::Tishrei, 1).clone();
        let rosh_hashana = year.and_month_day(Month::Tishrei, 1);

        let rosh_hashana_next_year = Year::new(year.year + 1).and_month_day(Month::Tishrei, 1);
        while (current_day) < (rosh_hashana_next_year) {
            let offset = if current_day > year.and_month_day(Month::Av, 9) {
                2
            } else if current_day > year.and_month_day(Month::Tishrei, 10) {
                1
            } else {
                0
            };
            if current_day != year.and_month_day(Month::Av, 9)
                && current_day != year.and_month_day(Month::Tishrei, 10)
            {
                let day = (current_day - rosh_hashana).get_days().days() as u32 - offset
                    + 1
                    + amount_of_learning_days_between_first_day_of_yerushalmi_yomi_and_rosh_hashana;
                return_vec.extend(std::iter::once(Some(DailyStudy {
                    day: current_day,
                    name: Name::YerushalmiYomi(get_daf_yomi_for_day_since_day_one_of_daf_yomi(
                        day % MAX_DAF as u32,
                    )),
                })));
            }
            current_day = current_day + Duration::days(1);
        }
    } else if year.year() == 5740 {
        let first_day_of_yerushalmi_yomi = Year::new(5740).and_month_day(Month::Shvat, 16);
        let mut current_day = first_day_of_yerushalmi_yomi.clone();
        let rosh_hashana_next_year = Year::new(5741).and_month_day(Month::Tishrei, 1);
        while (current_day) < (rosh_hashana_next_year) {
            let offset = if current_day > year.and_month_day(Month::Av, 9) {
                1
            } else {
                0
            };
            if current_day != year.and_month_day(Month::Av, 9) {
                let day = (current_day - first_day_of_yerushalmi_yomi)
                    .get_days()
                    .days() as u32
                    - offset
                    + 1;
                return_vec.extend(std::iter::once(Some(DailyStudy {
                    day: current_day,
                    name: Name::YerushalmiYomi(get_daf_yomi_for_day_since_day_one_of_daf_yomi(day)),
                })));
            }
            current_day = current_day + Duration::days(1);
        }
    }
}

fn get_daf_yomi_for_day_since_day_one_of_daf_yomi(day: u32) -> Daf {
    let mut day = day;
    let mut index = 0;
    let mut masechta: Masechta;

    let daf = loop {
        masechta = MASECHTAS[index];
        if day < (max_daf(MASECHTAS[index]) as u32) {
            break day as u8;
        } else {
            day -= max_daf(MASECHTAS[index]) as u32;
            index += 1;
        }
    };
    Daf {
        masechta,
        daf: daf + 1,
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Daf {
    masechta: Masechta,
    daf: u8,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Masechta {
    Berakhot,
    Peah,
    Demai,
    Kilayim,
    Sheviit,
    Terumot,
    Maasrot,
    MaaserSheni,
    Hallah,
    Orlah,
    Bikkurim,
    Shabbat,
    Eruvin,
    Pesachim,
    Beitzah,
    RoshHashanah,
    Yoma,
    Sukkah,
    Taanit,
    Shekalim,
    Megillah,
    Chagigah,
    MoedKattan,
    Yevamot,
    Ketubot,
    Sotah,
    Nedarim,
    Nazir,
    Gittin,
    Kiddushin,
    BavaKamma,
    BavaMetsia,
    BavaBatra,
    Shevuot,
    Makkot,
    Sanhedrin,
    AvodahZarah,
    Horayot,
    Niddah,
}

const MASECHTAS: [Masechta; 39] = {
    use Masechta::*;
    [
        Berakhot,
        Peah,
        Demai,
        Kilayim,
        Sheviit,
        Terumot,
        Maasrot,
        MaaserSheni,
        Hallah,
        Orlah,
        Bikkurim,
        Shabbat,
        Eruvin,
        Pesachim,
        Beitzah,
        RoshHashanah,
        Yoma,
        Sukkah,
        Taanit,
        Shekalim,
        Megillah,
        Chagigah,
        MoedKattan,
        Yevamot,
        Ketubot,
        Sotah,
        Nedarim,
        Nazir,
        Gittin,
        Kiddushin,
        BavaKamma,
        BavaMetsia,
        BavaBatra,
        Shevuot,
        Makkot,
        Sanhedrin,
        AvodahZarah,
        Horayot,
        Niddah,
    ]
};

fn max_daf(masechta: Masechta) -> u8 {
    match masechta {
        Masechta::Berakhot => 68,
        Masechta::Peah => 37,
        Masechta::Demai => 34,
        Masechta::Kilayim => 44,
        Masechta::Sheviit => 31,
        Masechta::Terumot => 59,
        Masechta::Maasrot => 26,
        Masechta::MaaserSheni => 33,
        Masechta::Hallah => 28,
        Masechta::Orlah => 20,
        Masechta::Bikkurim => 13,
        Masechta::Shabbat => 92,
        Masechta::Eruvin => 65,
        Masechta::Pesachim => 71,
        Masechta::Beitzah => 22,
        Masechta::RoshHashanah => 22,
        Masechta::Yoma => 42,
        Masechta::Sukkah => 26,
        Masechta::Taanit => 26,
        Masechta::Shekalim => 33,
        Masechta::Megillah => 34,
        Masechta::Chagigah => 22,
        Masechta::MoedKattan => 19,
        Masechta::Yevamot => 85,
        Masechta::Ketubot => 72,
        Masechta::Sotah => 47,
        Masechta::Nedarim => 40,
        Masechta::Nazir => 47,
        Masechta::Gittin => 54,
        Masechta::Kiddushin => 48,
        Masechta::BavaKamma => 44,
        Masechta::BavaMetsia => 37,
        Masechta::BavaBatra => 34,
        Masechta::Shevuot => 44,
        Masechta::Makkot => 9,
        Masechta::Sanhedrin => 57,
        Masechta::AvodahZarah => 37,
        Masechta::Horayot => 19,
        Masechta::Niddah => 13,
    }
}

#[test]
fn get_yerushalmi_of_before_yom_kippur_5741() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5741), &mut tiny_vec);
    //panic!("{:?}",tiny_vec.into_iter().filter_map(|x|x).map(|x|x.name()).collect::<Vec<Name>>());
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::YerushalmiYomi(Daf {
                        masechta: Masechta::Terumot,
                        daf: 16,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5741).and_month_day(Month::Tishrei, 9)
    );
}

#[test]
fn get_yerushalmi_of_day_after_yom_kippur_5741() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5741), &mut tiny_vec);
    //panic!("{:?}",tiny_vec.into_iter().filter_map(|x|x).map(|x|x.name()).collect::<Vec<Name>>());
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::YerushalmiYomi(Daf {
                        masechta: Masechta::Terumot,
                        daf: 17,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5741).and_month_day(Month::Tishrei, 11)
    );
}

#[test]
fn get_yerushalmi_of_last_day_of_year_5740() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5740), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::YerushalmiYomi(Daf {
                        masechta: Masechta::Terumot,
                        daf: 7,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5740).and_month_day(Month::Elul, 29)
    );
}
#[test]
fn get_yerushalmi_of_rosh_hashana_5779() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5779), &mut tiny_vec);

    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::YerushalmiYomi(Daf {
                        masechta: Masechta::Berakhot,
                        daf: 38,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5779).and_month_day(Month::Tishrei, 1)
    );
}

#[test]
fn get_yerushalmi_of_rosh_hashana_5780() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
    get(&Year::new(5780), &mut tiny_vec);

    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::YerushalmiYomi(Daf {
                        masechta: Masechta::Shabbat,
                        daf: 28,
                    })
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5780).and_month_day(Month::Tishrei, 1)
    );
}

#[test]
fn check_no_yerushalmi_9th_av() {
    for i in 5740..5800 {
        let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
        get(&Year::new(i), &mut tiny_vec);

        assert!(tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| { x.day() == Year::new(i).and_month_day(Month::Av, 9) })
            .is_none());
    }
}

#[test]
fn check_no_yerushalmi_yom_kippur() {
    for i in 5740..5800 {
        let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();
        get(&Year::new(i), &mut tiny_vec);

        assert!(tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| { x.day() == Year::new(i).and_month_day(Month::Tishrei, 10) })
            .is_none());
    }
}
