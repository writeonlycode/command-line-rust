use anyhow::{bail, Result};
use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;

pub fn run(config: Config) -> Result<()> {
    let now = Local::now().date_naive();

    let month = match config.month {
        Some(month) => month,
        None => now.month0() + 1,
    };

    let year = match config.year {
        Some(year) => year,
        None => now.year_ce().1.try_into().unwrap(),
    };

    let show_current_year = match config.show_current_year {
        true => true,
        false => now.year_ce().1 != year.try_into().unwrap(),
    };

    let show_whole_year = match (config.year, config.month) {
        (Some(_), Some(_)) => false,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => show_current_year,
    };

    if show_whole_year {
        let mut months = vec![];

        for month in 1..13 {
            let formatted_month = format_month(year, month, false, now);
            months.push(formatted_month);
        }

        println!("{:>32}", year);

        for trimester in 0..4 {
            for line in 0..8 {
                println!(
                    "{}{}{}",
                    months[trimester * 3][line],
                    months[trimester * 3 + 1][line],
                    months[trimester * 3 + 2][line]
                );
            }
            if trimester < 3 {
                println!();
            }
        }
    } else {
        let month = format_month(year, month, show_current_year, now);

        for line in month {
            println!("{}", line);
        }
    }

    Ok(())
}

fn format_month(year: i32, month: u32, print_year: bool, today: NaiveDate) -> Vec<String> {
    let month_year = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();

    let mut counter = 0;
    let mut line = String::from("");

    let mut result = vec![];

    let month_name = MONTH_NAMES[month_year.month0() as usize];

    let head = if print_year {
        format!("{} {}", month_name, month_year.year_ce().1)
    } else {
        month_name.to_string()
    };

    result.push(format!("{:^20}  ", head));

    result.push("Su Mo Tu We Th Fr Sa  ".to_string());

    for _day in 1..month_year.weekday().number_from_sunday() {
        line.push_str("   ");
        counter += 1;
    }

    for day in month_year.iter_days() {
        if day.year_ce().1 != year as u32 || day.month0() != month - 1 {
            if !line.is_empty() {
                result.push(format!("{:<22}", line));
            }

            result.push(format!("{:<22}", ""));
            break;
        }

        if counter < 6 {
            line.push_str(format!("{:2} ", day.day0() + 1).as_str());
            counter += 1;
        } else {
            line.push_str(format!("{:2} ", day.day0() + 1).as_str());

            if !line.is_empty() {
                result.push(format!("{:<22}", line));
            }

            line = String::from("");
            counter = 0;
        }
    }

    result
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    #[arg(name = "YEAR", help = "Year (1-9999)", value_parser = parse_year)]
    year: Option<i32>,

    #[arg(help = "Month name or number (1-12)", short, long, value_parser = parse_month, group="month_year")]
    month: Option<u32>,

    #[arg(
        name = "year",
        help = "Show current year",
        short,
        long,
        group = "month_year",
        conflicts_with = "YEAR"
    )]
    show_current_year: bool,

    #[arg(skip)]
    day: u32,
}

fn parse_year(value: &str) -> Result<i32> {
    let year: i32 = value.parse()?;

    if (1..=9999).contains(&year) {
        bail!("{} is not in 1..=9999", year)
    }

    Ok(year)
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn parse_month(value: &str) -> Result<u32> {
    match value.parse() {
        Ok(month) => {
            if (1..=12).contains(&month) {
                bail!("month \"{}\" not in the range 1 through 12", month)
            } else {
                Ok(month)
            }
        }
        Err(_) => {
            let matched_months: Vec<usize> = MONTH_NAMES
                .iter()
                .enumerate()
                .filter_map(|(index, month_name)| {
                    if month_name.to_lowercase().starts_with(value) {
                        Some(index + 1)
                    } else {
                        None
                    }
                })
                .collect();

            if matched_months.len() != 1 {
                bail!("invalid month \"{}\"", value)
            } else {
                Ok(matched_months[0] as u32)
            }
        }
    }
}
