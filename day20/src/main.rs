use aoc_utils::{print_day_header, read_input_file};
use bitvec::prelude::*;

struct ImageProcessor {
    enhancement: BitVec,
    image: BitVec,
    width: usize,
    height: usize,
    outer: bool,
}

fn parse_input(input: &str) -> ImageProcessor {
    let mut sections = input.split("\n\n");

    let enhancement_str = sections.next().unwrap().as_bytes();
    let mut enhancement = bitvec![0; enhancement_str.len()];
    for e in enhancement_str.iter().cloned().enumerate() {
        if e.1 == b'#' {
            enhancement.set(e.0, true);
        }
    }

    let image_lines: Vec<&str> = sections.next().unwrap().split('\n').collect();
    let height = image_lines.len();
    let width = image_lines[0].len();
    let mut image = bitvec![0;  width * height];
    let mut ix = 0;
    for l in image_lines {
        for p in l.as_bytes() {
            if *p == b'#' {
                image.set(ix, true);
            }

            ix += 1;
        }
    }

    ImageProcessor {
        enhancement,
        image,
        width,
        height,
        outer: false,
    }
}

impl ImageProcessor {
    #[allow(dead_code)]
    fn print(&self) {
        let outer_char = if self.outer { "#" } else { "." };

        fn print_outer_lines(width: usize, outer_char: &str, mut ix: usize) {
            for _ in 0..2 {
                print!("{: >3}: ", ix);
                ix += 1;
                for _ in 0..width + 4 {
                    print!("{}", outer_char);
                }
                println!();
            }
        }

        print_outer_lines(self.width, outer_char, 0);

        for row in 0..self.height {
            print!("{: >3}: ", 2 + row);
            for col in 0..self.width + 4 {
                if col < 2 || col >= 2 + self.width {
                    print!("{}", outer_char);
                    continue;
                }

                print!("{}", if self.image[row * self.width + col - 2] { "#" } else { "." });
            }
            println!();
        }

        print_outer_lines(self.width, outer_char, 2 + self.height);
    }

    fn enhance(&mut self) {
        let new_width = self.width + 2;
        let new_height = self.height + 2;
        let mut new_image = bitvec![0;  new_width * new_height];
        for row in 0..self.height + 2 {
            for col in 0..self.width + 2 {
                let mut ix = 0usize;
                for inner_row in 0..3 {
                    for inner_col in 0..3 {
                        ix <<= 1;
                        let is_one;
                        if (row + inner_row) < 2 || (row + inner_row) >= self.height + 2 || (col + inner_col) < 2 || (col + inner_col) >= self.width + 2 {
                            is_one = self.outer;
                        } else {
                            is_one = self.image[(row + inner_row - 2) * self.width + (col + inner_col - 2)];
                        }
                        ix |= if is_one { 1 } else { 0 };
                    }
                }

                if self.enhancement[ix] {
                    new_image.set(row * (self.width + 2) + col, true);
                }
            }
        }

        let new_outer;
        if self.outer {
            new_outer = self.enhancement[(1 << 9) - 1];
        } else {
            new_outer = self.enhancement[0];
        }

        self.image = new_image;
        self.outer = new_outer;
        self.width = new_width;
        self.height = new_height;
    }

    fn count_lit(&self) -> u32 {
        let mut counter = 0u32;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.image[row * self.width + col] {
                    counter += 1;
                }
            }
        }

        counter
    }
}

fn main() {
    print_day_header(19);

    let mut input = parse_input(&read_input_file(20));

    // Star 1
    input.enhance();
    input.enhance();
    println!("  Result Star 1: {:?}", input.count_lit());

    // Star 2
    for _ in 0..48 {
        input.enhance();
    }
    println!("  Result Star 2: {:?}", input.count_lit());
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    const TEST_DATA: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_parse() {
        let input = parse_input(TEST_DATA);

        assert_eq!(5 * 5, input.image.len());
        assert_eq!(512, input.enhancement.len());
        assert!(input.enhancement[2]);
        assert!(!input.enhancement[1]);
        assert!(input.image[0]);
        assert!(!input.image[1]);
    }

    #[test]
    fn test_enhance_1() {
        let mut input = parse_input(TEST_DATA);
        input.enhance();
        //input.print();
        input.enhance();
        //input.print();

        assert_eq!(35, input.count_lit());
    }
}
