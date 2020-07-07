mod rosh_chodesh_elul_data;

use heca_lib::hebrew::Month;
use heca_lib::prelude::*;
use rosh_chodesh_elul_data::ROSH_CHODESH_ELUL_SECULAR_DAY;
use std::convert::TryInto;

#[test]
fn test_rosh_chodesh_elul() {
    ROSH_CHODESH_ELUL_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: (heca_lib::hebrew::Date, heca_lib::hebrew::Date) =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        if (hebrew_date.0.month() != Month::Elul || hebrew_date.0.day().get() != 1)
            && (hebrew_date.0.month() != Month::Av || hebrew_date.0.day().get() != 30)
        {
            panic!("{:#?}", x);
        }
    });
}
