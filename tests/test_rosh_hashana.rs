mod rosh_hashana_data;
#[test]
fn test_rosh_hashana() {
    use heca_lib::hebrew::Month;
    use heca_lib::prelude::*;
    use rosh_hashana_data::*;
    use std::convert::TryInto;
    ROSH_HASHANA_SECOND_DAY_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: (heca_lib::hebrew::Date, heca_lib::hebrew::Date) =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        let hebrew_date = hebrew_date.0;
        if hebrew_date.month() != Month::Tishrei || hebrew_date.day().get() != 2 {
            panic!("{:#?}", hebrew_date);
        }
    });
}
