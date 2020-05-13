use crate::prelude::*;
use crate::{HebrewDate, HebrewYear};
use std::num::NonZeroU8;
use tinyvec::TinyVec;
use Parsha::Haazinu;

#[inline]
pub(crate) fn get_yt_list(
    year: &HebrewYear,
    location: Location,
    array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    array_vec.extend_from_slice(&[
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::RoshHashanah1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(2).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::RoshHashanah2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(10).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::YomKippur),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(15).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(16).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(17).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos3),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(18).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos4),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(19).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos5),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(20).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos6),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(21).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos7),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(22).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::ShminiAtzeres),
        })),
    ]);
    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday::TorahReadingDay(
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(23).unwrap())
                    .unwrap(),
                name: Name::YomTov(YomTov::SimchasTorah),
            },
        ))));
    }
    array_vec.extend_from_slice(&[
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(15).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(16).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(17).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach3),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(18).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach4),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(19).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach5),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(20).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach6),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(21).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach7),
        })),
    ]);

    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday::TorahReadingDay(
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(22).unwrap())
                    .unwrap(),
                name: Name::YomTov(YomTov::Pesach8),
            },
        ))));
    }
    array_vec.extend(std::iter::once(Some(Holiday::TorahReadingDay(
        TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(6).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Shavuos1),
        },
    ))));

    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday::TorahReadingDay(
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(7).unwrap())
                    .unwrap(),
                name: Name::YomTov(YomTov::Shavuos2),
            },
        ))));
    }
}

pub(crate) fn get_chol_list(
    year: &HebrewYear,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    return_vec.extend_from_slice(&[
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshCheshvan1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshCheshvan2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(25).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(26).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(27).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah3),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(28).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah4),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(29).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah5),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshShvat),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(10).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TenTeves),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshNissan),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshIyar1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshIyar2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshSivan),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshTammuz1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshTammuz2),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshAv),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshElul1),
        })),
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshElul2),
        })),
    ]);

    if year.sched[1] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshKislev1),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshKislev2),
            })),
        ]);
    } else {
        return_vec.extend(std::iter::once(Some(Holiday::TorahReadingDay(
            TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshKislev),
            },
        ))));
    }

    if year.sched[2] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves1),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves2),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah6),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah7),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(2).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah8),
            })),
        ]);
    } else {
        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah6),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(2).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah7),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(3).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah8),
            })),
        ]);
    }

    if year.sched[5] != 0 {
        //If this is a regular year
        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdar1),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdar2),
            })),
            // Taanis Esther is on the 13th of Adar. However, If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday::TorahReadingDay(TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(13).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                }))
            } else {
                Some(Holiday::TorahReadingDay(TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(11).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                }))
            },
        ]);
        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(14).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Purim),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(15).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::ShushanPurim),
            })),
        ]);
    } else {
        //If this is a leap year

        return_vec.extend_from_slice(&[
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarRishon1),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                name: Name::Chol(Chol::RoshChodeshAdarRishon2),
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroU8::new(1).unwrap())
                    .unwrap(),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarSheni1),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarSheni2),
            })),
            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday::TorahReadingDay(TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(13).unwrap())
                        .unwrap(),

                    name: Name::Chol(Chol::TaanisEsther),
                }))
            } else {
                Some(Holiday::TorahReadingDay(TorahReadingDay {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(11).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                }))
            },
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(14).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Purim),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(15).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::ShushanPurim),
            })),
        ]);
    }

    //17th of Tammuz is on Shabbos when the next Rosh Hashana starts on Monday (For example
    //5782/5783).
    return_vec.extend_from_slice(&if year.day_of_next_rh == Day::Monday {
        [
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(18).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::SeventeenTammuz),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(10).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::NineAv),
            })),
        ]
    } else {
        [
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(17).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::SeventeenTammuz),
            })),
            Some(Holiday::TorahReadingDay(TorahReadingDay {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(9).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::NineAv),
            })),
        ]
    });
    return_vec.extend(std::iter::once(if year.day_of_rh == Day::Thursday {
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(4).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TzomGedalia),
        }))
    } else {
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(3).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TzomGedalia),
        }))
    }));
}

/// This is based on the Biyur Halacha to Orach Chaim 428:4:3
pub(crate) fn get_shabbos_list(
    year: &HebrewYear,
    location: Location,
    ignore_dates: &TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    let rh_dow = year.day_of_rh;
    let rh_dow_next = year.day_of_next_rh;
    let year_len = year.year_len;

    //Tazriya/Metzorah Acharei-Mos/Kedoshim and Behar/Bechukosai are always split on a leap year
    //and connected on a regular year. The only exception is (in Israel) that Behar is split when the year is a non-leap year, is regular ordered and Rosh Hashana is on Thursday
    let (split_tazriya, split_acharei, mut split_behar) = if year_len > 365 {
        (true, true, true)
    } else {
        (false, false, false)
    };
    if location == Location::Israel {
        split_behar = split_behar || year_len == 354 && rh_dow == Day::Thursday;
    }

    //Vayakhel/Pekudei is split if the year is a leap year or if it's a full year and Rosh Hashana
    //starts on Thursday.
    let split_vayakhel = year_len > 365 || (year_len == 355 && rh_dow == Day::Thursday);

    //Chukas Balak is split when the second day of Shavuos doesn't fall on Shabbos (The first day can't fall out on Shabbos, as then the next Rosh Hashana would start on Friday, which it can't). Shavuos falls on Shabbos (5783, for example) when the first day of the next Rosh Hashana is on a Shabbos.
    //Obviously, in Israel it's never split (as they don't have the second day of Shavuos).
    let split_chukas = location == Location::Israel || rh_dow_next != Day::Shabbos;
    //Mattos/Maasei is split only if it's a leap year and Rosh Hashana starts on a Thursday, and
    //the year is full, or empty.
    //In Israel, It's also split in a leap year which starts on a Monday and is full, or a
    //leap year starting on a Tuesday, and the year is an ordered year.
    //See this for more information: https://he.wikipedia.org/wiki/%D7%A4%D7%A8%D7%A9%D7%AA_%D7%9E%D7%98%D7%95%D7%AA

    let split_mattos = rh_dow == Day::Thursday && (year_len == 383 || year_len == 385)
        || (location == Location::Israel
            && (rh_dow == Day::Monday && year_len == 385
                || rh_dow == Day::Tuesday && year_len == 384));
    //
    //Nitzavim/Vayelech is split only if Rosh Hashana starts on a Monday or Tuesday
    let split_nitzavim = rh_dow == Day::Monday || rh_dow == Day::Tuesday;
    let split_nitzavim_next_year = rh_dow_next == Day::Monday || rh_dow_next == Day::Tuesday;
    let regular_shabbosim_list = get_shabbosim(year, &ignore_dates);
    let mut parsha_vec = Vec::with_capacity(60);

    if split_nitzavim {
        parsha_vec.extend(std::iter::once(Parsha::Vayelech));
    };
    parsha_vec.extend_from_slice(&HAAZINU_KI_SISA);

    if split_vayakhel {
        parsha_vec.extend_from_slice(&[Parsha::Vayakhel, Parsha::Pikudei]);
    } else {
        parsha_vec.push(Parsha::VayakhelPikudei);
    }
    parsha_vec.extend_from_slice(&VAYIKRA_SHMINI);
    if split_tazriya {
        parsha_vec.push(Parsha::Tazriya);
        parsha_vec.push(Parsha::Metzorah);
    } else {
        parsha_vec.push(Parsha::TazriyaMetzorah);
    }
    if split_acharei {
        parsha_vec.push(Parsha::AchareiMos);
        parsha_vec.push(Parsha::Kedoshim);
    } else {
        parsha_vec.push(Parsha::AchareiMosKedoshim);
    }
    parsha_vec.extend_from_slice(&EMOR);
    if split_behar {
        parsha_vec.push(Parsha::Behar);
        parsha_vec.push(Parsha::Bechukosai);
    } else {
        parsha_vec.push(Parsha::BeharBechukosai);
    }
    parsha_vec.extend_from_slice(&BAMIDBAR_KORACH);
    if split_chukas {
        parsha_vec.push(Parsha::Chukas);
        parsha_vec.push(Parsha::Balak);
    } else {
        parsha_vec.push(Parsha::ChukasBalak);
    }

    parsha_vec.extend_from_slice(&PINCHAS);
    if split_mattos {
        parsha_vec.push(Parsha::Matos);
        parsha_vec.push(Parsha::Maasei);
    } else {
        parsha_vec.push(Parsha::MatosMaasei);
    }

    parsha_vec.extend_from_slice(&DEVARIM_KISAVO);
    if split_nitzavim_next_year {
        parsha_vec.push(Parsha::Nitzavim);
    } else {
        parsha_vec.push(Parsha::NitzavimVayelech);
    }
    eprintln!(
        "{}, {:?}, {}",
        regular_shabbosim_list
            .iter()
            .map(|x| format!("{:?}/{}/{}", x.month(), x.day(), x.year()))
            .fold(String::new(), |old, new| format!("{},{}", old, new)),
        ignore_dates,
        year.year()
    );
    return_vec.extend(regular_shabbosim_list.iter().enumerate().map(|(i, &v)| {
        Some(Holiday::TorahReadingDay(TorahReadingDay {
            name: Name::Shabbos(parsha_vec[i]),
            day: v,
        }))
    }))
}

pub(crate) fn get_special_parsha_list(
    year: &HebrewYear,
    array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    let rh_dow_next = year.day_of_next_rh;
    let len_of_year = year.year_len;

    let shekalim = TorahReadingDay {
        //Parshas Shekalim is the Shabbos before, or the Shabbos of the second day of Rosh Chodesh Adar (or the second day of Rosh Chodesh Adar Beis).
        // The first day of Rosh Chodesh Adar II can never be on Shabbos, as Purim would then
        // be on Sunday, and then the next Rosh Hashana would start on a Wednesday, breaking Lo
        // Adu Rosh.
        //
        //If Rosh Chodesh Adar (Beis) is on Shabbos (like in the year 5805), the next Rosh Hashana starts on a Tuesday
        day: if rh_dow_next == Day::Tuesday {
            if len_of_year < 360 {
                // This is a regular year
                year.get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(1).unwrap())
                    .unwrap()
            } else {
                // This is a leap year
                year.get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(1).unwrap())
                    .unwrap()
            }
        } else {
            let month = if len_of_year < 360 {
                HebrewMonth::Shvat
            } else {
                HebrewMonth::Adar1
            };
            // This is a regular year
            year.get_hebrew_date(
                month,
                NonZeroU8::new(match rh_dow_next {
                    Day::Monday => 25,
                    Day::Thursday => 29,
                    Day::Shabbos => 27,
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                })
                .unwrap(),
            )
            .unwrap()
        },
        name: Name::SpecialParsha(SpecialParsha::Shekalim),
    };
    let zachor = TorahReadingDay {
        //Parshas Zachor is read on the Shabbos before Purim.
        day: {
            let month = if len_of_year < 360 {
                HebrewMonth::Adar
            } else {
                HebrewMonth::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 9,    //For example, 5782
                Day::Tuesday => 8,   //For example, 5781
                Day::Thursday => 13, // For example, 5784
                Day::Shabbos => 11,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            year.get_hebrew_date(month, NonZeroU8::new(day).unwrap())
                .unwrap()
        },
        name: Name::SpecialParsha(SpecialParsha::Zachor),
    };
    let parah = TorahReadingDay {
        //Parshas Parah is read on the Shabbos before Hachodesh.
        day: {
            let month = if len_of_year < 360 {
                HebrewMonth::Adar
            } else {
                HebrewMonth::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 23,   //For example, 5782
                Day::Tuesday => 22,  //For example, 5781
                Day::Thursday => 20, // For example, 5784
                Day::Shabbos => 18,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            year.get_hebrew_date(month, NonZeroU8::new(day).unwrap())
                .unwrap()
        },
        name: Name::SpecialParsha(SpecialParsha::Parah),
    };
    let hachodesh = TorahReadingDay {
        //Parshas Hachodesh is read on the Shabbos before Rosh Chodesh Nissan, or on Rosh Chodesh
        //Nissan itself.
        day: {
            if rh_dow_next == Day::Monday {
                //Hachodesh is read on Rosh Chodesh Nissan itself
                year.get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(1).unwrap())
                    .unwrap()
            } else {
                let month = if len_of_year < 360 {
                    HebrewMonth::Adar
                } else {
                    HebrewMonth::Adar2
                };
                let day = match rh_dow_next {
                    Day::Tuesday => 29,  //For example, 5781
                    Day::Thursday => 27, // For example, 5784
                    Day::Shabbos => 25,  //For example, 5780
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                };
                year.get_hebrew_date(month, NonZeroU8::new(day).unwrap())
                    .unwrap()
            }
        },
        name: Name::SpecialParsha(SpecialParsha::HaChodesh),
    };

    array_vec.extend_from_slice(&[
        Some(Holiday::TorahReadingDay(shekalim)),
        Some(Holiday::TorahReadingDay(zachor)),
        Some(Holiday::TorahReadingDay(parah)),
        Some(Holiday::TorahReadingDay(hachodesh)),
    ]);
}

pub(crate) fn get_shabbosim(
    year: &HebrewYear,
    ignore_dates: &TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) -> Vec<HebrewDate> {
    let amnt_days_to_shabbos = Day::Shabbos as u32 - (year.day_of_rh as u32);
    let mut cur_day = year.days_since_epoch + amnt_days_to_shabbos;
    let mut return_regular_shabbosim: Vec<HebrewDate> = Vec::with_capacity(60);
    while cur_day < year.days_since_epoch + year.year_len as u32 {
        let day = year.get_hebrewdate_from_days_after_rh(cur_day);
        if ignore_dates
            .iter()
            .filter(|x| {
                if x.is_some() {
                    x.unwrap().day() == day
                } else {
                    false
                }
            })
            .count()
            == 0
        {
            return_regular_shabbosim.push(day);
        }

        cur_day += 7;
    }
    return_regular_shabbosim
}

const HAAZINU_KI_SISA: [Parsha; 22] = [
    Haazinu,
    Parsha::Bereishis,
    Parsha::Noach,
    Parsha::LechLecha,
    Parsha::Vayeira,
    Parsha::ChayeiSara,
    Parsha::Toldos,
    Parsha::Vayetzei,
    Parsha::Vayishlach,
    Parsha::Vayeshev,
    Parsha::Miketz,
    Parsha::Vayigash,
    Parsha::Vayechi,
    Parsha::Shemos,
    Parsha::Vaeira,
    Parsha::Bo,
    Parsha::Beshalach,
    Parsha::Yisro,
    Parsha::Mishpatim,
    Parsha::Terumah,
    Parsha::Tetzaveh,
    Parsha::KiSisa,
];
const VAYIKRA_SHMINI: [Parsha; 3] = [Parsha::Vayikra, Parsha::Tzav, Parsha::Shemini];
const EMOR: [Parsha; 1] = [Parsha::Emor];
const BAMIDBAR_KORACH: [Parsha; 5] = [
    Parsha::Bamidbar,
    Parsha::Naso,
    Parsha::Behaaloscha,
    Parsha::Shlach,
    Parsha::Korach,
];
const PINCHAS: [Parsha; 1] = [Parsha::Pinchas];
const DEVARIM_KISAVO: [Parsha; 7] = [
    Parsha::Devarim,
    Parsha::Vaeschanan,
    Parsha::Eikev,
    Parsha::Reeh,
    Parsha::Shoftim,
    Parsha::KiSeitzei,
    Parsha::KiSavoh,
];

#[cfg(test)]
mod test {
    use crate::holidays::*;
    use chrono::prelude::*;
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
            let english: NaiveDateTime =
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
