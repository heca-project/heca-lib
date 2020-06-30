use super::{Holiday, Name};
use crate::{
    hebrew::{Date, Month, Year},
    prelude::{Day, Location},
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;

#[inline]
pub(crate) fn get<S: Clone, T: Fn(Date) -> S, U: Fn(Date) -> S>(
    year: &Year,
    location: Location,
    array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<S>>>>,
    shkiya_func: Option<T>,
    tzeis_func: Option<U>,
) {
    let arr1 = [
        {
            let day = year.and_month_day(Month::Tishrei, 1);
            let candle_lighting = shkiya_func.as_ref().map(|ref x| x(day));
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));
            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::RoshHashanah1),
                candle_lighting,
                tzeis: tzeis,
            })
        },
        {
            let day = year.and_month_day(Month::Tishrei, 2);
            let candle_lighting = shkiya_func.as_ref().map(|ref x| x(day));
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));
            Some(Holiday {
                day: day,
                name: Name::YomTov(YomTov::RoshHashanah2),
                candle_lighting,
                tzeis,
            })
        },
        {
            let day = year.and_month_day(Month::Tishrei, 10);
            let candle_lighting = shkiya_func.as_ref().map(|ref x| x(day));
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::YomKippur),
                candle_lighting,
                tzeis,
            })
        },
        {
            let day = year.and_month_day(Month::Tishrei, 15);
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            let candle_lighting = if day.day_of_week() == Day::Sunday {
                tzeis_func.as_ref().map(|ref x| x(day))
            } else {
                shkiya_func.as_ref().map(|ref x| x(day))
            };
            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::Sukkos1),
                candle_lighting,
                tzeis,
            })
        },
        {
            let day = year.and_month_day(Month::Tishrei, 16);
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            let candle_lighting = if location == Location::Chul {
                if day.day_of_week() == Day::Shabbos {
                    shkiya_func.as_ref().map(|ref x| x(day))
                } else {
                    tzeis_func.as_ref().map(|ref x| x(day))
                }
            } else {
                None
            };

            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::Sukkos2),
                candle_lighting,
                tzeis,
            })
        },
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 17),
            name: Name::YomTov(YomTov::Sukkos3),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 18),
            name: Name::YomTov(YomTov::Sukkos4),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 19),
            name: Name::YomTov(YomTov::Sukkos5),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 20),
            name: Name::YomTov(YomTov::Sukkos6),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 21),
            name: Name::YomTov(YomTov::Sukkos7),
            candle_lighting: None,
            tzeis: None,
        }),
        {
            let day = year.and_month_day(Month::Tishrei, 22);
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            let candle_lighting = shkiya_func.as_ref().map(|ref x| x(day));
            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::ShminiAtzeres),
                candle_lighting,
                tzeis,
            })
        },
    ];
    array_vec.extend_from_slice(&arr1);
    if location == Location::Chul {
        let day = year.and_month_day(Month::Tishrei, 23);
        let candle_lighting = shkiya_func.as_ref().map(|ref x| x(day));
        let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

        array_vec.extend(std::iter::once(Some(Holiday {
            day,
            name: Name::YomTov(YomTov::SimchasTorah),
            candle_lighting,
            tzeis,
        })));
    }
    array_vec.extend_from_slice(&[
        {
            let day = year.and_month_day(Month::Nissan, 15);
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            let candle_lighting = if day.day_of_week() == Day::Sunday {
                tzeis_func.as_ref().map(|ref x| x(day))
            } else {
                shkiya_func.as_ref().map(|ref x| x(day))
            };
            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::Pesach1),
                candle_lighting,
                tzeis,
            })
        },
        {
            let day = year.and_month_day(Month::Nissan, 16);
            let candle_lighting = if location == Location::Chul {
                if day.day_of_week() == Day::Friday {
                    shkiya_func.as_ref().map(|ref x| x(day))
                } else {
                    tzeis_func.as_ref().map(|ref x| x(day))
                }
            } else {
                None
            };
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            Some(Holiday {
                day: year.and_month_day(Month::Nissan, 16),
                name: Name::YomTov(YomTov::Pesach2),
                candle_lighting: candle_lighting,
                tzeis,
            })
        },
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 17),
            name: Name::YomTov(YomTov::Pesach3),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 18),
            name: Name::YomTov(YomTov::Pesach4),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 19),
            name: Name::YomTov(YomTov::Pesach5),
            candle_lighting: None,
            tzeis: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 20),
            name: Name::YomTov(YomTov::Pesach6),
            candle_lighting: None,
            tzeis: None,
        }),
        {
            let day = year.and_month_day(Month::Nissan, 21);
            let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

            let candle_lighting = if day.day_of_week() == Day::Sunday {
                tzeis_func.as_ref().map(|ref x| x(day))
            } else {
                shkiya_func.as_ref().map(|ref x| x(day))
            };
            Some(Holiday {
                day,
                name: Name::YomTov(YomTov::Pesach7),
                candle_lighting,
                tzeis,
            })
        },
    ]);

    if location == Location::Chul {
        let day = year.and_month_day(Month::Nissan, 22);
        let candle_lighting = if day.day_of_week() == Day::Shabbos {
            shkiya_func.as_ref().map(|ref x| x(day))
        } else {
            tzeis_func.as_ref().map(|ref x| x(day))
        };
        array_vec.extend(std::iter::once(Some(Holiday {
            day,
            name: Name::YomTov(YomTov::Pesach8),
            candle_lighting: None,
            tzeis: None,
        })));
    }

    {
        let day = year.and_month_day(Month::Sivan, 6);
        let candle_lighting = if day.day_of_week() == Day::Sunday {
            tzeis_func.as_ref().map(|ref x| x(day))
        } else {
            shkiya_func.as_ref().map(|ref x| x(day))
        };
        let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

        array_vec.extend(std::iter::once(Some(Holiday {
            day,
            name: Name::YomTov(YomTov::Shavuos1),
            candle_lighting,
            tzeis,
        })));
    }

    if location == Location::Chul {
        let day = year.and_month_day(Month::Sivan, 7);
        let tzeis = tzeis_func.as_ref().map(|ref x| x(day));

        let candle_lighting = if day.day_of_week() == Day::Shabbos {
            shkiya_func.as_ref().map(|ref x| x(day))
        } else {
            tzeis_func.as_ref().map(|ref x| x(day))
        };
        array_vec.extend(std::iter::once(Some(Holiday {
            day,
            name: Name::YomTov(YomTov::Shavuos2),
            candle_lighting: candle_lighting,
            tzeis,
        })));
    }
}

/// Yom Tov, including Rosh Hashana, Yom Kippur and Chol HaMoed
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum YomTov {
    RoshHashanah1,
    RoshHashanah2,
    YomKippur,
    Sukkos1,
    Sukkos2,
    Sukkos3,
    Sukkos4,
    Sukkos5,
    Sukkos6,
    Sukkos7,
    ShminiAtzeres,
    SimchasTorah,
    Pesach1,
    Pesach2,
    Pesach3,
    Pesach4,
    Pesach5,
    Pesach6,
    Pesach7,
    Pesach8,
    Shavuos1,
    Shavuos2,
}
