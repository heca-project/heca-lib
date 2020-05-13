use serde::{Deserialize, Serialize};

use super::{Holiday, Name};
use crate::{
    prelude::{HebrewMonth, Location},
    HebrewYear,
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;

#[inline]
pub(crate) fn get_yt_list(
    year: &HebrewYear,
    location: Location,
    array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday>>>,
) {
    array_vec.extend_from_slice(&[
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(1).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::RoshHashanah1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(2).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::RoshHashanah2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(10).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::YomKippur),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(15).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(16).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(17).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos3),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(18).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos4),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(19).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos5),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(20).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos6),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(21).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Sukkos7),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(22).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::ShminiAtzeres),
        }),
    ]);
    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Tishrei, NonZeroU8::new(23).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::SimchasTorah),
        })));
    }
    array_vec.extend_from_slice(&[
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(15).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach1),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(16).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach2),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(17).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach3),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(18).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach4),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(19).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach5),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(20).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach6),
        }),
        Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(21).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach7),
        }),
    ]);

    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Nissan, NonZeroU8::new(22).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Pesach8),
        })));
    }
    array_vec.extend(std::iter::once(Some(Holiday {
        day: year
            .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(6).unwrap())
            .unwrap(),
        name: Name::YomTov(YomTov::Shavuos1),
    })));

    if location == Location::Chul {
        array_vec.extend(std::iter::once(Some(Holiday {
            day: year
                .get_hebrew_date(HebrewMonth::Sivan, NonZeroU8::new(7).unwrap())
                .unwrap(),
            name: Name::YomTov(YomTov::Shavuos2),
        })));
    }
}

/// Yom Tov, including Rosh Hashana, Yom Kippur and Chol HaMoed
#[derive(Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
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
