use serde::{Deserialize, Serialize};

use super::{Holiday, Name};
use crate::{
    prelude::{Day, Location},
    HebrewDate, HebrewYear,
};

use tinyvec::TinyVec;

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
        Some(Holiday {
            name: Name::Shabbos(parsha_vec[i]),
            day: v,
        })
    }))
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
    Parsha::Haazinu,
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

/// Weekly Torah Portion
#[derive(Clone, Debug, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum Parsha {
    Vayelech,
    Haazinu,
    Bereishis,
    Noach,
    LechLecha,
    Vayeira,
    ChayeiSara,
    Toldos,
    Vayetzei,
    Vayishlach,
    Vayeshev,
    Miketz,
    Vayigash,
    Vayechi,
    Shemos,
    Vaeira,
    Bo,
    Beshalach,
    Yisro,
    Mishpatim,
    Terumah,
    Tetzaveh,
    KiSisa,
    VayakhelPikudei,
    Vayakhel,
    Pikudei,
    Vayikra,
    Tzav,
    Shemini,
    TazriyaMetzorah,
    Tazriya,
    Metzorah,
    AchareiMosKedoshim,
    AchareiMos,
    Kedoshim,
    Emor,
    BeharBechukosai,
    Behar,
    Bechukosai,
    Bamidbar,
    Naso,
    Behaaloscha,
    Shlach,
    Korach,
    ChukasBalak,
    Chukas,
    Balak,
    Pinchas,
    MatosMaasei,
    Matos,
    Maasei,
    Devarim,
    Vaeschanan,
    Eikev,
    Reeh,
    Shoftim,
    KiSeitzei,
    KiSavoh,
    NitzavimVayelech,
    Nitzavim,
}
