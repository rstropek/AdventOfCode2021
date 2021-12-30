use std::collections::{HashMap, hash_map::Entry::{Occupied, Vacant}};

use aoc_utils::print_day_header;

// Trivial solution; could be enhanced but isn't worth it as the real challenge is second star.
fn solve_1(mut start_1: u32, mut start_2: u32) -> (u32, u32) {
    let mut dice = 1u32;
    let mut p1 = 0u32;
    let mut p2 = 0u32;

    loop {
        start_1 = (start_1 + dice + dice + 1 + dice + 2) % 10;
        dice += 3;
        p1 += start_1 + 1;
        if p1 >= 1000 {
            return (p2, dice - 1);
        }

        start_2 = (start_2 + dice + dice + 1 + dice + 2) % 10;
        dice += 3;
        p2 += start_2 + 1;
        if p2 >= 1000 {
            return (p1, dice - 1);
        }
    }
}

/// Calculates the number of dice combinations to reach a certain dice sum.
fn number_of_possibilities() -> HashMap<u8, u64> {
    let mut possiblities_per_sum = HashMap::<u8, u64>::with_capacity(7);
    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                match possiblities_per_sum.entry(d1 + d2 + d3) {
                    Occupied(mut e) => { e.insert(e.get() + 1); },
                    Vacant(e) => { e.insert(1); }
                };
            }
        }
    }

    possiblities_per_sum
}

fn roll(possiblities_per_sum: &HashMap<u8, u64>, possibilities: u64, player: usize, pos: [u8; 2], points: [u8; 2], universes: &mut [u64; 2], round: u8) {
    for dice_sum in 3..=9 {
        let mut pos = pos;
        let mut points = points;
        let mut possibilities = possibilities;
        pos[player] = (pos[player] - 1 + dice_sum) % 10 + 1;
        points[player] += pos[player];
        possibilities *= possiblities_per_sum[&dice_sum];
        if points[player] >= 21 {
            universes[player] += possibilities;
        } else {
            roll(possiblities_per_sum, possibilities, (player + 1) % 2, pos, points, universes, round + 1);
        }
    }
}

fn main() {
    print_day_header(21);

    // Star 1
    let result = solve_1(4 - 1, 7 - 1);
    println!("  Result Star 1: {:?}", result.0 * result.1);

    // Star 2
    let pos = [4u8, 7];
    let points = [0u8; 2];
    let possibilities = 1u64;
    let mut universes = [0u64; 2];
    roll(&number_of_possibilities(), possibilities, 0, pos, points, &mut universes, 1);
    println!("  Result Star 2: {:?}", if universes[0] > universes[1] { universes[0] } else { universes[1] });
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let result = solve_1(4 - 1, 8 - 1);
        assert_eq!(739785, result.0 * result.1);
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_2() {
        let pos = [4u8, 8];
        let points = [0u8; 2];
        let possibilities = 1u64;
        let mut universes = [0u64; 2];
        roll(&number_of_possibilities(), possibilities, 0, pos, points, &mut universes, 1);

        assert_eq!(444356092776315, universes[0]);
        assert_eq!(341960390180808, universes[1]);
    }
}
