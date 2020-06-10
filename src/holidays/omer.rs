use super::{Holiday, Name};
use crate::hebrew::{Month, Year};
use std::num::NonZeroU8;
use tinyvec::TinyVec;
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Omer {
    Omer1,
    Omer2,
    Omer3,
    Omer4,
    Omer5,
    Omer6,
    Omer7,
    Omer8,
    Omer9,
    Omer10,
    Omer11,
    Omer12,
    Omer13,
    Omer14,
    Omer15,
    Omer16,
    Omer17,
    Omer18,
    Omer19,
    Omer20,
    Omer21,
    Omer22,
    Omer23,
    Omer24,
    Omer25,
    Omer26,
    Omer27,
    Omer28,
    Omer29,
    Omer30,
    Omer31,
    Omer32,
    Omer33,
    Omer34,
    Omer35,
    Omer36,
    Omer37,
    Omer38,
    Omer39,
    Omer40,
    Omer41,
    Omer42,
    Omer43,
    Omer44,
    Omer45,
    Omer46,
    Omer47,
    Omer48,
    Omer49,
}

pub(crate) fn get<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    //generated from: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e61202134e8dac71fdd7f86f35bcb7f0
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 16),
            name: Name::Omer(Omer::Omer1),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 17),
            name: Name::Omer(Omer::Omer2),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 18),
            name: Name::Omer(Omer::Omer3),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 19),
            name: Name::Omer(Omer::Omer4),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 20),
            name: Name::Omer(Omer::Omer5),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 21),
            name: Name::Omer(Omer::Omer6),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 22),
            name: Name::Omer(Omer::Omer7),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 23),
            name: Name::Omer(Omer::Omer8),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 24),
            name: Name::Omer(Omer::Omer9),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 25),
            name: Name::Omer(Omer::Omer10),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 26),
            name: Name::Omer(Omer::Omer11),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 27),
            name: Name::Omer(Omer::Omer12),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 28),
            name: Name::Omer(Omer::Omer13),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 29),
            name: Name::Omer(Omer::Omer14),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 1),
            name: Name::Omer(Omer::Omer15),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 2),
            name: Name::Omer(Omer::Omer16),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 3),
            name: Name::Omer(Omer::Omer17),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 4),
            name: Name::Omer(Omer::Omer18),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 5),
            name: Name::Omer(Omer::Omer19),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 6),
            name: Name::Omer(Omer::Omer20),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 7),
            name: Name::Omer(Omer::Omer21),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 8),
            name: Name::Omer(Omer::Omer22),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 9),
            name: Name::Omer(Omer::Omer23),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 10),
            name: Name::Omer(Omer::Omer24),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 11),
            name: Name::Omer(Omer::Omer25),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 12),
            name: Name::Omer(Omer::Omer26),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 13),
            name: Name::Omer(Omer::Omer27),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 14),
            name: Name::Omer(Omer::Omer28),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 15),
            name: Name::Omer(Omer::Omer29),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 16),
            name: Name::Omer(Omer::Omer30),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 17),
            name: Name::Omer(Omer::Omer31),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 18),
            name: Name::Omer(Omer::Omer32),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 19),
            name: Name::Omer(Omer::Omer33),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 20),
            name: Name::Omer(Omer::Omer34),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 21),
            name: Name::Omer(Omer::Omer35),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 22),
            name: Name::Omer(Omer::Omer36),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 23),
            name: Name::Omer(Omer::Omer37),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 24),
            name: Name::Omer(Omer::Omer38),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 25),
            name: Name::Omer(Omer::Omer39),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 26),
            name: Name::Omer(Omer::Omer40),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 27),
            name: Name::Omer(Omer::Omer41),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 28),
            name: Name::Omer(Omer::Omer42),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 29),
            name: Name::Omer(Omer::Omer43),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 30),
            name: Name::Omer(Omer::Omer44),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 1),
            name: Name::Omer(Omer::Omer45),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 2),
            name: Name::Omer(Omer::Omer46),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 3),
            name: Name::Omer(Omer::Omer47),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 4),
            name: Name::Omer(Omer::Omer48),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 5),
            name: Name::Omer(Omer::Omer49),
            candle_lighting: None,
        }),
    ])
}
