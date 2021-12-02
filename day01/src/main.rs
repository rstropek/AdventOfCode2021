use aoc_utils::{print_day_header, read_input_file};

fn get_depths(contents: &'_ str) -> impl Iterator<Item = i32> + '_ {
    contents.lines().map(|d| d.parse().unwrap())
}

fn count_increases(depths: &[i32]) -> i32 {
    depths
        .iter()
        .enumerate()
        .map(|d| {
            if d.0 > 0 && *d.1 > depths[d.0 - 1] {
                1
            } else {
                0
            }
        })
        .sum()
}

fn get_windows(depths: &[i32]) -> Vec<i32> {
    depths[..depths.len() - 2]
        .iter()
        .enumerate()
        .map(|d| *d.1 + depths[d.0 + 1] + depths[d.0 + 2])
        .collect()
}

fn main() {
    print_day_header(1);

    // Star 1
    let depths: Vec<i32> = get_depths(&read_input_file(1)).collect();
    let increases = count_increases(&depths);
    println!("  Result Star 1: {:?}", increases);

    // Star 2
    let depth_windows = get_windows(&depths);
    let increases = count_increases(&depth_windows);
    println!("  Result Star 2: {:?}", increases);
}

#[cfg(test)]
const TEST_INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let d: Vec<i32> = get_depths(&String::from(TEST_INPUT)).collect();
        assert_eq!(count_increases(&d), 7);
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_2() {
        let d: Vec<i32> = get_depths(&String::from(TEST_INPUT)).collect();
        let w = get_windows(&d);
        assert_eq!(count_increases(&w), 5);
    }
}
