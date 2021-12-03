use aoc_utils::{print_day_header, read_input_file};
use num::PrimInt;

fn get_numbers<T: PrimInt + Default>(contents: &'_ str) -> (Vec<T>, u32) {
    // Number of bits (=length of first line)
    let mut bits = 0;
    let result = contents
        .lines()
        .map(|c| {
            if bits == 0 {
                bits = c.len() as u32;
            }
            T::from_str_radix(c, 2).unwrap_or_default()
        })
        .collect();
    (result, bits)
}

enum BitResult {
    Equal,
    MoreOnes,
    MoreZeroes,
}

fn find_majority_bit<T: PrimInt + Clone>(numbers: &[T], bit: u32) -> BitResult {
    let mut ones: usize = 0;
    for n in numbers.iter().cloned().enumerate() {
        if n.1.unsigned_shr(bit) & T::from(1).unwrap() == T::from(1).unwrap() {
            ones += 1;
        }
    }

    if ones as f32 == numbers.len() as f32 / 2f32 {
        BitResult::Equal
    } else if ones > numbers.len() / 2 {
        BitResult::MoreOnes
    } else {
        BitResult::MoreZeroes
    }
}

fn oxygen_filter<T: PrimInt>(numbers: &[T], bit: u32) -> T {
    match find_majority_bit(numbers, bit) {
        BitResult::Equal => T::from(1).unwrap(),
        BitResult::MoreOnes => T::from(1).unwrap(),
        BitResult::MoreZeroes => T::from(0).unwrap(),
    }
}

fn co2_filter<T: PrimInt>(numbers: &[T], bit: u32) -> T {
    match find_majority_bit(numbers, bit) {
        BitResult::Equal => T::from(0).unwrap(),
        BitResult::MoreOnes => T::from(0).unwrap(),
        BitResult::MoreZeroes => T::from(1).unwrap(),
    }
}

fn aggregate<T: PrimInt>(numbers: &[T], bits: u32) -> (T, T) {
    let mut gamma: T = T::from(0).unwrap();
    for i in (0..bits).rev() {
        if let BitResult::MoreOnes = find_majority_bit(numbers, i) {
            gamma = gamma | T::from(1 << i).unwrap();
        }
    }

    (gamma, !gamma & T::from((1 << bits) - 1).unwrap())
}

fn filter<T: PrimInt>(mut numbers: Vec<T>, bits: u32, f: fn(&[T], u32) -> T) -> T {
    for i in (0..bits).rev() {
        let r = f(&numbers, i);
        numbers = numbers
            .into_iter()
            .filter(|n| ((*n).unsigned_shr(i)) & T::from(1).unwrap() == r)
            .collect();
        if numbers.len() == 1 {
            break;
        }
    }

    numbers[0]
}

fn main() {
    print_day_header(3);

    // Star 1
    let input = read_input_file(3);
    let (numbers, bits) = get_numbers::<u16>(&input);
    let result = aggregate(&numbers, bits);
    println!("  Result Star 1: {:?}", result.0 as u32 * result.1 as u32);

    // Star 2
    let oxygen = filter(numbers.clone(), bits, oxygen_filter);
    let co2 = filter(numbers, bits, co2_filter);
    println!("  Result Star 2: {:?}", oxygen as u32 * co2 as u32);
}

#[cfg(test)]
const TEST_INPUT: &str =
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_1() {
        let (numbers, bits) = get_numbers::<u16>(TEST_INPUT);
        let result = aggregate(&numbers, bits);
        assert_eq!((0b10110, 0b01001), result);
        assert_eq!(198, result.0 as u32 * result.1 as u32);
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_oxygen() {
        let (numbers, bits) = get_numbers::<u16>(TEST_INPUT);
        let result = filter(numbers, bits, oxygen_filter);
        assert_eq!(0b10111, result);
    }

    #[test]
    fn test_co2() {
        let (numbers, bits) = get_numbers::<u16>(TEST_INPUT);
        let result = filter(numbers, bits, co2_filter);
        assert_eq!(0b01010, result);
    }

    #[test]
    fn test_result() {
        let (numbers, bits) = get_numbers::<u16>(TEST_INPUT);
        let oxygen = filter(numbers.clone(), bits, oxygen_filter);
        let co2 = filter(numbers, bits, co2_filter);
        assert_eq!(230, oxygen as i32 * co2 as i32);
    }
}
