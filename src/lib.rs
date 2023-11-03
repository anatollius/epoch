use chrono::naive::{Days, NaiveDate};
use regex::Regex;

const EPOCH_ZERO: &str = "2021-03-01";

pub fn get_inputs(mut args: impl Iterator<Item = String>) -> Result<Vec<String>, &'static str> {
    args.next()
        .expect("The first arg should be the programme name");

    let mut inputs: Vec<String> = Vec::new();

    for arg in args {
        inputs.push(arg)
    }

    if inputs.len() == 0 {
        return Err("You need to provide at least one epoch or date");
    }

    Ok(inputs)
}

pub fn convert_input(input: &str) -> Result<String, &str> {
    let epoch_re: Regex = Regex::new(r"^\d+(-\d\d)?$").unwrap();

    match epoch_re.find(&input) {
        Some(_) => Ok(convert_epoch(&input)),
        None => Ok(convert_date(&input)?),
    }
}

fn convert_epoch(epoch: &str) -> String {
    let mut epoch_parts = epoch.split("-");
    let days: u64 = epoch_parts
        .next()
        .expect("Missing the first part of epoch")
        .parse()
        .expect("counldn't convert epoch days into an int");

    if let Some(date) = NaiveDate::parse_from_str(EPOCH_ZERO, "%Y-%m-%d")
        .unwrap()
        .checked_add_days(Days::new(days))
    {
        return date.to_string();
    } else {
        panic!("Date out of range!")
    };
}

fn convert_date(date: &str) -> Result<String, &str> {
    let epoch_zero = NaiveDate::parse_from_str(EPOCH_ZERO, "%Y-%m-%d").unwrap();
    if let Ok(naive_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        let days = naive_date.signed_duration_since(epoch_zero).num_days();
        if days < 0 {
            Err("Date must be on or after 2021-03-01")
        } else {
            Ok(format!("{}-01", days))
        }
    } else {
        Err("Invalid epoch or date")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_convert_epoch() {
        assert_eq!(convert_epoch("0"), String::from("2021-03-01"));
        assert_eq!(convert_epoch("10"), String::from("2021-03-11"));
        assert_eq!(convert_epoch("365-01"), String::from("2022-03-01"));
    }

    #[test]
    fn check_convert_date() {
        assert_eq!(convert_date("2021-03-01"), Ok(String::from("0-01")));
        assert_eq!(convert_date("2022-04-22"), Ok(String::from("417-01")));
        assert_eq!(
            convert_date("2020-04-22"),
            Err("Date must be on or after 2021-03-01")
        );
    }

    #[test]
    fn check_convert_input() {
        assert_eq!(convert_input("600-01"), Ok(String::from("2022-10-22")));
        assert_eq!(convert_input("2021-08-19"), Ok(String::from("171-01")));
        assert_eq!(convert_input(""), Err("Invalid epoch or date"));
        assert_eq!(convert_input("foobar"), Err("Invalid epoch or date"));
    }

    #[test]
    fn check_get_inputs_1_arg() {
        let args = vec![String::from("epoch"), String::from("220-01")];
        assert_eq!(
            get_inputs(args.into_iter()),
            Ok(vec![String::from("220-01")])
        )
    }

    #[test]
    fn check_get_inputs_2_arg() {
        let args = vec![
            String::from("epoch"),
            String::from("220-01"),
            String::from("2023-02-02"),
        ];
        assert_eq!(
            get_inputs(args.into_iter()),
            Ok(vec![String::from("220-01"), String::from("2023-02-02")])
        )
    }

    #[test]
    fn check_get_inputs_error() {
        let args = vec![String::from("epoch")];
        assert_eq!(
            get_inputs(args.into_iter()),
            Err("You need to provide at least one epoch or date")
        )
    }
}
