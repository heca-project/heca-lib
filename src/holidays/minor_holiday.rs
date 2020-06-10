use super::{Holiday, Name};
use crate::{
    hebrew::{Month, Year},
    prelude::Day,
};
use std::num::NonZeroU8;
use tinyvec::TinyVec;
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum MinorHoliday {
    ErevYomKippur,
    ErevSukkos,
    ErevPesach,
    PesachSheni,
    LagBaOmer,
    ErevShavuos,
    ErevRoshHashanah,
    FifteenShvat,
    FifteenAv,
    PurimKattan,
    ShushanPurimKattan,
    ShabbosHaGadol,
    TaanisBechoros,
    ShabbosChazon,
    ShabbosNachamu,
    LeilSlichos,
    ShabbosShuva,
}

pub(crate) fn get<S: Clone>(
    year: &Year,
    return_vec: &mut TinyVec<impl tinyvec::Array<Item = Option<Holiday<S>>>>,
) {
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 9),
            name: Name::MinorHoliday(MinorHoliday::ErevYomKippur),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 14).into(),
            name: Name::MinorHoliday(MinorHoliday::ErevYomKippur),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Tishrei, 14).into(),
            name: Name::MinorHoliday(MinorHoliday::ErevSukkos),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Nissan, 14).into(),
            name: Name::MinorHoliday(MinorHoliday::ErevPesach),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 14).into(),
            name: Name::MinorHoliday(MinorHoliday::PesachSheni),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Iyar, 18).into(),
            name: Name::MinorHoliday(MinorHoliday::LagBaOmer),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Sivan, 5).into(),
            name: Name::MinorHoliday(MinorHoliday::ErevShavuos),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Elul, 29).into(),
            name: Name::MinorHoliday(MinorHoliday::ErevRoshHashanah),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Shvat, 15).into(),
            name: Name::MinorHoliday(MinorHoliday::FifteenShvat),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year.and_month_day(Month::Av, 15).into(),
            name: Name::MinorHoliday(MinorHoliday::FifteenAv),
            candle_lighting: None,
        }),
    ]);

    if year.is_leap_year() {
        return_vec.extend_from_slice(&[
            Some(Holiday {
                day: year.and_month_day(Month::Adar1, 14).into(),
                name: Name::MinorHoliday(MinorHoliday::PurimKattan),
                candle_lighting: None,
            }),
            Some(Holiday {
                day: year.and_month_day(Month::Adar1, 15).into(),
                name: Name::MinorHoliday(MinorHoliday::ShushanPurimKattan),
                candle_lighting: None,
            }),
        ]);
    }
    let first_day_of_pesach = year.and_month_day(Month::Nissan, 15).day_of_week();
    let day_in_nissan = match first_day_of_pesach {
        Day::Shabbos => 14,
        Day::Monday => 12,
        Day::Wednesday => 10,
        Day::Friday => 8,
        _ => panic!("Pesach shouldn't fall out on a {:?}", first_day_of_pesach),
    };
    return_vec.extend(std::iter::once(Some(Holiday {
        day: year.and_month_day(Month::Nissan, day_in_nissan).into(),
        name: Name::MinorHoliday(MinorHoliday::ShabbosHaGadol),
        candle_lighting: None,
    })));

    let day_of_taanis_bechoros = if first_day_of_pesach == Day::Shabbos {
        12
    } else {
        14
    };

    return_vec.extend(std::iter::once(Some(Holiday {
        day: year
            .and_month_day(Month::Nissan, day_of_taanis_bechoros)
            .into(),
        name: Name::MinorHoliday(MinorHoliday::TaanisBechoros),
        candle_lighting: None,
    })));

    let day_of_tisha_beav = year.and_month_day(Month::Av, 9);

    let day_of_month_of_shabbos_chazon = match day_of_tisha_beav.day_of_week() {
        Day::Shabbos => 8,
        Day::Monday => 6,
        Day::Wednesday => 4,
        Day::Friday => 9,
        x => panic!("Tisha Beav shouldn't be on {:?}", x),
    };
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year
                .and_month_day(Month::Av, day_of_month_of_shabbos_chazon)
                .into(),
            name: Name::MinorHoliday(MinorHoliday::ShabbosChazon),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year
                .and_month_day(Month::Av, day_of_month_of_shabbos_chazon + 7)
                .into(),
            name: Name::MinorHoliday(MinorHoliday::ShabbosNachamu),
            candle_lighting: None,
        }),
    ]);

    let day_of_rh = year.and_month_day(Month::Tishrei, 1);

    let day_of_month_of_shabbos_shuva = match day_of_rh.day_of_week() {
        Day::Sunday => 6,
        Day::Monday => 5,
        Day::Wednesday => 3,
        Day::Friday => 8,
        x => panic!("Shabbos Shuva shouldn't be on {:?}", x),
    };

    let day_of_erev_rh = year.and_month_day(Month::Elul, 29);
    let day_of_month_of_leil_slichos = match day_of_erev_rh.day_of_week() {
        Day::Sunday => 21,
        Day::Tuesday => 26,
        Day::Thursday => 24,
        Day::Shabbos => 22,
        x => panic!("Leil Slichos shouldn't be on {:?}", x),
    };
    return_vec.extend_from_slice(&[
        Some(Holiday {
            day: year
                .and_month_day(Month::Elul, day_of_month_of_leil_slichos)
                .into(),
            name: Name::MinorHoliday(MinorHoliday::LeilSlichos),
            candle_lighting: None,
        }),
        Some(Holiday {
            day: year
                .and_month_day(Month::Tishrei, day_of_month_of_shabbos_shuva)
                .into(),
            name: Name::MinorHoliday(MinorHoliday::ShabbosShuva),
            candle_lighting: None,
        }),
    ]);
}
