use std::{collections::HashSet};

use crate::utils::Vector3d;
use crate::utils::*;
use aoc_utils::{print_day_header, read_input_file};

mod utils;

fn parse_input(input: &str) -> Vec<ScannerData> {
    input.split("\n\n").map(|s| ScannerData::new(s.split('\n').skip(1).map(|si| si.into()).collect())).collect()
}

fn solve(input: &mut [ScannerData]) -> HashSet<Vector3d> {
    // s = all sensors except 0
    let mut s = HashSet::from_iter(1..input.len());
    let mut beacons = HashSet::from_iter(input[0].beacons.iter().cloned());

    // s_new = [s0]
    let mut s_new = HashSet::new();
    s_new.insert(0);

    loop {
        // s_new = all sensors in s with overlap to s_new
        let mut s_new_new = HashSet::new();
        'outer: for snix in s_new.iter().cloned() {
            for six in s.iter().cloned() {
                if let Some(mut translation) = get_required_translation(&input[snix], &input[six], 12) {
                    let mut translated_beacons = translation.translate_set(&input[six].beacons);

                    input[six].translations_to_zero = input[snix].translations_to_zero.clone();
                    input[six].translations_to_zero.insert(0, translation.clone());
                    for trans in input[snix].translations_to_zero.iter() {
                        translation.movement = trans.rotate(translation.movement);
                        translated_beacons = trans.translate_set(&translated_beacons);
                    }

                    input[six].movement_to_zero = input[snix].movement_to_zero + translation.movement;
                    s_new_new.insert(six);

                    beacons = HashSet::from_iter(beacons.union(&translated_beacons).cloned());

                    // Check whether we have found all matches
                    if s_new_new.len() == s.len() {
                        break 'outer;
                    }
                }
            }
        }

        // if s_new is empty: panic!
        if s_new_new.is_empty() {
            panic!("No overlap found!");
        }

        s_new = s_new_new;

        // remove s_new from s
        s = HashSet::from_iter(s.difference(&s_new).cloned());
        if s.is_empty() {
            break;
        }
    }

    beacons
}

fn get_required_translation(s1: &ScannerData, s2: &ScannerData, threashold: usize) -> Option<Translation> {
    // Check if we have at least the minimum amount of equal distances. If we don't, we cannot have a match
    let min_equal_distances = threashold * (threashold - 1) / 2;
    if s1.distances.intersection(&s2.distances).count() < min_equal_distances {
        return None;
    }

    for s1b in s1.beacons.iter().cloned() {
        for s2b in s2.beacons.iter().cloned() {
            for r in ROTATORS.iter().enumerate() {
                // Move s2 so that s2b overlaps with s1b
                let s2br = r.1(s2b);
                let translation = Translation::new(s2br - s1b, r.0);
                let s2_translated = translation.translate_set(&s2.beacons);

                // Check overlapping points
                let mut intersection_size = 0;
                let mut checked = 0;
                for s2mb in s2_translated.iter().cloned() {
                    if s1.beacons.contains(&s2mb) {
                        intersection_size += 1;
                    }

                    checked += 1;

                    if s2_translated.len() - checked + intersection_size < threashold {
                        // Too few items left, we cannot reach threashold anymore
                        break;
                    }

                    // If overlapping points are >= threashold, we have found a match
                    if intersection_size >= threashold {
                        // Put all beacons from s2 (moved positions) into result
                        return Some(translation);
                    }
                }
            }
        }
    }

    None
}

fn get_max_manhattan_distance(input: &[ScannerData]) -> i32 {
    let mut max = 0;
    for s1 in input {
        for s2 in input {
            let dist = s1.movement_to_zero.manhattan_distance(&s2.movement_to_zero);
            if dist > max {
                max = dist;
            }
        }
    }

    max
}

fn main() {
    print_day_header(19);

    let mut input = parse_input(&read_input_file(19));
    let solution = solve(&mut input);

    // Star 1
    println!("  Result Star 1: {:?}", solution.len());

    // Star 2
    println!("  Result Star 2: {:?}", get_max_manhattan_distance(&input));
}

#[cfg(test)]
mod test_data;

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use crate::test_data::TEST_BEACONS;

    use super::*;

    #[test]
    fn test_parse_input() {
        let result = parse_input(
            "--- scanner 0 ---
404,-588,-901
528,-643,409

--- scanner 1 ---
686,422,578
605,423,415

--- scanner 2 ---
649,640,665
682,-795,504",
        );
        assert_eq!(3, result.len());
        assert_eq!(2, result[0].beacons.len());
        assert_eq!(2, result[1].beacons.len());
        assert_eq!(2, result[2].beacons.len());
        assert!(result[0].beacons.contains(&v3!(404, -588, -901)));
        assert!(result[1].beacons.contains(&v3!(605, 423, 415)));
        assert!(result[2].beacons.contains(&v3!(682, -795, 504)));
        assert_eq!(result[0].distances.len(), 2 * 1 / 2);
        assert_eq!(result[1].distances.len(), 2 * 1 / 2);
        assert_eq!(result[2].distances.len(), 2 * 1 / 2);
        assert!(result[0].distances.contains(&131700455));
    }

    #[test]
    fn test_simple_translation() {
        let s1 = ScannerData::new(vec![v3![1, 1, 1], v3![2, 2, 2], v3![6, 6, 6]]);
        let s2 = ScannerData::new(vec![v3![2, -2, 2], v3![3, -3, 3], v3![7, -7, 7]]);

        let translation = get_required_translation(&s1, &s2, 2).unwrap();
        let s2_translated = translation.translate_set(&s2.beacons);
        assert!(s2_translated.iter().filter(|b| s1.beacons.contains(b)).count() >= 2);
    }

    #[test]
    fn test_no_overlap() {
        let scanners = parse_input(
            "--- scanner 0 ---
1,1,1
2,2,2
6,6,6

--- scanner 1 ---
3,3,3
9,9,9
15,15,15",
        );
        assert!(get_required_translation(&scanners[0], &scanners[1], 2).is_none());
    }

    #[test]
    fn test_1_1() {
        let result = parse_input(TEST_BEACONS);
        let translate_1_to_0 = get_required_translation(&result[0], &result[1], 12);
        if let Some(translate_1_to_0) = translate_1_to_0 {
            let translated_sensor2 = translate_1_to_0.translate_set(&result[1].beacons);
            let intersection = HashSet::<Vector3d>::from_iter(result[0].beacons.intersection(&translated_sensor2).cloned());
            assert_eq!(12, intersection.len());
            assert!(intersection.contains(&v3![-618, -824, -621]));
            assert!(intersection.contains(&v3![-537, -823, -458]));
            assert!(intersection.contains(&v3![-447, -329, 318]));
            assert!(intersection.contains(&v3![404, -588, -901]));
            assert!(intersection.contains(&v3![544, -627, -890]));
            assert!(intersection.contains(&v3![528, -643, 409]));
            assert!(intersection.contains(&v3![-661, -816, -575]));
            assert!(intersection.contains(&v3![390, -675, -793]));
            assert!(intersection.contains(&v3![423, -701, 434]));
            assert!(intersection.contains(&v3![-345, -311, 381]));
            assert!(intersection.contains(&v3![459, -707, 401]));
            assert!(intersection.contains(&v3![-485, -357, 347]));
            assert_eq!(v3![-68, 1246, 43], translate_1_to_0.movement);
            assert_eq!(4, translate_1_to_0.rotator_ix);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_1_2() {
        let result = parse_input(TEST_BEACONS);
        let translate_1_to_0 = get_required_translation(&result[0], &result[1], 12).unwrap();
        let translate_4_to_1 = get_required_translation(&result[1], &result[4], 12);
        if let Some(translate_4_to_1) = translate_4_to_1 {
            let translated_sensor4 = translate_4_to_1.translate_set(&result[4].beacons);
            let intersection = HashSet::<Vector3d>::from_iter(result[1].beacons.intersection(&translated_sensor4).cloned());
            let intersection = translate_1_to_0.translate_set(&intersection);
            assert_eq!(12, intersection.len());
            assert!(intersection.contains(&v3![459, -707, 401]));
            assert!(intersection.contains(&v3![-739, -1745, 668]));
            assert!(intersection.contains(&v3![-485, -357, 347]));
            assert!(intersection.contains(&v3![432, -2009, 850]));
            assert!(intersection.contains(&v3![528, -643, 409]));
            assert!(intersection.contains(&v3![423, -701, 434]));
            assert!(intersection.contains(&v3![-345, -311, 381]));
            assert!(intersection.contains(&v3![408, -1815, 803]));
            assert!(intersection.contains(&v3![534, -1912, 768]));
            assert!(intersection.contains(&v3![-687, -1600, 576]));
            assert!(intersection.contains(&v3![-447, -329, 318]));
            assert!(intersection.contains(&v3![-635, -1737, 486]));
            let movement_4_to_1_from_0 = translate_1_to_0.rotate(translate_4_to_1.movement);
            assert_eq!(v3![20, 1133, -1061], translate_1_to_0.movement + movement_4_to_1_from_0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solve() {
        let mut result = parse_input(TEST_BEACONS);
        let beacons = solve(&mut result);

        assert_eq!(v3![-68, 1246, 43], result[1].movement_to_zero);
        assert_eq!(v3![-1105, 1205, -1229], result[2].movement_to_zero);
        assert_eq!(v3![92, 2380, 20], result[3].movement_to_zero);
        assert_eq!(v3![20, 1133, -1061], result[4].movement_to_zero);

        assert_eq!(79, beacons.len());
        assert!(beacons.contains(&v3![-892,524,684]));
        assert!(beacons.contains(&v3![-876,649,763]));
        assert!(beacons.contains(&v3![-838,591,734]));
        assert!(beacons.contains(&v3![-789,900,-551]));
        assert!(beacons.contains(&v3![-739,-1745,668]));
        assert!(beacons.contains(&v3![-706,-3180,-659]));
        assert!(beacons.contains(&v3![-697,-3072,-689]));
        assert!(beacons.contains(&v3![-689,845,-530]));
        assert!(beacons.contains(&v3![-687,-1600,576]));
        assert!(beacons.contains(&v3![-661,-816,-575]));
        assert!(beacons.contains(&v3![-654,-3158,-753]));
        assert!(beacons.contains(&v3![-635,-1737,486]));
        assert!(beacons.contains(&v3![-631,-672,1502]));
        assert!(beacons.contains(&v3![-624,-1620,1868]));
        assert!(beacons.contains(&v3![-620,-3212,371]));
        assert!(beacons.contains(&v3![-618,-824,-621]));
        assert!(beacons.contains(&v3![-612,-1695,1788]));
        assert!(beacons.contains(&v3![-601,-1648,-643]));
        assert!(beacons.contains(&v3![-584,868,-557]));
        assert!(beacons.contains(&v3![-537,-823,-458]));
        assert!(beacons.contains(&v3![-532,-1715,1894]));
        assert!(beacons.contains(&v3![-518,-1681,-600]));
        assert!(beacons.contains(&v3![-499,-1607,-770]));
        assert!(beacons.contains(&v3![-485,-357,347]));
        assert!(beacons.contains(&v3![-470,-3283,303]));
        assert!(beacons.contains(&v3![-456,-621,1527]));
        assert!(beacons.contains(&v3![-447,-329,318]));
        assert!(beacons.contains(&v3![-430,-3130,366]));
        assert!(beacons.contains(&v3![-413,-627,1469]));
        assert!(beacons.contains(&v3![-345,-311,381]));
        assert!(beacons.contains(&v3![-36,-1284,1171]));
        assert!(beacons.contains(&v3![-27,-1108,-65]));
        assert!(beacons.contains(&v3![7,-33,-71]));
        assert!(beacons.contains(&v3![12,-2351,-103]));
        assert!(beacons.contains(&v3![26,-1119,1091]));
        assert!(beacons.contains(&v3![346,-2985,342]));
        assert!(beacons.contains(&v3![366,-3059,397]));
        assert!(beacons.contains(&v3![377,-2827,367]));
        assert!(beacons.contains(&v3![390,-675,-793]));
        assert!(beacons.contains(&v3![396,-1931,-563]));
        assert!(beacons.contains(&v3![404,-588,-901]));
        assert!(beacons.contains(&v3![408,-1815,803]));
        assert!(beacons.contains(&v3![423,-701,434]));
        assert!(beacons.contains(&v3![432,-2009,850]));
        assert!(beacons.contains(&v3![443,580,662]));
        assert!(beacons.contains(&v3![455,729,728]));
        assert!(beacons.contains(&v3![456,-540,1869]));
        assert!(beacons.contains(&v3![459,-707,401]));
        assert!(beacons.contains(&v3![465,-695,1988]));
        assert!(beacons.contains(&v3![474,580,667]));
        assert!(beacons.contains(&v3![496,-1584,1900]));
        assert!(beacons.contains(&v3![497,-1838,-617]));
        assert!(beacons.contains(&v3![527,-524,1933]));
        assert!(beacons.contains(&v3![528,-643,409]));
        assert!(beacons.contains(&v3![534,-1912,768]));
        assert!(beacons.contains(&v3![544,-627,-890]));
        assert!(beacons.contains(&v3![553,345,-567]));
        assert!(beacons.contains(&v3![564,392,-477]));
        assert!(beacons.contains(&v3![568,-2007,-577]));
        assert!(beacons.contains(&v3![605,-1665,1952]));
        assert!(beacons.contains(&v3![612,-1593,1893]));
        assert!(beacons.contains(&v3![630,319,-379]));
        assert!(beacons.contains(&v3![686,-3108,-505]));
        assert!(beacons.contains(&v3![776,-3184,-501]));
        assert!(beacons.contains(&v3![846,-3110,-434]));
        assert!(beacons.contains(&v3![1135,-1161,1235]));
        assert!(beacons.contains(&v3![1243,-1093,1063]));
        assert!(beacons.contains(&v3![1660,-552,429]));
        assert!(beacons.contains(&v3![1693,-557,386]));
        assert!(beacons.contains(&v3![1735,-437,1738]));
        assert!(beacons.contains(&v3![1749,-1800,1813]));
        assert!(beacons.contains(&v3![1772,-405,1572]));
        assert!(beacons.contains(&v3![1776,-675,371]));
        assert!(beacons.contains(&v3![1779,-442,1789]));
        assert!(beacons.contains(&v3![1780,-1548,337]));
        assert!(beacons.contains(&v3![1786,-1538,337]));
        assert!(beacons.contains(&v3![1847,-1591,415]));
        assert!(beacons.contains(&v3![1889,-1729,1762]));
        assert!(beacons.contains(&v3![1994,-1805,1792]));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use crate::test_data::TEST_BEACONS;
    use super::*;

    #[test]
    fn test_2() {
        let mut result = parse_input(TEST_BEACONS);
        solve(&mut result);
        assert_eq!(3621, get_max_manhattan_distance(&result));
    }
}
