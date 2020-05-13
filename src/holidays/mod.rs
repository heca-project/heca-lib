use crate::prelude::*;
pub(crate) mod chol;
pub(crate) mod shabbos;
pub(crate) mod special_parsha;
pub(crate) mod yom_tov;
use serde::*;

use std::cmp::Ordering;

use crate::convert::HebrewDate;
use shabbos::Parsha;

#[derive(Debug, Eq, Copy, Clone, Serialize)]
/// This struct holds a day on which the Torah is read.
///
/// You can get the Hebrew Date and the Torah reading.
pub struct Holiday {
    pub(crate) day: HebrewDate,
    pub(crate) name: Name,
}

impl Holiday {
    #[inline]
    pub fn day(&self) -> HebrewDate {
        self.day
    }

    #[inline]
    pub fn name(&self) -> Name {
        self.name
    }
}

impl PartialOrd for Holiday {
    #[inline]
    fn partial_cmp(&self, other: &Holiday) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Holiday {
    #[inline]
    fn cmp(&self, other: &Holiday) -> Ordering {
        self.day.cmp(&other.day)
    }
}

impl PartialEq for Holiday {
    #[inline]
    fn eq(&self, other: &Holiday) -> bool {
        self.day == other.day
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Name {
    YomTov(YomTov),
    Chol(Chol),
    Shabbos(Parsha),
    SpecialParsha(SpecialParsha),
}

/// Special Parshas read every winter

#[cfg(test)]

mod test {
    use crate::{holidays::*, HebrewYear};
    use chol::get_chol_list;
    use chrono::prelude::*;
    use shabbos::{get_shabbos_list, get_shabbosim};
    use std::num::NonZeroU8;
    use yom_tov::get_yt_list;

    #[test]
    fn purim_should_never_start_on_a_friday_night() {
        for i in 3764..9999 {
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            get_chol_list(&HebrewYear::new(i).unwrap(), &mut sum_vec);
            for day in sum_vec.iter() {
                if day.unwrap().name() == Name::Chol(Chol::Purim) {
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Weekday::Fri);
                }
            }
        }
    }
    #[test]
    fn fasts_should_never_start_on_friday_night() {
        for i in 3764..9999 {
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            get_chol_list(&HebrewYear::new(i).unwrap(), &mut sum_vec);
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
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Weekday::Fri);
                }
                //Taanis Esther can never be on a Friday night or on a Thursday night
                if day.unwrap().name() == Name::Chol(Chol::TaanisEsther) {
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Weekday::Fri);
                    assert_ne!(day.unwrap().day().to_gregorian().weekday(), Weekday::Thu);
                }
            }
        }
    }
    #[test]
    fn check_shekalim_on_shabbos_mevorchim_or_rosh_chodesh() {
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let y = HebrewYear::new(i).unwrap();
                let date = if let Ok(date) =
                    HebrewDate::from_ymd(i, HebrewMonth::Adar, NonZeroU8::new(1).unwrap())
                {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, NonZeroU8::new(1).unwrap()).unwrap()
                }
                .to_gregorian();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                get_yt_list(&y, *loc, &mut ignore_vec);
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                get_shabbos_list(&y, *loc, &ignore_vec, &mut sum_vec);
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
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let date = HebrewDate::from_ymd(i, HebrewMonth::Nissan, NonZeroU8::new(1).unwrap())
                    .unwrap()
                    .to_gregorian();
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                let y = HebrewYear::new(i).unwrap();
                get_yt_list(&y, *loc, &mut ignore_vec);
                get_shabbos_list(&y, *loc, &ignore_vec, &mut sum_vec);

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
        use chrono::Duration;
        for loc in [Location::Chul, Location::Israel].iter() {
            for i in 5764..9999 {
                let date = if let Ok(date) =
                    HebrewDate::from_ymd(i, HebrewMonth::Adar, NonZeroU8::new(14).unwrap())
                {
                    date
                } else {
                    HebrewDate::from_ymd(i, HebrewMonth::Adar2, NonZeroU8::new(14).unwrap())
                        .unwrap()
                }
                .to_gregorian();
                let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
                let y = HebrewYear::new(i).unwrap();
                get_yt_list(&y, *loc, &mut ignore_vec);
                get_shabbos_list(&y, *loc, &mut ignore_vec, &mut sum_vec);
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
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            let y = HebrewYear::new(i).unwrap();
            get_yt_list(&y, Location::Chul, &mut ignore_vec);
            get_shabbos_list(&y, Location::Chul, &mut ignore_vec, &mut sum_vec);

            assert_eq!(
                sum_vec
                    .iter()
                    .filter(|&x| (*x).unwrap().day().to_gregorian().weekday() != Weekday::Fri)
                    .count(),
                0
            );
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            let mut ignore_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();
            let y = HebrewYear::new(i).unwrap();
            get_yt_list(&y, Location::Chul, &mut ignore_vec);
            get_shabbos_list(&y, Location::Chul, &ignore_vec, &mut sum_vec);

            assert_eq!(
                sum_vec
                    .iter()
                    .filter(|&x| (*x).unwrap().day().to_gregorian().weekday() != Weekday::Fri)
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
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();

            get_yt_list(&HebrewYear::new(i).unwrap(), Location::Chul, &mut sum_vec);
            sum_vec.clear();
            println!("Getting eretz yt list");
            get_yt_list(&HebrewYear::new(i).unwrap(), Location::Israel, &mut sum_vec);
            sum_vec.clear();
            println!("Getting chol list");
            get_chol_list(&HebrewYear::new(i).unwrap(), &mut sum_vec);
        }
    }

    #[test]
    fn large_dates() {
        use std::convert::TryInto;
        for i in 200_000..210_000 {
            println!("{}", i);
            println!("Getting chul yt list");
            let mut sum_vec = tinyvec::TinyVec::<[Option<Holiday>; 32]>::new();

            get_yt_list(&HebrewYear::new(i).unwrap(), Location::Chul, &mut sum_vec);
            sum_vec.clear();
            println!("Getting eretz yt list");
            get_yt_list(&HebrewYear::new(i).unwrap(), Location::Israel, &mut sum_vec);
            sum_vec.clear();
            println!("Getting chol list");
            get_chol_list(&HebrewYear::new(i).unwrap(), &mut sum_vec);
            let _english: NaiveDateTime =
                HebrewDate::from_ymd(i, HebrewMonth::Elul, std::num::NonZeroU8::new(29).unwrap())
                    .unwrap()
                    .try_into()
                    .unwrap();
        }
    }

    #[test]
    fn get_shabbosim_fall_on_shabbos() {
        for i in 3764..9999 {
            get_shabbosim(
                &HebrewYear::new(i).unwrap(),
                &tinyvec::TinyVec::<[Option<Holiday>; 32]>::new(),
            )
            .iter()
            //Shabbos starts on _Friday_ night
            .for_each(|x| assert_eq!(x.to_gregorian().weekday(), Weekday::Fri));
        }
    }
}
