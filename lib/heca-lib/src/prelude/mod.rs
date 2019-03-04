pub mod location;
pub use location::*;
enum_from_primitive! {
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Day{
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Shabbos
}
}
enum_from_primitive! {
  #[derive(Debug, PartialEq, Copy, Clone)]
  pub enum HebrewMonth {
    Tishrei = 0,
    Cheshvan = 1,
    Kislev = 2,
    Teves = 3,
    Shvat = 4,
    Adar = 5,
    Adar1 = 6,
    Adar2 = 7,
    Nissan = 8,
    Iyar = 9,
    Sivan = 10,
    Tammuz = 11,
    Av = 12,
    Elul = 13
  }
}

#[derive(Debug)]
pub enum ConversionError {
    IsNotLeapYear,
    TooManyDaysInMonth(u8),
    IsLeapYear,
    MonthDoesntExist,
    YearTooSmall,
    DayIsZero,
}