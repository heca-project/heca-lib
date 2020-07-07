mod rosh_chodesh_adar_data;

use heca_lib::hebrew::Month;
use heca_lib::prelude::*;
use rosh_chodesh_adar_data::ROSH_CHODESH_ADAR_SECULAR_DAY;
use std::convert::TryInto;

#[test]
fn test_rosh_chodesh_adar() {
    ROSH_CHODESH_ADAR_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: (heca_lib::hebrew::Date, heca_lib::hebrew::Date) =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        let hebrew_date = hebrew_date.0;
        if (hebrew_date.month() != Month::Adar || hebrew_date.day().get() != 1)
            && (hebrew_date.month() != Month::Adar1 || hebrew_date.day().get() != 1)
            && (hebrew_date.month() != Month::Adar2 || hebrew_date.day().get() != 1)
            && (hebrew_date.month() != Month::Adar1 || hebrew_date.day().get() != 30)
            && (hebrew_date.month() != Month::Shvat || hebrew_date.day().get() != 30)
        {
            panic!("{:#?}, {:#?}", x, hebrew_date);
        }
    });
}
