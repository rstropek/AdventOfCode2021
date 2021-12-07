use aoc_utils::{print_day_header, read_input_file};
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|v| v.parse().unwrap())
        .sorted()
        .collect()
}

fn calculate_fuel(values: &[i32]) -> i32 {
    let median = values[values.len() / 2];
    values
        .iter()
        .cloned()
        .fold(0, |acc, v| acc + (v - median).abs())
}

fn calculate_fuel_2(p1: i32, p2: i32) -> i32 {
    (p2 - p1).abs() * ((p2 - p1).abs() + 1) / 2
}

fn find_lowest_fuel(values: &[i32]) -> i32 {
    let mut min_fuel = i32::MAX;
    'outer: for i in values[0]..values[values.len() - 1] {
        let mut fuel = 0;
        for v in values.iter().cloned() {
            fuel += calculate_fuel_2(i, v);
            if fuel >= min_fuel {
                break 'outer;
            }
        }
        
        min_fuel = fuel;
    }

    min_fuel
}

fn main() {
    print_day_header(7);

    let input = read_input_file(7);
    let values = parse_input(&input);

    // Star 1
    let result = calculate_fuel(&values);
    println!("  Result Star 1: {:?}", result);

    // Star 2
    println!("  Result Star 2: {:?}", find_lowest_fuel(&values));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let values = parse_input(&"16,1,2,0,4,2,7,1,2,14");
        assert_eq!(37, calculate_fuel(&values));
    }

    #[test]
    fn test_calculate_fuel_2() {
        assert_eq!(66, calculate_fuel_2(16, 5));
        assert_eq!(66, calculate_fuel_2(5, 16));
        assert_eq!(0, calculate_fuel_2(5, 5));
        assert_eq!(1, calculate_fuel_2(4, 5));
        assert_eq!(1, calculate_fuel_2(5, 4));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_2() {
        let values = parse_input(&"16,1,2,0,4,2,7,1,2,14");
        assert_eq!(168, find_lowest_fuel(&values))
    }
}
