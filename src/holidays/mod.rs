use crate::prelude::*;
pub(crate) mod chabad_holidays;
pub(crate) mod chol;
pub(crate) mod israeli_holidays;
pub(crate) mod minor_holiday;
pub(crate) mod omer;
pub(crate) mod shabbos;
pub(crate) mod shabbos_mevarchim;
pub(crate) mod special_parsha;
pub(crate) mod yom_tov;
use std::cmp::Ordering;

use crate::hebrew::Date;
use crate::hebrew::Month;
use chabad_holidays::ChabadHoliday;
use israeli_holidays::IsraeliHoliday;
use minor_holiday::MinorHoliday;
use omer::Omer;
use shabbos::Parsha;

#[derive(Debug, Copy, Clone)]
/// This struct holds a day on which the Torah is read.
///
/// You can get the Hebrew Date and the Torah reading.
pub struct Holiday<T: Clone> {
    pub(crate) day: Date,
    pub(crate) name: Name,
    pub(crate) candle_lighting: Option<T>,
    pub(crate) tzeis: Option<T>,
}

#[derive(Debug, Copy, Clone)]
pub struct DailyStudy {
    pub(crate) day: Date,
    pub(crate) name: Name,
}

impl DailyStudy {
    #[inline]
    pub fn day(&self) -> Date {
        self.day
    }

    #[inline]
    pub fn name(&self) -> Name {
        self.name
    }
}

impl<T: Clone> Holiday<T> {
    #[inline]
    pub fn day(&self) -> Date {
        self.day
    }

    #[inline]
    pub fn name(&self) -> Name {
        self.name
    }

    #[inline]
    pub fn candle_lighting(&self) -> Option<T> {
        self.candle_lighting.clone()
    }
}

impl<T: Clone> PartialOrd for Holiday<T> {
    #[inline]
    fn partial_cmp(&self, other: &Holiday<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone> Ord for Holiday<T> {
    #[inline]
    fn cmp(&self, other: &Holiday<T>) -> Ordering {
        self.day.cmp(&other.day)
    }
}

impl<T: Clone> Eq for Holiday<T> {}

impl<T: Clone> PartialEq for Holiday<T> {
    #[inline]
    fn eq(&self, other: &Holiday<T>) -> bool {
        self.day == other.day
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Name {
    YomTov(YomTov),
    Chol(Chol),
    Shabbos(Parsha),
    SpecialParsha(SpecialParsha),
    Omer(Omer),
    MinorHoliday(MinorHoliday),
    ShabbosMevarchim { hebrew_month: Month, molad: Molad },
    IsraeliHoliday(IsraeliHoliday),
    ChabadHoliday(ChabadHoliday),
    DafYomi(crate::daily_study::daf_yomi::Daf),
    YerushalmiYomi(crate::daily_study::yerushalmi_yomi::Daf),
    RambamThreeChapters(crate::daily_study::rambam::three_chapters::Material),
    RambamOneChapter(crate::daily_study::rambam::one_chapter::Material),
}

/// Special Parshas read every winter

#[cfg(test)]

mod test {
    use crate::{
        hebrew::{Month, Year},
        holidays::*,
    };
    use std::num::NonZeroU8;

    #[test]
    fn purim_should_never_start_on_a_friday_night() {
        use crate::prelude::Day;
        for i in 3764..9999 {
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            chol::get(&Year::new(i), &mut sum_vec);
            for day in sum_vec.iter() {
                if day.unwrap().name() == Name::Chol(Chol::Purim) {
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Day::Friday);
                }
            }
        }
    }
    #[test]
    fn fasts_should_never_start_on_friday_night() {
        for i in 3764..9999 {
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            chol::get(&Year::new(i), &mut sum_vec);
            for day in sum_vec.iter() {
                if day.unwrap().name() == Name::Chol(Chol::TzomGedalia)
                    || day.unwrap().name() == Name::Chol(Chol::TenTeves)
                    || day.unwrap().name() == Name::Chol(Chol::SeventeenTammuz)
                    || day.unwrap().name() == Name::Chol(Chol::NineAv)
                {
                    println!(
                        "{:?}  {:?} {:?}",
                        day.unwrap().name(),
                        day.unwrap().day(),
                        day.unwrap().day().to_gregorian().weekday()
                    );
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Day::Friday);
                }
                //Taanis Esther can never be on a Friday night or on a Thursday night
                if day.unwrap().name() == Name::Chol(Chol::TaanisEsther) {
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Day::Friday);
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Day::Thursday);
                }
            }
        }
    }
    #[test]
    fn check_shekalim_on_shabbos_mevorchim_or_rosh_chodesh() {
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let y = Year::new(i);
                let date = if let Ok(date) = Date::from_ymd_internal(Month::Adar, 1, y) {
                    date
                } else {
                    Date::from_ymd(i, Month::Adar2, 1)
                }
                .to_gregorian();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                yom_tov::get(
                    &y,
                    *loc,
                    &mut ignore_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                shabbos::get(
                    &y,
                    *loc,
                    &ignore_vec,
                    &mut sum_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );
                assert_eq!(
                    sum_vec
                        .iter()
                        .filter(
                            |x| x.unwrap().name() == Name::SpecialParsha(SpecialParsha::Shekalim)
                        )
                        .filter(|x| x.unwrap().day().to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }
    #[test]
    fn check_hachodesh_on_shabbos_mevorchim_or_rosh_chodesh() {
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let date = Date::from_ymd(i, Month::Nissan, 1).to_gregorian();
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                let y = Year::new(i);
                yom_tov::get(
                    &y,
                    *loc,
                    &mut ignore_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );
                shabbos::get(
                    &y,
                    *loc,
                    &ignore_vec,
                    &mut sum_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );

                assert_eq!(
                    sum_vec
                        .iter()
                        .filter(
                            |x| x.unwrap().name() == Name::SpecialParsha(SpecialParsha::HaChodesh)
                        )
                        .filter(|x| x.unwrap().day().to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }

    #[test]
    fn check_zachor_on_shabbos_before_purim() {
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let date = if let Ok(date) = Date::from_ymd_internal(Month::Adar, 14, Year::new(i))
                {
                    date
                } else {
                    Date::from_ymd(i, Month::Adar2, 14)
                }
                .to_gregorian();
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
                let y = Year::new(i);
                yom_tov::get(
                    &y,
                    *loc,
                    &mut ignore_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );
                shabbos::get(
                    &y,
                    *loc,
                    &mut ignore_vec,
                    &mut sum_vec,
                    None::<fn(Date) -> bool>,
                    None::<fn(Date) -> bool>,
                );
                assert_eq!(
                    sum_vec
                        .iter()
                        .filter(|x| x.unwrap().name() == Name::SpecialParsha(SpecialParsha::Zachor))
                        .filter(|x| x.unwrap().day().to_gregorian() - date > Duration::days(7))
                        .count(),
                    0
                );
            }
        }
    }
    #[test]
    fn check_all_shabbosim_and_torah_readings_are_on_shabbos() {
        for i in 5764..9999 {
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            let y = Year::new(i);
            yom_tov::get(
                &y,
                Location::Chul,
                &mut ignore_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            shabbos::get(
                &y,
                Location::Chul,
                &mut ignore_vec,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );

            assert_eq!(
                sum_vec
                    .iter()
                    .filter(|&x| (*x).unwrap().day().to_gregorian().weekday() != Day::Friday)
                    .count(),
                0
            );
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();
            let y = Year::new(i);
            yom_tov::get(
                &y,
                Location::Chul,
                &mut ignore_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            shabbos::get(
                &y,
                Location::Chul,
                &ignore_vec,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );

            assert_eq!(
                sum_vec
                    .iter()
                    .filter(|&x| (*x).unwrap().day().to_gregorian().weekday() != Day::Friday)
                    .count(),
                0
            );
        }
    }

    #[test]
    fn check_fns_work_without_panic() {
        for i in 5764..9999 {
            println!("{}", i);
            println!("Getting chul yt list");
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();

            yom_tov::get(
                &Year::new(i),
                Location::Chul,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            sum_vec.clear();
            println!("Getting eretz yt list");
            yom_tov::get(
                &Year::new(i),
                Location::Israel,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            sum_vec.clear();
            println!("Getting chol list");
            chol::get(&Year::new(i), &mut sum_vec);
        }
    }

    #[test]
    fn large_dates() {
        use std::convert::TryInto;
        for i in 200_000..210_000 {
            println!("{}", i);
            println!("Getting chul yt list");
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday<bool>>; 32]>::new();

            yom_tov::get(
                &Year::new(i),
                Location::Chul,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            sum_vec.clear();
            println!("Getting eretz yt list");
            yom_tov::get(
                &Year::new(i),
                Location::Israel,
                &mut sum_vec,
                None::<fn(Date) -> bool>,
                None::<fn(Date) -> bool>,
            );
            sum_vec.clear();
            println!("Getting chol list");
            chol::get(&Year::new(i), &mut sum_vec);
            let _english: crate::secular::Date =
                Date::from_ymd(i, Month::Elul, 29).try_into().unwrap();
        }
    }
}
