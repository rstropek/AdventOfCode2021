use aoc_utils::{print_day_header, read_input_file};

fn parse_input(input: &str) -> [u64; 9] {
    let mut fishes = [0u64; 9];
    for f in input.split(',') {
        fishes[f.parse::<usize>().unwrap()] += 1;
    }

    fishes
}

fn calculate(mut fishes: [u64; 9], iterations: usize) -> u64 {
    for _ in 0..iterations {
        let mut new_fishes = [0; 9];
        new_fishes[6] = fishes[0];
        new_fishes[8] = fishes[0];
        for j in 1..9 {
            new_fishes[j - 1] += fishes[j];
        }

        fishes = new_fishes;
    }

    fishes.into_iter().sum()
}

fn main() {
    print_day_header(6);

    let input = read_input_file(6);
    let fishes = parse_input(&input);

    // Star 1
    println!("  Result Star 1: {:?}", calculate(fishes, 80));

    // Star 2
    println!("  Result Star 2: {:?}", calculate(fishes, 256));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let fishes = parse_input(&"3,4,3,1,2");
        assert_eq!(26, calculate(fishes, 18));
        assert_eq!(5934, calculate(fishes, 80));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_2() {
        let fishes = parse_input(&"3,4,3,1,2");
        assert_eq!(26984457539, calculate(fishes, 256));
    }
}
