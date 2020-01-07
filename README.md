[![Crates.io](https://img.shields.io/crates/v/heca_lib.svg)](https://crates.io/crates/heca_lib)
[![Documentation](https://docs.rs/heca-lib/badge.svg)](https://docs.rs/heca-lib)
[![Build Status](https://travis-ci.org/heca-project/heca.svg?branch=master)](https://travis-ci.org/heca-project/heca)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/heca-project/heca.svg)](https://isitmaintained.com/project/heca-project/heca "Percentage of issues still open")
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


# heca-lib

## heca-lib
heca-lib is a blazingly fast Hebrew calender library. It's the backend behind the heca program.

## Usage:

1. Add to Cargo.toml:

```toml
    [dependencies]
    heca-lib = "1.3"
```

3. Import the types:

```rust
use heca_lib::prelude::*;
use heca_lib::*;
```

## Overview:

### Convert:

This library can convert from Hebrew to Gregorian dates and back. You can get a HebrewDate either from a known Hebrew date or from a Gregorian date:

```rust

extern crate heca_lib;

use chrono::Utc;
use chrono::offset::TimeZone;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;

assert_eq!(HebrewDate::from_gregorian(Utc.ymd(2018,9,10).and_hms(17,59,59)).unwrap(),HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,1).unwrap());

```

You can then get back a Gregorian date from this Hebrew Date.

```rust

extern crate heca_lib;

use chrono::Utc;
use chrono::offset::TimeZone;
use heca_lib::{HebrewDate};
use heca_lib::prelude::*;

assert_eq!(HebrewDate::from_ymd(5779,HebrewMonth::Tishrei,10).unwrap().to_gregorian(),Utc.ymd(2018, 9,18).and_hms(18,00,00));

```

### Get Information on the Hebrew Year

This library can also list the major Jewish holidays and Torah readings in a given year (for
both Israel and the Diaspora):

```rust

extern crate heca_lib;

use heca_lib::{HebrewYear,HebrewDate};
use heca_lib::prelude::*;

assert_eq!(HebrewYear::new(5779).unwrap().get_holidays(Location::Chul, &[TorahReadingType::Shabbos])[0].name(), TorahReading::Shabbos(Parsha::Vayelach));
assert_eq!(HebrewYear::new(5779).unwrap().get_holidays(Location::Chul, &[TorahReadingType::SpecialParsha]).iter().find(|x| x.name() == TorahReading::SpecialParsha(SpecialParsha::Zachor)).unwrap().day(),HebrewDate::from_ymd(5779,HebrewMonth::Adar2,9).unwrap());

```


## Notes:

1. This library won't work for years before 3764 (4).
2. I tested this library against hebcal for all Rosh Hashanas between 3764 and 9999 (4-6239). I also checked it for all Rosh Chodesh Adars in those years. However, I take no resposibility if you accidently keep Yom Tov on the wrong day!

Current version: 1.3.1


License: MIT
