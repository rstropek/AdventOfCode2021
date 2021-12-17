use aoc_utils::{print_day_header};

fn solve_1(min_x: i32) -> i32 {
    let y = min_x.abs() - 1;
    y * (y + 1) / 2
}

fn solve_2(min: (i32, i32), max: (i32, i32)) -> i32 {
    let range_x = min.0..=max.0;
    let range_y = min.1..=max.1;
    let mut count = 0;
    for x in 0..=max.0 {
        for y in min.1..=min.1.abs() - 1 {
            let mut t = 1;
            loop {
                let mut xt = 0;
                let mut xc = x;
                let mut yt = 0;
                let mut yc = y;
                for _ in 0..t {
                    xt += xc;
                    yt += yc;
                    if xc > 0 {
                        xc -= 1;
                    }
                    yc -= 1;
                }

                if xt > max.0 || yt < min.1 {
                    break;
                }

                if range_x.contains(&xt) && range_y.contains(&yt) {
                    count += 1;
                    break;
                }

                t += 1;
            }
        }
    }

    count
}

fn main() {
    print_day_header(17);

    // Star 1
    println!("  Result Star 1: {:?}", solve_1(-75));

    // Star 2
    println!("  Result Star 2: {:?}", solve_2((241, -75), (275, -49)));
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_solve_2() {
        let res = solve_2((20, -10), (30, -5));
        assert_eq!(112, res);
    }
}
