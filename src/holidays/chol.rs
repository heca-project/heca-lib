use super::{Holiday, Name};
use crate::{
    hebrew::{Month, Year},
    prelude::Day,
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;

pub(crate) fn get<S: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<S>>>>,
) {
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 30),
            name: Name::Chol(Chol::RoshChodeshCheshvan1),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Cheshvan, 1),
            name: Name::Chol(Chol::RoshChodeshCheshvan2),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Kislev, 25),
            name: Name::Chol(Chol::Chanukah1),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Kislev, 26),
            name: Name::Chol(Chol::Chanukah2),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Kislev, 27),
            name: Name::Chol(Chol::Chanukah3),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Kislev, 28),
            name: Name::Chol(Chol::Chanukah4),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Kislev, 29),
            name: Name::Chol(Chol::Chanukah5),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Shvat, 1),
            name: Name::Chol(Chol::RoshChodeshShvat),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Teves, 10),
            name: Name::Chol(Chol::TenTeves),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 1),
            name: Name::Chol(Chol::RoshChodeshNissan),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 30),
            name: Name::Chol(Chol::RoshChodeshIyar1),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 1),
            name: Name::Chol(Chol::RoshChodeshIyar2),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 1),
            name: Name::Chol(Chol::RoshChodeshSivan),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 30),
            name: Name::Chol(Chol::RoshChodeshTammuz1),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tammuz, 1),
            name: Name::Chol(Chol::RoshChodeshTammuz2),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Av, 1),
            name: Name::Chol(Chol::RoshChodeshAv),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Av, 30),
            name: Name::Chol(Chol::RoshChodeshElul1),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Elul, 1),
            name: Name::Chol(Chol::RoshChodeshElul2),
            candle_lighting: None,
            tzeis: None,
        }),
    ]);

    if year.sched[1] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Cheshvan, 30),
                name: Name::Chol(Chol::RoshChodeshKislev1),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Kislev, 1),
                name: Name::Chol(Chol::RoshChodeshKislev2),
                candle_lighting: None,
                tzeis: None,
            }),
        ]);
    } else {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Kislev, 1),
            name: Name::Chol(Chol::RoshChodeshKislev),
            candle_lighting: None,
            tzeis: None,
        })));
    }

    if year.sched[2] == 30 {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Kislev, 30),
                name: Name::Chol(Chol::RoshChodeshTeves1),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 1),
                name: Name::Chol(Chol::RoshChodeshTeves2),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Kislev, 30),
                name: Name::Chol(Chol::Chanukah6),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 1),
                name: Name::Chol(Chol::Chanukah7),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 2),
                name: Name::Chol(Chol::Chanukah8),
                candle_lighting: None,
                tzeis: None,
            }),
        ]);
    } else {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 1),
                name: Name::Chol(Chol::RoshChodeshTeves),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 1),
                name: Name::Chol(Chol::Chanukah6),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 2),
                name: Name::Chol(Chol::Chanukah7),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Teves, 3),
                name: Name::Chol(Chol::Chanukah8),
                candle_lighting: None,
                tzeis: None,
            }),
        ]);
    }

    if year.sched[5] != 0 {
        //If this is a regular year
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Shvat, 30),
                name: Name::Chol(Chol::RoshChodeshAdar1),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar, 1),
                name: Name::Chol(Chol::RoshChodeshAdar2),
                candle_lighting: None,
                tzeis: None,
            }),
            // Taanis Esther is on the 13th of Adar. However, If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday {
                    day: year.and_month_day(Month::Adar, 13),
                    name: Name::Chol(Chol::TaanisEsther),
                    candle_lighting: None,
                    tzeis: None,
                })
            } else {
                Some(Holiday {
                    day: year.and_month_day(Month::Adar, 11),
                    name: Name::Chol(Chol::TaanisEsther),
                    candle_lighting: None,
                    tzeis: None,
                })
            },
        ]);
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Adar, 14),
                name: Name::Chol(Chol::Purim),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar, 15),
                name: Name::Chol(Chol::ShushanPurim),
                candle_lighting: None,
                tzeis: None,
            }),
        ]);
    } else {
        //If this is a leap year

        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Shvat, 30),
                name: Name::Chol(Chol::RoshChodeshAdarRishon1),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                name: Name::Chol(Chol::RoshChodeshAdarRishon2),
                candle_lighting: None,
                tzeis: None,
                day: year.and_month_day(Month::Adar1, 1),
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar1, 30),
                name: Name::Chol(Chol::RoshChodeshAdarSheni1),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar2, 1),
                name: Name::Chol(Chol::RoshChodeshAdarSheni2),
                candle_lighting: None,
                tzeis: None,
            }),
            // If the next Rosh Hashana starts on a Thursday, the previous Purim starts on a
            // Sunday, and Taanis Esther needs to be brought back to Thursday.
            if year.day_of_next_rh != Day::Thursday {
                Some(Holiday {
                    day: year.and_month_day(Month::Adar2, 13),
                    name: Name::Chol(Chol::TaanisEsther),
                    candle_lighting: None,
                    tzeis: None,
                })
            } else {
                Some(Holiday {
                    day: year.and_month_day(Month::Adar2, 11),
                    name: Name::Chol(Chol::TaanisEsther),
                    candle_lighting: None,
                    tzeis: None,
                })
            },
            Some(Holiday {
                day: year.and_month_day(Month::Adar2, 14),
                name: Name::Chol(Chol::Purim),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar2, 15),
                name: Name::Chol(Chol::ShushanPurim),
                candle_lighting: None,
                tzeis: None,
            }),
        ]);
    }

    //17th of Tammuz is on Shabbos when the next Rosh Hashana starts on Monday (For example
    //5782/5783).
    return_vec.extend_from_slice(&if year.day_of_next_rh == Day::Monday {
        [
            Some(Holiday {
                day: year.and_month_day(Month::Tammuz, 18),
                name: Name::Chol(Chol::SeventeenTammuz),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Av, 10),
                name: Name::Chol(Chol::NineAv),
                candle_lighting: None,
                tzeis: None,
            }),
        ]
    } else {
        [
            Some(Holiday {
                day: year.and_month_day(Month::Tammuz, 17),
                name: Name::Chol(Chol::SeventeenTammuz),
                candle_lighting: None,
                tzeis: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Av, 9),
                name: Name::Chol(Chol::NineAv),
                candle_lighting: None,
                tzeis: None,
            }),
        ]
    });
    return_vec.extend(std::iter::once(if year.day_of_rh == Day::Thursday {
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 4),
            name: Name::Chol(Chol::TzomGedalia),
            candle_lighting: None,
            tzeis: None,
        })
    } else {
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 3),
            name: Name::Chol(Chol::TzomGedalia),
            candle_lighting: None,
            tzeis: None,
        })
    }))
}

/// Possible weekday Torah readings
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
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
