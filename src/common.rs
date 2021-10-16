use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::result;

use cached::proc_macro::cached;

#[cached]
pub fn warning_message(threshold: u8) -> String {
    format!(
        "Battery percentage reached the {}% threshold, please unplug your charger",
        threshold,
    )
}

pub fn vec_to_hashset<T>(v: Vec<T>) -> HashSet<T>
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

#[cfg(test)]
mod tests {
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
}
