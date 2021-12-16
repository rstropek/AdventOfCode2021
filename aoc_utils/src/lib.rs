use std::{fs::{self, File}, path::{Path, PathBuf}, io::Read};

use termion::{color, style};

/// Constructs the path and file name for an input file for given day
pub fn get_input_file_name(day: i32) -> PathBuf {
    Path::new(format!("day{:0>2}", day).as_str()).join("input.txt")
}

/// Reads the input file for a given day into a string
pub fn read_input_file(day: i32) -> String {
    fs::read_to_string(get_input_file_name(day)).unwrap()
}

/// Reads the input file for a given day into a string
pub fn read_input_file_into_bytes(day: i32) -> Vec<u8> {
    let input_file_name = get_input_file_name(day);
    let mut f = File::open(input_file_name).unwrap();
    let md = f.metadata().unwrap();
    let mut result = Vec::<u8>::with_capacity(md.len() as usize);
    f.read_to_end(&mut result).unwrap();
    result
}

/// Prints colored day header
pub fn print_day_header(day: i32) {
    println!("{}{}Day {}:{}", style::Bold, color::Fg(color::Yellow), day, style::Reset);
}

/// Tests for star 1
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_digit() {
        let res = get_input_file_name(1);
        assert!(res.starts_with("day01"), "path = '{:?}'", res);
    }

    #[test]
    fn double_digit() {
        let res = get_input_file_name(10);
        assert!(res.starts_with("day10"), "path = '{:?}'", res);
    }

    #[test]
    fn read_day99_input() {
        let res = read_input_file(99);
        assert_eq!(res, "Test", "file content = '{:?}'", res);
    }

    #[test]
    fn read_day99_input_bytes() {
        let res = read_input_file_into_bytes(99);
        assert_eq!(4, res.len());
        assert_eq!(b'T', res[0]);
    }}