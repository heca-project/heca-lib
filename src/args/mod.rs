use clap::{App, Arg, ArgMatches, SubCommand};

pub mod types;
use crate::args::types::*;
use atoi::*;
use chrono::prelude::*;
use either::*;
use heca_lib::prelude::*;
use heca_lib::HebrewDate;
use std::env;

pub fn build_args<'a>() -> MainArgs {
    parse_args(App::new("Hebrew Calendar Manipulator")
        .version("0.2.0")
        .about(
            "This program is a fast utility to convert and analyze dates in the Hebrew Calendar.",
        )
/*        .arg(
     Arg::with_name("configfile")
                .long("config")
                .help("Sets a custom config file (default: $XDG_CONFIG_HOME/heca/config.yaml)")
                .takes_value(true)
                .required(false),
        )*/
.arg(
            Arg::with_name("type")
                .long("print")
                .help("Set output type")
                .possible_values(&["regular", "pretty", "json"])
                .takes_value(true)
                .default_value("pretty")
                .required(false),
        )
        .arg(
            Arg::with_name("language")
                .long("language")
                .help("Set language")
                .possible_values(&["en_US", "he_IL"])
                .takes_value(true)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("convert")
                .about("Converts Hebrew to Gregorian and back")
                .arg(
                    Arg::with_name("DateFormat")
                        .long("datefmt")
                        .help("Set date format (for Gregorian only): US or M for mm/dd/yyyy, UK or L for dd/mm/yyyy, ISO or B for yyyy/mm/dd")
                        .possible_values(&["US","M","UK","L","ISO","B"])
                        .takes_value(true)
                        .required(false)
                        .default_value("ISO")
                )
                .arg(Arg::with_name("T")
                     .long("type")
                    .long_help("Force conversion from type T, where T is either \"hebrew\" (then date must be written as '5/אדרא/5779'), as \"gregorian\" (where the date must be written as '1996/12/19'), or fuzzy (is Hebrew if year is above 4000, Gregorian otherwise).")
                    .possible_values(&["hebrew", "gregorian", "fuzzy"])
                    .takes_value(true)
                    .required(false)
                    .default_value("fuzzy")
        )
                .arg(Arg::with_name("Date")
                     .required(true)
                     .takes_value(true))

         ).subcommand(SubCommand::with_name("list")
                      .arg(Arg::with_name("YearType")
                           .long("type")
                           .help("Specify if the year is a Hebrew or a Gregorian year.")
                           .possible_values(&["hebrew", "gregorian", "fuzzy"])
                           .default_value("fuzzy")
                           .takes_value(true)
                           .required(false)
                      )
                     .arg(Arg::with_name("NoSort")
                          .long("no-sort")
                          .help("Don't sort output."))
                     .arg(Arg::with_name("Location")
                           .long("location")
                           .help("Are you looking for an Israeli calendar or a Chutz La'aretz calendar?")
                           .takes_value(true)
                           .required(false)
                           .possible_values(&["Chul","Israel"]))
                      .arg(Arg::with_name("AmountYears")
                           .long("years")
                           .help("Generate events for n years")
                           .takes_value(true)
                           .required(false)
                           .default_value("1"))
                      .arg(Arg::with_name("Events")
                           .long("show")
                           .help("What events to list")
                           .takes_value(true)
                           .multiple(true)
                           .required(false)
                           .use_delimiter(true)
                           .possible_values(&["yom-tov","shabbos","special-parshas","chol","minor-holidays", "omer"])
                           .default_value("yom-tov"))
                      .arg(Arg::with_name("Year")
.required(true)
                     .takes_value(true))
                           )
        .get_matches())
}

fn parse_args(matches: ArgMatches) -> MainArgs {
    let config = {
        if let Some(v) = matches.value_of("configfile") {
            //            Some(String::from(v))
            None
        } else {
            if let Ok(base_dir) = xdg::BaseDirectories::with_prefix("heca") {
                if let Some(path) = base_dir.find_config_file("config.yaml") {
                    //    Some(String::from(path.to_string_lossy()))
                    None
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    let output_type = match matches.value_of("type").unwrap() {
        "regular" => OutputType::Regular,
        "pretty" => OutputType::Pretty,
        "json" => OutputType::JSON,
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let language = match matches.value_of("language").unwrap_or("") {
        "en_US" => Language::English,
        "he_IL" => Language::Hebrew,
        "" => {
            let lang = env::vars().into_iter().find(|x| x.0 == "LANG");
            match lang {
                None => Language::English,
                Some(x) => {
                    if x.1 == "he_IL.UTF-8" {
                        Language::Hebrew
                    } else {
                        Language::English
                    }
                }
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let command = if let Some(matches) = matches.subcommand_matches("list") {
        parse_list(matches, language == Language::Hebrew)
    } else if let Some(matches) = matches.subcommand_matches("convert") {
        parse_convert(matches, language == Language::Hebrew)
    } else {
        println!("{}", matches.usage());
        std::process::exit(1);
    };

    MainArgs {
        config,
        output_type,
        language,
        command,
    }
}

fn parse_hebrew(sp: &[&str]) -> Command {
    let day = atoi::<u8>(sp[0].as_bytes()).expect("Day entered is not a number");
    let year = atoi::<u64>(sp[2].as_bytes()).expect("Year entered is not a number.");
    let month = {
        let mut a = str_to_month(sp[1], true);
        if !a.is_some() {
            a = str_to_month(&(String::from(sp[1]).to_lowercase()), false);
        }
        a
    }
    .expect(&format!("Cannot parse month: {}", sp[1]));
    Command::Convert(ConvertArgs {
        date: ConvertType::Hebrew(HebrewDate::from_ymd(year, month, day).unwrap()),
    })
}

fn parse_gregorian(sp: &[&str], format: &str) -> Command {
    let (day, month, year) = match format {
        "ISO" | "B" => {
            let year = atoi::<i32>(sp[0].as_bytes()).expect("Could not parse year");
            let month =
                atoi::<u32>(sp[1].as_bytes()).expect(&format!("Cannot not parse month {}", sp[1]));
            let day = atoi::<u32>(sp[2].as_bytes()).expect("Could not parse day");
            (day, month, year)
        }
        "US" | "M" => {
            let month = atoi::<u32>(sp[0].as_bytes()).expect("Could not parse month");
            let day = atoi::<u32>(sp[1].as_bytes()).expect("Day entered is not a number");
            let year = atoi::<i32>(sp[2].as_bytes()).expect("Year entered is not a number.");

            (day, month, year)
        }
        "UK" | "L" => {
            let day = atoi::<u32>(sp[0].as_bytes()).expect("Day entered is not a number");
            let month = atoi::<u32>(sp[1].as_bytes()).expect("Could not parse month");
            let year = atoi::<i32>(sp[2].as_bytes()).expect("Year entered is not a number.");

            (day, month, year)
        }
        x => {
            panic!(format!("Assertion error! How did {} get here?", x));
        }
    };
    Command::Convert(ConvertArgs {
        date: ConvertType::Gregorian(Utc.ymd(year, month, day)),
    })
}
fn parse_convert(matches: &ArgMatches, hebrew: bool) -> Command {
    let date = matches.value_of("Date").unwrap();

    let sp = date
        .split(&['-', '/', '_', '\\', '.', ',', '='][..])
        .collect::<Vec<&str>>();
    if sp.len() != 3 {
        panic!("Couldn't parse date");
    }

    match matches.value_of("T").unwrap() {
        "hebrew" => parse_hebrew(&sp),
        "gregorian" => parse_gregorian(&sp, matches.value_of("DateFormat").unwrap()),
        "fuzzy" => {
            if sp[1].parse::<f64>().is_ok() {
                parse_gregorian(&sp, matches.value_of("DateFormat").unwrap())
            } else {
                parse_hebrew(&sp)
            }
        }
        x => panic!(format!("Assertion error! How did {} get here?", x)),
    }
}

fn str_to_month(text: &str, exact: bool) -> Option<HebrewMonth> {
    let ret = match text {
        "תשרי" => Some(HebrewMonth::Tishrei),
        "חשון" => Some(HebrewMonth::Cheshvan),
        "כסלב" => Some(HebrewMonth::Kislev),
        "טבת" => Some(HebrewMonth::Teves),
        "שבט" => Some(HebrewMonth::Shvat),
        "אדר" => Some(HebrewMonth::Adar),
        "אדרא" => Some(HebrewMonth::Adar1),
        "אדרב" => Some(HebrewMonth::Adar2),
        "ניסן" => Some(HebrewMonth::Nissan),
        "אייר" => Some(HebrewMonth::Iyar),
        "סיון" => Some(HebrewMonth::Sivan),
        "תמוז" => Some(HebrewMonth::Tammuz),
        "אב" => Some(HebrewMonth::Av),
        "אלול" => Some(HebrewMonth::Elul),
        _ => None,
    };
    if exact || ret.is_some() {
        return ret;
    }
    match text {
        "tishrei" | "tishre" => Some(HebrewMonth::Tishrei),
        "cheshvan" | "marcheshvan" | "mar cheshvan" => Some(HebrewMonth::Cheshvan),
        "kislev" => Some(HebrewMonth::Kislev),
        "teves" | "tevet" | "teiveis" => Some(HebrewMonth::Teves),
        "shvat" | "shevat" => Some(HebrewMonth::Shvat),
        "adar" => Some(HebrewMonth::Adar),
        "adar1" | "adar 1" | "adar aleph" | "adar rishon" => Some(HebrewMonth::Adar1),
        "adar2" | "adar 2" | "adar beis" | "adar bet" | "adar sheini" => Some(HebrewMonth::Adar2),
        "nissan" | "Nisan" => Some(HebrewMonth::Nissan),
        "iyar" => Some(HebrewMonth::Iyar),
        "sivan" => Some(HebrewMonth::Sivan),
        "tammuz" | "tamuz" => Some(HebrewMonth::Tammuz),
        "av" | "menachem av" => Some(HebrewMonth::Av),
        "elul" | "ellul" => Some(HebrewMonth::Elul),
        _ => None,
    }
}

fn parse_list(matches: &ArgMatches, hebrew: bool) -> Command {
    use atoi::atoi;
    let year_num = atoi::<u64>(matches.value_of("Year").unwrap().as_bytes())
        .expect("The supplied year must be a number");
    let amnt_years = atoi::<u64>(matches.value_of("AmountYears").unwrap().as_bytes())
        .expect("The supplied year must be a number");

    let year = match matches.value_of("YearType").unwrap() {
        "hebrew" => YearType::Hebrew(year_num),
        "gregorian" => YearType::Gregorian(year_num),
        "fuzzy" => {
            if year_num > 3000 {
                YearType::Hebrew(year_num)
            } else {
                YearType::Gregorian(year_num)
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let no_sort = matches.occurrences_of("NoSort") > 0;

    let location = match matches.value_of("Location").unwrap_or("") {
        "Chul" => Location::Chul,
        "Israel" => Location::Israel,
        "" => {
            if hebrew {
                Location::Israel
            } else {
                Location::Chul
            }
        }
        x => panic!(format!("Assertion Error! How did {} get here?", x)),
    };

    let events = matches
        .values_of("Events")
        .unwrap()
        .into_iter()
        .map(|x| match x {
            "yom-tov" => Either::Left(TorahReadingType::YomTov),
            "chol" => Either::Left(TorahReadingType::Chol),
            "shabbos" => Either::Left(TorahReadingType::Shabbos),
            "special-parshas" => Either::Left(TorahReadingType::SpecialParsha),
            "omer" => Either::Right(CustomHoliday::Omer),
            "minor-holidays" => Either::Right(CustomHoliday::Minor),
            x => panic!(format!("Assertion Error! How did {} get here?", x)),
        })
        .collect::<Vec<Either<TorahReadingType, CustomHoliday>>>();
    Command::List(ListArgs {
        year,
        location,
        events,
        amnt_years,
        no_sort,
    })
}
