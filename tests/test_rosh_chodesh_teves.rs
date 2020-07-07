mod rosh_chodesh_teves_data;

use heca_lib::hebrew::Month;
use heca_lib::prelude::*;
use rosh_chodesh_teves_data::ROSH_CHODESH_TEVES_SECULAR_DAY;
use std::convert::TryInto;

#[test]
fn test_rosh_chodesh_teves() {
    ROSH_CHODESH_TEVES_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: (heca_lib::hebrew::Date, heca_lib::hebrew::Date) =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        let hebrew_date = hebrew_date.0;
        if (hebrew_date.month() != Month::Teves || hebrew_date.day().get() != 1)
            && (hebrew_date.month() != Month::Kislev || hebrew_date.day().get() != 30)
        {
            panic!("{:#?}", x);
        }
    });
}
