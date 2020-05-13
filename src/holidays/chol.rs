use serde::{Deserialize, Serialize};

use super::{Holiday, Name};
use crate::{
    prelude::{Day, HebrewMonth},
    HebrewYear,
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;

pub(crate) fn get_chol_list(
    year: &HebrewYear,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshCheshvan1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshCheshvan2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(25).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(26).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(27).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah3),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(28).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah4),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(29).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::Chanukah5),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshShvat),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(10).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TenTeves),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshNissan),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshIyar1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Iyar, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshIyar2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshSivan),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshTammuz1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshTammuz2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshAv),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(30).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshElul1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Elul, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshElul2),
        }),
    ]);

    if year.sched[1] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Cheshvan, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshKislev1),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshKislev2),
            }),
        ]);
    } else {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::RoshChodeshKislev),
        })));
    }

    if year.sched[2] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves1),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves2),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Kislev, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah6),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah7),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(2).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah8),
            }),
        ]);
    } else {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshTeves),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah6),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(2).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah7),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Teves, NonZeroU8::new(3).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Chanukah8),
            }),
        ]);
    }

    if year.sched[5] != 0 {
        //If this is a regular year
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdar1),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdar2),
            }),
            // Taanis Esther is on the 13th of Adar. However, If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(13).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                })
            } else {
                Some(Holiday {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(11).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                })
            },
        ]);
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(14).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Purim),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar, NonZeroU8::new(15).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::ShushanPurim),
            }),
        ]);
    } else {
        //If this is a leap year

        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Shvat, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarRishon1),
            }),
            Some(Holiday {
                name: Name::Chol(Chol::RoshChodeshAdarRishon2),
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroU8::new(1).unwrap())
                    .unwrap(),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar1, NonZeroU8::new(30).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarSheni1),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(1).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::RoshChodeshAdarSheni2),
            }),
            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(13).unwrap())
                        .unwrap(),

                    name: Name::Chol(Chol::TaanisEsther),
                })
            } else {
                Some(Holiday {
                    day: year
                        .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(11).unwrap())
                        .unwrap(),
                    name: Name::Chol(Chol::TaanisEsther),
                })
            },
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(14).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::Purim),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Adar2, NonZeroU8::new(15).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::ShushanPurim),
            }),
        ]);
    }

    //17th of Tammuz is on Shabbos when the next Rosh Hashana starts on Monday (For example
    //5782/5783).
    return_vec.extend_from_slice(&if year.day_of_next_rh == Day::Monday {
        [
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(18).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::SeventeenTammuz),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(10).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::NineAv),
            }),
        ]
    } else {
        [
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Tammuz, NonZeroU8::new(17).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::SeventeenTammuz),
            }),
            Some(Holiday {
                day: year
                    .get_hebrew_date(HebrewMonth::Av, NonZeroU8::new(9).unwrap())
                    .unwrap(),
                name: Name::Chol(Chol::NineAv),
            }),
        ]
    });
    return_vec.extend(std::iter::once(if year.day_of_rh == Day::Thursday {
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(4).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TzomGedalia),
        })
    } else {
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(3).unwrap())
                .unwrap(),
            name: Name::Chol(Chol::TzomGedalia),
        })
    }))
}

/// Possible weekday Torah readings
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Chol {
    TzomGedalia,
    RoshChodeshCheshvan1,
    RoshChodeshCheshvan2,
    Chanukah1,
    Chanukah2,
    Chanukah3,
    Chanukah4,
    Chanukah5,
    Chanukah6,
    Chanukah7,
    Chanukah8,
    TenTeves,
    RoshChodeshShvat,
    RoshChodeshNissan,
    RoshChodeshIyar1,
    RoshChodeshIyar2,
    RoshChodeshSivan,
    RoshChodeshTammuz1,
    RoshChodeshTammuz2,
    RoshChodeshAv,
    RoshChodeshElul1,
    RoshChodeshElul2,
    RoshChodeshKislev1,
    RoshChodeshKislev2,
    RoshChodeshKislev,
    RoshChodeshTeves1,
    RoshChodeshTeves2,
    RoshChodeshTeves,
    RoshChodeshAdar1,
    RoshChodeshAdar2,
    TaanisEsther,
    Purim,
    ShushanPurim,
    RoshChodeshAdarRishon1,
    RoshChodeshAdarRishon2,
    RoshChodeshAdarSheni1,
    RoshChodeshAdarSheni2,
    SeventeenTammuz,
    NineAv,
}
