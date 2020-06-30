mod chanuka_data;
#[test]
fn test_rosh_hashana() {
    use chanuka_data::*;
    use heca_lib::hebrew::Month;
    use heca_lib::prelude::*;
    use std::convert::TryInto;
    CHANUKA_FIRST_NIGHT_SECULAR_DAY.iter().for_each(|x| {
        let hebrew_date: heca_lib::hebrew::Date =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1))
                .try_into()
                .unwrap();
        if hebrew_date.month() != Month::Kislev || hebrew_date.day().get() != 25 {
            panic!("{:#?}", x);
        }
    });
}
