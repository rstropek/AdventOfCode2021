use aoc_utils::{print_day_header, read_input_file};

fn add(n1: &str, n2: &str) -> String {
    format!("[{},{}]", n1, n2)
}

#[derive(PartialEq, Eq, Debug)]
struct PairIx {
    left_ix: usize,
    right_ix: usize,
    end_ix: usize,
}

/// Reads next pair and returns bounding index
///
/// Arguments:
/// * `ix` must be the index of the opening braces ([) in front of the pair
///
/// Return value:
/// * `PairIx.right_ix` will be the first index of the second part of the pair
/// * `PairIx.end_id` will be the last index of the pair BEFORE the closing braces after it
fn read_pair(num_bytes: &[u8], mut ix: usize) -> PairIx {
    let left_ix = ix + 1;
    let mut right_ix = 0;
    let end_ix;
    let mut braces = 0;
    loop {
        ix += 1;

        if num_bytes[ix] == b',' && braces == 0 {
            right_ix = ix + 1;
        }

        if num_bytes[ix] == b'[' {
            braces += 1;
        }

        if num_bytes[ix] == b']' {
            if braces == 0 {
                end_ix = ix - 1;
                break;
            }

            braces -= 1;
        }
    }

    PairIx { left_ix, right_ix, end_ix }
}

#[derive(PartialEq, Eq, Debug)]
struct Value {
    value: u32,
    start_ix: usize,
    end_ix: usize,
}

fn read_value(num_bytes: &[u8], mut ix: usize) -> Value {
    // find beginning
    let original_ix = ix;
    let start_ix;
    loop {
        if !(b'0'..=b'9').contains(&num_bytes[ix]) {
            start_ix = ix + 1;
            break;
        }

        if ix == 0 {
            start_ix = ix;
            break;
        }

        ix -= 1;
    }

    // find end
    let end_ix;
    ix = original_ix + 1;
    loop {
        if ix == num_bytes.len() || !(b'0'..=b'9').contains(&num_bytes[ix]) {
            end_ix = ix - 1;
            break;
        }

        ix += 1;
    }

    let str_val = std::str::from_utf8(&num_bytes[start_ix..=end_ix]).unwrap();
    Value {
        value: str_val.parse().unwrap(),
        start_ix,
        end_ix,
    }
}

enum FindDirection {
    Left,
    Right,
}

fn find_num(num_bytes: &[u8], mut ix: usize, direction: FindDirection) -> Option<usize> {
    if num_bytes.is_empty() {
        return None;
    }

    loop {
        if (b'0'..=b'9').contains(&num_bytes[ix]) {
            return Some(ix);
        }

        match direction {
            FindDirection::Left => {
                if ix == 0 {
                    break;
                }

                ix -= 1;
            }
            FindDirection::Right => {
                if ix == num_bytes.len() - 1 {
                    break;
                }

                ix += 1;
            }
        }
    }

    None
}

fn try_explode(input: &str) -> (bool, String) {
    let mut num = String::from(input);
    let num_bytes = input.as_bytes();

    fn explode_update_num(num: String, ix: usize, src_val_ix: usize, direction: FindDirection) -> (String, usize) {
        let num_bytes = num.as_bytes();
        match find_num(num_bytes, ix, direction) {
            Some(num_ix) => {
                let dest_val = read_value(num_bytes, num_ix);
                let src_val = read_value(num_bytes, src_val_ix);
                let insert_val = (dest_val.value + src_val.value).to_string();
                (
                    format!("{}{}{}", &num[..dest_val.start_ix], (dest_val.value + src_val.value).to_string(), &num[dest_val.end_ix + 1..]),
                    insert_val.len() - (dest_val.end_ix - dest_val.start_ix + 1),
                )
            }
            _ => (num, 0),
        }
    }

    // Find four nested braces
    let mut braces = 0;
    let mut ix = 0;
    loop {
        if num_bytes[ix] == b'[' {
            braces += 1;
            if braces == 4 {
                // ix contains index of 4th opening brace
                let pair_ix = read_pair(num_bytes, ix);

                // Check if left is pair
                if num_bytes[ix + 1] == b'[' {
                    // left pair has to explode
                    let mut left_pair_ix = read_pair(num_bytes, ix + 1);
                    let ex = explode_update_num(num, ix - 1, ix + 2, FindDirection::Left);
                    num = ex.0;
                    ix += ex.1;
                    left_pair_ix.left_ix += ex.1;
                    left_pair_ix.right_ix += ex.1;
                    left_pair_ix.end_ix += ex.1;
                    let ex = explode_update_num(num, left_pair_ix.end_ix + 1, left_pair_ix.right_ix, FindDirection::Right);
                    num = ex.0;
                    num = format!("{}0{}", &num[..ix + 1], &num[left_pair_ix.end_ix + 2..]);
                    return (true, num);
                } else if num_bytes[pair_ix.right_ix] == b'[' {
                    // right has to explode
                    let mut right_pair_ix = read_pair(num_bytes, pair_ix.right_ix);
                    let ex = explode_update_num(num, pair_ix.right_ix - 1, pair_ix.right_ix + 1, FindDirection::Left);
                    num = ex.0;
                    right_pair_ix.left_ix += ex.1;
                    right_pair_ix.right_ix += ex.1;
                    right_pair_ix.end_ix += ex.1;
                    let ex = explode_update_num(num, right_pair_ix.end_ix + 1, right_pair_ix.right_ix, FindDirection::Right);
                    num = ex.0;
                    num = format!("{}0{}", &num[..right_pair_ix.left_ix - 1], &num[right_pair_ix.end_ix + 2..]);
                    return (true, num);
                }

                ix = pair_ix.right_ix + 1;
            }
        }
        if num_bytes[ix] == b']' {
            braces -= 1;
        }

        ix += 1;
        if ix == num_bytes.len() {
            break;
        }
    }

    (false, num)
}

fn try_split(input: &str) -> (bool, String) {
    let num = String::from(input);
    let num_bytes = input.as_bytes();
    let mut ix = 0;

    while let Some(num_ix) = find_num(num_bytes, ix, FindDirection::Right) {
        let val = read_value(num_bytes, num_ix);
        if val.value >= 10 {
            return (
                true,
                format!(
                    "{}[{},{}]{}",
                    &num[..num_ix],
                    (val.value as f32 / 2f32).floor(),
                    (val.value as f32 / 2f32).ceil(),
                    &num[val.end_ix + 1..]
                ),
            );
        } else {
            ix = val.end_ix + 1;
        }
    }

    (false, num)
}

fn process(input: &str) -> String {
    let mut num = String::from(input);
    loop {
        let mut res = try_explode(&num);
        num = res.1;
        if !res.0 {
            // No explosions -> try splits
            res = try_split(&num);
            num = res.1;
            if !res.0 {
                // Also no splits -> done
                break;
            }
        }
    }

    num
}

fn process_input(input: &str) -> (u32, String) {
    let data: Vec<&str> = input.split('\n').collect();
    let mut num = String::from(data[0]);
    for item in data.iter().skip(1) {
        num = add(&num, item);
        num = process(&num);
    }

    (magnitute(&num), num)
}

fn magnitute_impl(num_bytes: &[u8], mut ix: usize) -> (u32, usize) {
    if num_bytes[ix] == b'[' {
        let left = magnitute_impl(num_bytes, ix + 1);
        ix = left.1 + 1;
        let right = magnitute_impl(num_bytes, ix + 1);
        (left.0 * 3 + right.0 * 2, right.1 + 1)
    } else {
        let mut val = 0u32;
        loop {
            if !(b'0'..=b'9').contains(&num_bytes[ix]) {
                break;
            }

            val = val * 10 + (num_bytes[ix] - b'0') as u32;
            ix += 1;
        }

        (val, ix - 1)
    }
}

fn magnitute(input: &str) -> u32 {
    let num_bytes = input.as_bytes();
    magnitute_impl(num_bytes, 0).0
}

fn find_largest(input: &str) -> u32 {
    let data: Vec<&str> = input.split('\n').collect();
    let mut max = 0u32;
    for outer in 0..data.len() {
        for inner in 0..data.len() {
            if outer == inner {
                continue;
            }

            let mut num = add(data[outer], data[inner]);
            num = process(&num);
            let res = magnitute(&num);
            if res > max {
                max = res;
            }
        }
    }

    max
}

fn main() {
    print_day_header(18);

    let input = read_input_file(18);

    // Star 1
    println!("  Result Star 1: {:?}", process_input(&input).0);

    // Star 2
    println!("  Result Star 2: {:?}", find_largest(&input));
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_add() {
        const NUM1: &str = "[1,2]";
        const NUM2: &str = "[[3,4],5]";
        assert_eq!("[[1,2],[[3,4],5]]", add(NUM1, NUM2));
    }

    #[test]
    fn test_read_pair() {
        assert_eq!(PairIx { left_ix: 1, right_ix: 3, end_ix: 3 }, read_pair("[1,2]".as_bytes(), 0));
        assert_eq!(PairIx { left_ix: 1, right_ix: 7, end_ix: 8 }, read_pair("[[3,2],20]".as_bytes(), 0));
        assert_eq!(PairIx { left_ix: 4, right_ix: 7, end_ix: 13 }, read_pair("[1,[30,[20,10]]]".as_bytes(), 3));
    }

    #[test]
    fn test_read_value() {
        assert_eq!(Value { value: 100, start_ix: 0, end_ix: 2 }, read_value(b"100", 1));
        assert_eq!(Value { value: 100, start_ix: 0, end_ix: 2 }, read_value(b"100", 2));
        assert_eq!(Value { value: 100, start_ix: 0, end_ix: 2 }, read_value(b"100", 0));
        assert_eq!(Value { value: 750, start_ix: 5, end_ix: 7 }, read_value(b"[100,750]", 7));
        assert_eq!(Value { value: 750, start_ix: 5, end_ix: 7 }, read_value(b"[100,750]", 5));
        assert_eq!(Value { value: 100, start_ix: 1, end_ix: 3 }, read_value(b"[100,750]", 1));
    }

    #[test]
    fn test_find_num() {
        assert_eq!(None, find_num(b"", 0, FindDirection::Right));
        assert_eq!(Some(1), find_num(b"a1aa", 3, FindDirection::Left));
        assert_eq!(Some(2), find_num(b"aa10a", 0, FindDirection::Right));
        assert_eq!(Some(1), find_num(b"10aa", 3, FindDirection::Left));
        assert_eq!(Some(2), find_num(b"aa1", 0, FindDirection::Right));
        assert_eq!(None, find_num(b"aaa", 2, FindDirection::Left));
        assert_eq!(None, find_num(b"aaa", 1, FindDirection::Right));
    }

    #[test]
    fn test_explode() {
        assert_eq!((false, String::from("[[[9,8],1],2]")), try_explode(&"[[[9,8],1],2]"));
        assert_eq!((false, String::from("[[[[9,8],1],2],3]")), try_explode(&"[[[[9,8],1],2],3]"));
        assert_eq!((true, String::from("[[[[0,9],2],3],4]")), try_explode(&"[[[[[9,8],1],2],3],4]"));
        assert_eq!((true, String::from("[7,[6,[5,[7,0]]]]")), try_explode(&"[7,[6,[5,[4,[3,2]]]]]"));
        assert_eq!((true, String::from("[[6,[5,[7,0]]],3]")), try_explode(&"[[6,[5,[4,[3,2]]]],1]"));
        assert_eq!((true, String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")), try_explode(&"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
        assert_eq!((true, String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")), try_explode(&"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        assert_eq!((true, String::from("[[[[0,7],4],[15,[0,13]]],[1,1]]")), try_explode(&"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));
    }

    #[test]
    fn test_split() {
        assert_eq!((true, String::from("[[5,5],1]")), try_split(&"[10,1]"));
        assert_eq!((false, String::from("[1,1]")), try_split(&"[1,1]"));
        assert_eq!((true, String::from("[[5,5],10]")), try_split(&"[10,10]"));
    }

    #[test]
    fn test_process() {
        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", process(&"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
    }

    #[test]
    fn test_magnitute() {
        assert_eq!(29, magnitute("[9,1]"));
        assert_eq!(21, magnitute("[1,9]"));
        assert_eq!(129, magnitute("[[9,1],[1,9]]"));
        assert_eq!(143, magnitute("[[1,2],[[3,4],5]]"));
        assert_eq!(1384, magnitute("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        assert_eq!(445, magnitute("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
        assert_eq!(791, magnitute("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
        assert_eq!(1137, magnitute("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
        assert_eq!(3488, magnitute("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

    #[test]
    fn test_process_input() {
        let res = process_input(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        );
        assert_eq!(4140, res.0);
        assert_eq!("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", res.1);
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_to_points() {

        assert_eq!(3993, find_largest(&"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"));
    }
}
