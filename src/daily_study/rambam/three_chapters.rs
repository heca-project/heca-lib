use super::{max_halacha, Halacha, HALACHAS};
use crate::{
    hebrew::{Month, Year},
    holidays::DailyStudy,
    prelude::Name,
    secular::Duration,
};
use tinyvec::TinyVec;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Material(Chapter, Chapter, Chapter);

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Chapter {
    section: Halacha,
    chapter: u8,
}

pub(crate) fn get(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<DailyStudy>>>,
) {
    let first_day_of_rambam_yomi = Year::new(5744).and_month_day(Month::Nissan, 27);
    if year.year() == 5744 {
        let mut current_day = first_day_of_rambam_yomi.clone();
        let rosh_hashana_next_year = Year::new(5745).and_month_day(Month::Tishrei, 1);
        let mut i = 0;
        while (current_day) < (rosh_hashana_next_year) {
            return_vec.extend(std::iter::once(Some(DailyStudy {
                day: current_day,
                name: Name::RambamThreeChapters(Material(
                    get_rambam(
                        3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                    get_rambam(
                        1 + 3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                    get_rambam(
                        2 + 3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                )),
            })));
            current_day = current_day + Duration::days(1);
            i += 1;
        }
    } else if year.year() > 5744 {
        let mut current_day = year.and_month_day(Month::Tishrei, 1);
        let amount_of_days_since_first_day_of_rambam_yomi_until_rosh_hashana =
            current_day - first_day_of_rambam_yomi;
        let rosh_hashana_next_year = Year::new(year.year + 1).and_month_day(Month::Tishrei, 1);
        let mut i = 0;
        while (current_day) < (rosh_hashana_next_year) {
            return_vec.extend(std::iter::once(Some(DailyStudy {
                day: current_day,
                name: Name::RambamThreeChapters(Material(
                    get_rambam(
                        3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                    get_rambam(
                        1 + 3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                    get_rambam(
                        2 + 3 * (current_day - first_day_of_rambam_yomi).get_days().days() as u16,
                    ),
                )),
            })));
            current_day = current_day + Duration::days(1);
            i += 1;
        }
    }
}

fn get_rambam(first_day_of_rambam_yomi: u16) -> Chapter {
    let mut day = first_day_of_rambam_yomi % 1017;
    let mut section: Halacha;
    let mut index = 0;

    let chapter = loop {
        section = HALACHAS[index];
        if day < (max_halacha(HALACHAS[index]) as u16) {
            break day as u8;
        } else {
            day -= max_halacha(HALACHAS[index]) as u16;
            index += 1;
        }
    };
    Chapter {
        section,
        chapter: chapter + 1,
    }
}

#[test]
fn test_rambam_chapter_one_5780() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();

    get(&Year::new(5780), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::RambamThreeChapters(Material(
                        Chapter {
                            section: Halacha::Testimony,
                            chapter: 8,
                        },
                        Chapter {
                            section: Halacha::Testimony,
                            chapter: 9,
                        },
                        Chapter {
                            section: Halacha::Testimony,
                            chapter: 10,
                        },
                    ))
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5780).and_month_day(Month::Tammuz, 2)
    );
}

#[test]
fn test_rambam_chapter_three_5744() {
    let mut tiny_vec = tinyvec::TinyVec::<[Option<DailyStudy>; 32]>::new();

    get(&Year::new(5744), &mut tiny_vec);
    assert_eq!(
        tiny_vec
            .iter()
            .filter_map(|x| *x)
            .find(|x| {
                x.name()
                    == Name::RambamThreeChapters(Material(
                        Chapter {
                            section: Halacha::TransmissionOfTheOralLaw,
                            chapter: 1,
                        },
                        Chapter {
                            section: Halacha::TransmissionOfTheOralLaw,
                            chapter: 2,
                        },
                        Chapter {
                            section: Halacha::TransmissionOfTheOralLaw,
                            chapter: 3,
                        },
                    ))
            })
            .unwrap()
            .day(),
        crate::hebrew::Year::new(5744).and_month_day(Month::Nissan, 27)
    );
}
