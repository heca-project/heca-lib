extern crate heca;
extern crate chrono; 
extern crate time; 
use heca::*;
use chrono::prelude::*;

fn main() {
    println!("Year: {:?}", HebrewDate::from(Utc.ymd(2019, 1, 1).and_hms(23, 0, 0)));
}

    
