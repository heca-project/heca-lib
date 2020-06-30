use crate::hebrew::Month;
use crate::hebrew::Year;
use crate::prelude::Holiday;
use crate::prelude::*;
use tinyvec::TinyVec;
pub(crate) fn get<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    get_yud_kislev(year, return_vec);
    get_yud_tes_kislev(year, return_vec);
    get_chof_kislev(year, return_vec);
    get_yud_beis_tammuz(year, return_vec);
    get_yud_gimmel_tammuz(year, return_vec);
}

fn get_yud_kislev<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5588 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Kislev, 10),
            name: Name::ChabadHoliday(ChabadHoliday::YudKislev),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_yud_tes_kislev<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5560 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Kislev, 19),
            name: Name::ChabadHoliday(ChabadHoliday::YudTesKislev),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_chof_kislev<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5560 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Kislev, 20),
            name: Name::ChabadHoliday(ChabadHoliday::ChofKislev),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_yud_beis_tammuz<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5688 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Tammuz, 12),
            name: Name::ChabadHoliday(ChabadHoliday::YudBeisTammuz),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_yud_gimmel_tammuz<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5689 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Tammuz, 13),
            name: Name::ChabadHoliday(ChabadHoliday::YudGimmelTammuz),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChabadHoliday {
    YudKislev,
    YudTesKislev,
    ChofKislev,
    YudBeisTammuz,
    YudGimmelTammuz,
}
