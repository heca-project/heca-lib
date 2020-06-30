mod sukkos_data;
#[test]
fn test_sukkos() {
    use heca_lib::hebrew::Month;
    use heca_lib::prelude::*;
    use std::convert::TryInto;
    use sukkos_data::*;
    SUKKOS_FIRST_DAY.iter().for_each(|x| {
        let hebrew_date: heca_lib::hebrew::Date =
            (heca_lib::secular::Date::from_ymd(x.2 as i32, x.0, x.1) - Duration::days(1))
                .try_into()
                .unwrap();
        if hebrew_date.month() != Month::Tishrei || hebrew_date.day().get() != 15 {
            panic!("{:#?}", x);
        }
    });
}
