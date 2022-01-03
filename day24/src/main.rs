use std::collections::HashMap;

use aoc_utils::print_day_header;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn main() {
    print_day_header(24);

    // Star 1
    let mut res = HashMap::<i64, i64>::new();
    for d in 1..=9 {
        res.insert(d, CALCULATORS[CALCULATORS.len() - 14](0, d));
    }

    let mut max = 26;
    for c in (0..=12).rev() {
        max = match c {
            12 => max * 26,
            11 => max * 26,
            10 => max / 26,
            9 => max / 26,
            8 => max * 26,
            7 => max * 26,
            6 => max * 26,
            5 => max / 26,
            4 => max / 26,
            3 => max / 26,
            2 => max * 26,
            1 => max / 26,
            0 => max / 26,
            _ => panic!()
        };
        let mut new_res = HashMap::<i64, i64>::new();
        for r in res {
            for d in 1..=9 {
                let check = r.0 * 10 + d;
                let z = CALCULATORS[CALCULATORS.len() - 1 - c](r.1, d);
                if z < max {
                    new_res.insert(check, z);
                }
            }
        }

        res = new_res;
    }

    let monads: Vec<i64> = res.into_iter().filter(|item| item.1 == 0i64).map(|item| item.0).collect();
    println!("  Result Star 1: {:?}", monads.iter().max().unwrap());

    // Star 2
    println!("  Result Star 2: {:?}", monads.iter().min().unwrap());
}
