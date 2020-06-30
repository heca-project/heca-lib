use crate::hebrew::Month;
use crate::hebrew::Year;
use crate::prelude::Holiday;
use crate::prelude::YearSchedule;
use crate::prelude::*;
use tinyvec::TinyVec;
pub(crate) fn get<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
    exact_days: bool,
) {
    get_yom_haaliyah(year, return_vec);
    get_yom_yerushalayim(year, return_vec);
    get_sigd(year, return_vec);
    get_yom_hashoah(year, return_vec, exact_days);
    get_yom_haatzmaut_and_yom_hazikaron(year, return_vec, exact_days);
}
fn get_yom_haaliyah<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5777 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Cheshvan, 7),
            name: Name::IsraeliHoliday(IsraeliHoliday::YomHaAliyah),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_yom_yerushalayim<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5727 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Iyar, 28),
            name: Name::IsraeliHoliday(IsraeliHoliday::YomYerushalayim),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_sigd<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    if year.year() >= 5769 {
        return_vec.extend(std::iter::once(Some(Holiday {
            day: year.and_month_day(Month::Cheshvan, 29),
            name: Name::IsraeliHoliday(IsraeliHoliday::Sigd),
            candle_lighting: None,
            tzeis: None,
        })));
    }
}

fn get_yom_hashoah<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
    exact_days: bool,
) {
    if year.year() >= 5711 {
        let offset: i8 = if exact_days {
            0
        } else {
            match year.year_type() {
                YearSchedule::BaChaG
                | YearSchedule::HaShaG
                | YearSchedule::ZaChaG
                | YearSchedule::ZaShaG => 1,
                YearSchedule::GaKaZ | YearSchedule::BaShaZ | YearSchedule::HaKaZ => 0,
                YearSchedule::BaChaH
                | YearSchedule::BaShaH
                | YearSchedule::GaChaH
                | YearSchedule::ZaShaH => 0,
                YearSchedule::ZaChA | YearSchedule::HaShA | YearSchedule::HaChA => -1,
            }
        };
        if year.year() >= 5769 {
            return_vec.extend(std::iter::once(Some(Holiday {
                day: year.and_month_day(Month::Nissan, (27 + offset) as u8),
                name: Name::IsraeliHoliday(IsraeliHoliday::Sigd),
                candle_lighting: None,
                tzeis: None,
            })));
        }
    }
}

fn get_yom_haatzmaut_and_yom_hazikaron<T: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
    exact_days: bool,
) {
    let offset = if exact_days {
        0
    } else {
        match year.year_type() {
            YearSchedule::BaChaG
            | YearSchedule::HaShaG
            | YearSchedule::ZaChaG
            | YearSchedule::ZaShaG => {
                if year.year() < 5764 {
                    0
                } else {
                    1
                }
            }
            YearSchedule::GaKaZ | YearSchedule::BaShaZ | YearSchedule::HaKaZ => -1,
            YearSchedule::BaChaH
            | YearSchedule::BaShaH
            | YearSchedule::GaChaH
            | YearSchedule::ZaShaH => 0,
            YearSchedule::ZaChA | YearSchedule::HaShA | YearSchedule::HaChA => -2,
        }
    };

    return_vec.extend(std::iter::once(Some(Holiday {
        day: year.and_month_day(Month::Iyar, (4 + offset) as u8),
        name: Name::IsraeliHoliday(IsraeliHoliday::YomHaZikaron),
        candle_lighting: None,
        tzeis: None,
    })));

    return_vec.extend(std::iter::once(Some(Holiday {
        day: year.and_month_day(Month::Iyar, (5 + offset) as u8),
        name: Name::IsraeliHoliday(IsraeliHoliday::YomHaAtzmaut),
        candle_lighting: None,
        tzeis: None,
    })));
}

#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum IsraeliHoliday {
    YomHaZikaron,
    YomHaAtzmaut,
    YomYerushalayim,
    YomHaShoah,
    YomHaAliyah,
    Sigd,
}
