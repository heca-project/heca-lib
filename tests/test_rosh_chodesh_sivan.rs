mod rosh_chodesh_sivan_data;

use heca_lib::hebrew::Month;
use heca_lib::prelude::*;
use rosh_chodesh_sivan_data::ROSH_CHODESH_SIVAN_SECULAR_DAY;
use std::convert::TryInto;

#[test]
fn test_rosh_chodesh_sivan() {
    ROSH_CHODESH_SIVAN_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: (heca_lib::hebrew::Date, heca_lib::hebrew::Date) =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        if hebrew_date.0.month() != Month::Sivan || hebrew_date.0.day().get() != 1 {
            panic!("{:#?}", x);
        }
    });
}
