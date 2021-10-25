use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::process::{Command, Output};
use std::{io, result};

use cached::proc_macro::cached;

#[cached]
pub fn warning_message(threshold: u8) -> String {
    format!(
        "Battery percentage reached the {}% threshold, please unplug your charger",
        threshold,
    )
}

pub fn vec_to_set<T>(v: Vec<T>) -> HashSet<T>
where
    T: Eq + Hash,
{
    v.into_iter().collect()
}

pub fn warn_on_err<T, E>(result: result::Result<T, E>) -> Option<T>
where
    E: Display,
{
    result.map_err(|e| log::warn!("{}", e)).ok()
}

pub fn print_slice<T>(slice: &[T])
where
    T: Display,
{
    slice.iter().for_each(|item| println!("{}", item));
}

pub fn format_option<T>(option: &Option<T>) -> String
where
    T: Display,
{
    match option {
        None => "None".into(),
        Some(value) => format!("{}", value),
    }
}

pub fn slice_to_string(slice: &[u8]) -> String {
    String::from_utf8_lossy(slice).to_string()
}

pub fn command(args: &str) -> Result<Output, io::Error> {
    log::debug!("sh -c \"{}\"", args);

    Command::new("sh").arg("-c").arg(args).output()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::error;

    use super::*;

    #[test]
    fn test_warning_message() {
        let threshold = 50;

        let result = warning_message(threshold);

        assert_eq!(result, format!(
            "Battery percentage reached the {}% threshold, please unplug your charger",
            threshold,
        ));
    }

    #[test]
    fn test_vec_to_set() {
        let v = vec![1, 2, 3];

        let result = vec_to_set(v);

        assert_eq!(HashSet::from_iter([1, 2, 3]), result);
    }

    #[test]
    fn test_vec_to_set_empty() {
        let v = vec![];

        let result: HashSet<u8> = vec_to_set(v);

        assert_eq!(HashSet::with_capacity(0), result);
    }

    #[test]
    fn test_warn_on_err_ok() {
        let r: Result<(), error::Error> = Ok(());

        let result = warn_on_err(r);

        assert_eq!(Some(()), result);
    }

    #[test]
    fn test_warn_on_err_err() {
        let r: Result<(), error::Error> =
            Err(error::Error::Battery(error::Battery::NotFound {
                model: error::Model(Some("test".into())),
            }));

        let result = warn_on_err(r);

        assert_eq!(None, result);
    }

    #[test]
    fn test_format_option_none() {
        let option: Option<&str> = None;

        let result = format_option(&option);

        assert_eq!("None", result);
    }

    #[test]
    fn test_format_option_some() {
        let option: Option<&str> = Some("123");

        let result = format_option(&option);

        assert_eq!("123", result);
    }

    #[test]
    fn test_slice_to_string() {
        let slice = [240, 159, 146, 150];

        let result = slice_to_string(&slice);

        assert_eq!("\u{1f496}", result);
    }

    #[test]
    fn test_slice_to_string_empty() {
        let slice = [];

        let result = slice_to_string(&slice);

        assert_eq!("", result);
    }
}
