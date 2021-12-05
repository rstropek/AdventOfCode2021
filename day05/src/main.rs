use std::collections::HashMap;

use aoc_utils::{print_day_header, read_input_file};
use genawaiter::{sync::gen, yield_};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for parsing input string
    static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn parse_input(input: &'_ str) -> impl Iterator<Item = Line> + '_ {
    // Use regex capture groups to parse input
    RE.captures_iter(input).map(|l| Line {
        start: Point {
            x: l[1].parse().unwrap(),
            y: l[2].parse().unwrap(),
        },
        end: Point {
            x: l[3].parse().unwrap(),
            y: l[4].parse().unwrap(),
        },
    })
}

fn filter_only_straight(input: impl Iterator<Item = Line>) -> impl Iterator<Item = Line> {
    input.filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
}

fn to_points(input: impl Iterator<Item = Line>) -> impl Iterator<Item = Point> {
    input.flat_map(|l| {
        gen!({
            let mut p = l.start;
            loop {
                yield_!(p);
                if p == l.end {
                    break;
                }

                if p.x != l.end.x {
                    p.x += if l.start.x < l.end.x { 1 } else { -1 };
                }

                if p.y != l.end.y {
                    p.y += if l.start.y < l.end.y { 1 } else { -1 };
                }
            }
        })
    })
}

fn count_points(input: impl Iterator<Item = Point>) -> HashMap<Point, usize> {
    input.counts_by(|p| p)
}

fn count_overlap_points(input: HashMap<Point, usize>) -> usize {
    input.into_values().filter(|v| *v >= 2).count()
}

fn main() {
    print_day_header(5);

    let input = read_input_file(5);
    let input: Vec<Line> = parse_input(&input).collect();

    // Star 1
    let points_stat = count_points(to_points(filter_only_straight(input.iter().cloned())));
    println!("  Result Star 1: {:?}", count_overlap_points(points_stat));

    // Star 2
    let points_stat = count_points(to_points(input.into_iter()));
    println!("  Result Star 2: {:?}", count_overlap_points(points_stat));
}

#[cfg(test)]
const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input: Vec<Line> = parse_input(TEST_INPUT).collect();
        assert_eq!(0, input[0].start.x);
        assert_eq!(0, input[1].start.y);
        assert_eq!(3, input[2].end.x);
        assert_eq!(1, input[3].end.y);
    }

    #[test]
    fn test_filter() {
        let input: Vec<Line> = vec![
            Line {
                start: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 2 },
            },
            Line {
                start: Point { x: 0, y: 0 },
                end: Point { x: 1, y: 1 },
            },
        ];
        let input = filter_only_straight(input.into_iter());
        assert_eq!(1, input.count());
    }

    #[test]
    fn test_to_points() {
        let input = filter_only_straight(parse_input(TEST_INPUT));
        let input = to_points(input.into_iter());
        assert_eq!(26, input.count());
    }

    #[test]
    fn test_count_points() {
        let input: Vec<Point> = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 2 },
        ];
        let input = count_points(input.into_iter());
        assert_eq!(2, input[&Point { x: 0, y: 0 }]);
    }

    #[test]
    fn test_1() {
        let points_stat = count_points(to_points(filter_only_straight(parse_input(TEST_INPUT))));
        assert_eq!(5, count_overlap_points(points_stat));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_to_points() {
        let input = parse_input(TEST_INPUT);
        let input = to_points(input.into_iter());
        assert_eq!(53, input.count());
    }

    #[test]
    fn test_2() {
        let points_stat = count_points(to_points(parse_input(TEST_INPUT)));
        assert_eq!(12, count_overlap_points(points_stat));
    }
}
