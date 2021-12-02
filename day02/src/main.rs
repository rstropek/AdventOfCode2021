use core::panic;
use std::ops::AddAssign;

use aoc_utils::{print_day_header, read_input_file};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// Regex for parsing input string
    static ref RE: Regex = Regex::new(r"(\w+) (\d+)").unwrap();
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Clone, Copy)]
struct Movement {
    direction: Direction,
    distance: i32,
}

impl Movement {
    fn new(direction: &str, distance: &str) -> Self {
        Self {
            direction: match direction {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => panic!(),
            },
            distance: distance.parse().unwrap(),
        }
    }
}

#[derive(Clone, Copy)]
struct Position {
    dx: i32,
    dy: i32,
}

impl Position {
    fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }

    fn from_movement(movement: &Movement) -> Self {
        Self {
            dx: match movement.direction {
                Direction::Forward => movement.distance,
                _ => 0,
            },
            dy: match movement.direction {
                Direction::Down => movement.distance,
                Direction::Up => -movement.distance,
                _ => 0,
            },
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.dx += rhs.dx;
        self.dy += rhs.dy;
    }
}

fn get_movements(contents: &'_ str) -> impl Iterator<Item = Movement> + '_ {
    RE.captures_iter(contents)
        .map(|c| Movement::new(&c[1], &c[2]))
}

fn get_position(movements: &[Movement]) -> i32 {
    let mut result: Position = Default::default();
    movements
        .iter()
        .map(|m| Position::from_movement(m))
        .for_each(|m| result += m);
    result.dx * result.dy
}

fn get_position_with_aim(movements: &[Movement]) -> i32 {
    let mut result: Position = Default::default();
    let mut aim = 0;
    movements.iter().for_each(|m| match m.direction {
        Direction::Down => aim += m.distance,
        Direction::Up => aim -= m.distance,
        Direction::Forward => result += Position::new(m.distance, aim * m.distance),
    });
    result.dx * result.dy
}

fn main() {
    print_day_header(2);

    // Star 1
    let movements: Vec<Movement> = get_movements(&read_input_file(2)).collect();
    println!("  Result Star 1: {:?}", get_position(&movements));

    // Star 2
    println!("  Result Star 2: {:?}", get_position_with_aim(&movements));
}

#[cfg(test)]
const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let movements: Vec<Movement> = get_movements(&String::from(TEST_INPUT)).collect();
        assert_eq!(150, get_position(&movements));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_2() {
        let movements: Vec<Movement> = get_movements(&String::from(TEST_INPUT)).collect();
        assert_eq!(900, get_position_with_aim(&movements));
    }
}
