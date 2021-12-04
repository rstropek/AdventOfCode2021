use aoc_utils::{print_day_header, read_input_file};
use bit_vec::BitVec;

struct BingoInput {
    drawn_numbers: Vec<u16>,
    cards: Vec<[u16; 25]>,
}

impl BingoInput {
    /// Parses input
    fn from_input(input: &str) -> Self {
        let blocks: Vec<&str> = input.split("\n\n").collect();

        // Don't know how many numbers were drawn -> need split
        let drawn_numbers: Vec<u16> = blocks[0].split(',').map(|c| c.parse().unwrap()).collect();

        let mut cards = Vec::with_capacity(blocks.len() - 1);
        for block in blocks.iter().skip(1) {
            // We know exact structure of bingo cards -> no need for split.
            // We can calculate the exact indexes.
            const LINE_LENGTH: usize = 2 * 5 + 4;
            let mut lines: [&str; 5] = [Default::default(); 5];
            for c in 0..5 {
                lines[c] = &block[(LINE_LENGTH + 1) * c..(LINE_LENGTH + 1) * c + LINE_LENGTH]
            }

            let mut values = [0u16; 5 * 5];
            for li in 0..=4 {
                for vi in 0..=4 {
                    let val_str = &lines[li][(vi * 3)..(vi * 3 + 2)];
                    values[li * 5 + vi] = val_str.trim().parse().unwrap();
                }
            }

            cards.push(values);
        }

        BingoInput{drawn_numbers, cards}
    }

    /// Get sum of undrawn numbers
    fn sum_of_undrawn(&self, card_ix: usize, card: &BitVec) -> u16 {
        let mut sum = 0;
        for i in 0..25 {
            if !card[i] {
                sum += self.cards[card_ix][i];
            }
        }   

        sum
    }

    fn get_drawn_bitvec(&self) -> Vec<BitVec> {
        let mut drawn = Vec::with_capacity(self.cards.len());
        for _ in 0..self.cards.len() {
            drawn.push(BitVec::from_elem(5 * 5, false));
        }

        drawn
    }

    fn draw(&self) -> u16 {
        let mut drawn = self.get_drawn_bitvec();
        for v in self.drawn_numbers.iter().cloned() {
            for c in self.cards.iter().enumerate() {
                if let Some(p) = c.1.iter().position(|val| *val == v) {
                    drawn[c.0].set(p, true);
                }

                if check_win(&drawn[c.0]) {
                    return self.sum_of_undrawn(c.0, &drawn[c.0]) * v;
                }
            }
        }

        panic!()
    }
    
    fn draw_to_last_winner(mut self) -> u16 {
        let mut drawn = self.get_drawn_bitvec();

        for v in self.drawn_numbers.iter().cloned() {
            let mut i = 0;
            while i < self.cards.len() {
                if let Some(p) = self.cards[i].iter().position(|val| *val == v) {
                    drawn[i].set(p, true);
                }

                if check_win(&drawn[i]) {
                    if self.cards.len() == 1 {
                        return self.sum_of_undrawn(i, &drawn[i]) * v;
                    }

                    self.cards.remove(i);
                    drawn.remove(i);
                } else {
                    i += 1;
                }
            }
        }

        panic!()
    }
}

fn check_win(card: &BitVec) -> bool {
    fn get_index(row: usize, col: usize) -> usize {
        row * 5 + col
    }

    'rows: for row in 0..5 {
        for col in 1..5 {
            if !card[get_index(row, col)] || card[get_index(row, col)] != card[get_index(row, 0)] {
                continue 'rows;
            }
        }

        return true;
    }

    'cols: for col in 0..5 {
        for row in 1..5 {
            if !card[get_index(row, col)] || card[get_index(row, col)] != card[get_index(0, col)] {
                continue 'cols;
            }
        }

        return true;
    }

    false
}

fn main() {
    print_day_header(4);

    // Star 1
    let input = read_input_file(4);
    let input = BingoInput::from_input(&input);
    println!("  Result Star 1: {:?}", input.draw());

    // Star 2
    println!("  Result Star 2: {:?}", input.draw_to_last_winner());
}

#[cfg(test)]
const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = BingoInput::from_input(TEST_INPUT);
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            input.drawn_numbers
        );
        assert_eq!(3, input.cards.len());
        assert_eq!(vec![22, 13, 17, 11,  0], input.cards[0][..5]);
        assert_eq!(vec![14, 21, 16, 12, 6], input.cards[1][5 * 4..]);
    }

    #[test]
    fn test_winner_row() {
        let vec = BitVec::from_fn(25, |i| i / 5 == 1);
        assert!(check_win(&vec));

        let vec = BitVec::from_fn(25, |i| i / 5 == 0);
        assert!(check_win(&vec));
    }

    #[test]
    fn test_winner_col() {
        let vec = BitVec::from_fn(25, |i| i % 5 == 1);
        assert!(check_win(&vec));

        let vec = BitVec::from_fn(25, |i| i % 5 == 4);
        assert!(check_win(&vec));
    }

    #[test]
    fn test_no_winner() {
        let vec = BitVec::from_elem(25, false);
        assert!(!check_win(&vec));
    }

    #[test]
    fn test_drawing() {
        let input = BingoInput::from_input(TEST_INPUT);
        assert_eq!(4512, input.draw());
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_drawing() {
        let input = BingoInput::from_input(TEST_INPUT);
        assert_eq!(1924, input.draw_to_last_winner());
    }
}
