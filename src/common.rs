use std::collections::HashSet;
use std::{fmt, hash, io, process};

pub fn vec_to_set<T>(v: Vec<T>) -> HashSet<T>
where
    T: Eq + hash::Hash,
{
    v.into_iter().collect()
}

pub fn warn_on_err<T, E>(prefix: &str, result: Result<T, E>) -> Option<T>
where
    E: fmt::Display,
{
    result.map_err(|e| log::warn!("{prefix}: {e}")).ok()
}

pub fn print_slice<T>(slice: &[T])
where
    T: fmt::Display,
{
    slice
        .iter()
        .enumerate()
        .for_each(|(index, item)| println!("{}. {item}", index + 1));
}

pub fn format_option<T>(option: Option<T>) -> String
where
    T: fmt::Display,
{
    option.map_or_else(|| "None".into(), |value| format!("{value}"))
}

pub fn format_string_set(set: &HashSet<String>) -> String {
    let size = set.iter().map(|s| s.len() + 1).sum();

    let mut result = String::with_capacity(size);

    let mut set_iter = set.iter();

    if let Some(first) = set_iter.next() {
        result.push_str(first);
    }

    for s in set_iter {
        result.push_str(", ");
        result.push_str(s);
    }

    result
}

pub fn slice_to_string(slice: &[u8]) -> String {
    String::from_utf8_lossy(slice).to_string()
}

pub fn command(
    program: &str,
    args: &[&str],
) -> Result<process::Output, io::Error> {
    let mut command = process::Command::new(program);

    command.args(args);

    log::debug!("common/command: {:#?}", command);

    command.output()
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use crate::error;

    use super::*;

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

        let result = warn_on_err("test/common", r);

        assert_eq!(Some(()), result);
    }

    #[test]
    fn test_warn_on_err_err() {
        let r: Result<(), error::Error> =
            Err(error::Error::Battery(error::Battery::NotFound {
                model: error::Model(Some("test".into())),
            }));

        let result = warn_on_err("test/common", r);

        assert_eq!(None, result);
    }

    #[test]
    fn test_format_option_none() {
        let option: Option<&str> = None;

        let result = format_option(option);

        assert_eq!("None", result);
    }

    #[test]
    fn test_format_option_some() {
        let option: Option<&str> = Some("123");

        let result = format_option(option);

        assert_eq!("123", result);
    }

    #[test]
    fn test_format_string_set() {
        let set = HashSet::from(["1".into(), "2".into()]);

        let result = format_string_set(&set);

        // set is unordered so we might get one of these 2 options
        assert!(["1, 2".into(), "2, 1".into()].contains(&result));
    }

    #[test]
    fn test_format_string_set_empty() {
        let set = HashSet::new();

        let result = format_string_set(&set);

        assert_eq!(result, "");
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
