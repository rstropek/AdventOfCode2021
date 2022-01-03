use std::fmt::Debug;

use aoc_utils::{print_day_header, read_input_file};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    East,
    South,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .split('\n')
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|b| match b {
                    b'.' => Cell::Empty,
                    b'>' => Cell::East,
                    b'v' => Cell::South,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn step(input: &[Vec<Cell>]) -> Option<Vec<Vec<Cell>>> {
    let mut result = vec![vec![Cell::Empty; input[0].len()]; input.len()];

    let mut moved_something = false;
    input.iter().enumerate().for_each(|row|
        (*row.1).iter().enumerate().filter(|col| *col.1 == Cell::East).for_each(|col| {
            let mut target_ix = (col.0 + 1) % row.1.len();
            if (*row.1)[target_ix] == Cell::Empty {
                moved_something |= true;
            } else {
                target_ix = col.0;
            }

            result[row.0][target_ix] = Cell::East;
        })
    );

    input.iter().enumerate().for_each(|row|
        (*row.1).iter().enumerate().filter(|col| *col.1 == Cell::South).for_each(|col| {
            let mut target_ix = (row.0 + 1) % input.len();
            if result[target_ix][col.0] == Cell::Empty && input[target_ix][col.0] != Cell::South {
                moved_something |= true;
            } else {
                target_ix = row.0;
            }

            result[target_ix][col.0] = Cell::South;
        })
    );

    if !moved_something {
        return None;
    }

    Some(result)
}

#[allow(dead_code)]
fn to_string(input: &[Vec<Cell>]) -> String {
    let mut result = String::new();

    for row in input.iter().enumerate() {
        for col in (*row.1).iter() {
            result.push(match *col {
                Cell::Empty => '.',
                Cell::East => '>',
                Cell::South => 'v',
            });
        }

        if row.0 < input.len() - 1 {
            result.push('\n');
        }
    }

    result
}

fn solve(mut input: Vec<Vec<Cell>>) -> i32 {
    let mut i = 1;
    loop {
        if let Some(result) = step(&input) {
            input = result;
            i += 1;
        } else {
            return i;
        }
    }
}

fn main() {
    print_day_header(25);

    // Star 1
    let input = parse_input(&read_input_file(25));
    println!("  Result Star 1: {:?}", solve(input));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input(".v>\nv>.");
        assert_eq!(2, result.len());
        assert_eq!(3, result[0].len());
        assert_eq!(3, result[1].len());
        assert_eq!(Cell::South, result[1][0]);
    }

    #[test]
    fn test_to_string() {
        let result = parse_input(".v>\nv>.");
        assert_eq!(".v>\nv>.", to_string(&result));
    }

    #[test]
    fn test_simple_step() {
        let input = parse_input("..........
.>v....v..
.......>..
..........");
        let result = step(&input);
        assert_eq!("..........
.>........
..v....v>.
..........", to_string(&result.unwrap()));
    }


    #[test]
    fn test_multi_step() {
        let input = parse_input("...>...
.......
......>
v.....>
......>
.......
..vvv..");
        let result = step(&input).unwrap();
        assert_eq!("..vv>..
.......
>......
v.....>
>......
.......
....v..", to_string(&result));
        let result = step(&result).unwrap();
        assert_eq!("....v>.
..vv...
.>.....
......>
v>.....
.......
.......", to_string(&result));
    }
    
    #[test]
    fn test_no_move() {
        let input = parse_input("..>>v>vv..
..v.>>vv..
..>>v>>vv.
..>>>>>vv.
v......>vv
v>v....>>v
vvv.....>>
>vv......>
.>v.vv.v..");
        assert!(step(&input).is_none());
    }
    
    #[test]
    fn test_stopping() {
        let input = parse_input("v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>");
        assert_eq!(58, solve(input));
    }
}
