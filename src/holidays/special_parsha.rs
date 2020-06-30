use super::{Holiday, Name};
use crate::{
    hebrew::{Month, Year},
    prelude::Day,
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;

pub(crate) fn get<T: Clone>(
    year: &Year,
    array_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<T>>>>,
) {
    let rh_dow_next = year.day_of_next_rh;
    let len_of_year = year.year_len;

    let shekalim = Holiday {
        //Parshas Shekalim is the Shabbos before, or the Shabbos of the second day of Rosh Chodesh Adar (or the second day of Rosh Chodesh Adar Beis).
        // The first day of Rosh Chodesh Adar II can never be on Shabbos, as Purim would then
        // be on Sunday, and then the next Rosh Hashana would start on a Wednesday, breaking Lo
        // Adu Rosh.
        //
        //If Rosh Chodesh Adar (Beis) is on Shabbos (like in the year 5805), the next Rosh Hashana starts on a Tuesday
        day: if rh_dow_next == Day::Tuesday {
            if len_of_year < 360 {
                // This is a regular year
                year.and_month_day(Month::Adar, 1)
            } else {
                // This is a leap year
                year.and_month_day(Month::Adar2, 1)
            }
        } else {
            let month = if len_of_year < 360 {
                Month::Shvat
            } else {
                Month::Adar1
            };
            // This is a regular year
            year.and_month_day(
                month,
                match rh_dow_next {
                    Day::Monday => 25,
                    Day::Thursday => 29,
                    Day::Shabbos => 27,
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                },
            )
        },
        name: Name::SpecialParsha(SpecialParsha::Shekalim),
        candle_lighting: None,
        tzeis: None,
    };
    let zachor = Holiday {
        //Parshas Zachor is read on the Shabbos before Purim.
        day: {
            let month = if len_of_year < 360 {
                Month::Adar
            } else {
                Month::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 9,    //For example, 5782
                Day::Tuesday => 8,   //For example, 5781
                Day::Thursday => 13, // For example, 5784
                Day::Shabbos => 11,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            year.and_month_day(month, day)
        },
        name: Name::SpecialParsha(SpecialParsha::Zachor),
        candle_lighting: None,
        tzeis: None,
    };
    let parah = Holiday {
        //Parshas Parah is read on the Shabbos before Hachodesh.
        day: {
            let month = if len_of_year < 360 {
                Month::Adar
            } else {
                Month::Adar2
            };
            let day = match rh_dow_next {
                Day::Monday => 23,   //For example, 5782
                Day::Tuesday => 22,  //For example, 5781
                Day::Thursday => 20, // For example, 5784
                Day::Shabbos => 18,  //For example, 5780
                _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
            };
            year.and_month_day(month, day)
        },
        name: Name::SpecialParsha(SpecialParsha::Parah),
        candle_lighting: None,
        tzeis: None,
    };
    let hachodesh = Holiday {
        //Parshas Hachodesh is read on the Shabbos before Rosh Chodesh Nissan, or on Rosh Chodesh
        //Nissan itself.
        day: {
            if rh_dow_next == Day::Monday {
                //Hachodesh is read on Rosh Chodesh Nissan itself
                year.and_month_day(Month::Nissan, 1)
            } else {
                let month = if len_of_year < 360 {
                    Month::Adar
                } else {
                    Month::Adar2
                };
                let day = match rh_dow_next {
                    Day::Tuesday => 29,  //For example, 5781
                    Day::Thursday => 27, // For example, 5784
                    Day::Shabbos => 25,  //For example, 5780
                    _ => panic!(format!("Day is on {:?}, violating ADU rosh", rh_dow_next)),
                };
                year.and_month_day(month, day)
            }
        },
        name: Name::SpecialParsha(SpecialParsha::HaChodesh),
        candle_lighting: None,
        tzeis: None,
    };

    array_vec.extend_from_slice(&[Some(shekalim), Some(zachor), Some(parah), Some(hachodesh)]);
}
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialParsha {
    Shekalim,
    Zachor,
    Parah,
    HaChodesh,
}
