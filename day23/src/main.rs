use aoc_utils::print_day_header;
use std::{
    cmp::{max, min},
    fmt::Debug,
};

#[derive(Clone, Copy)]
struct Burrow<const N: usize> {
    rooms: [[u8; N]; 4],
    hallway: [u8; 11],
}

impl<const N: usize> Burrow<N> {
    fn new(rooms: [[u8; N]; 4]) -> Self {
        Burrow { rooms, hallway: [b'.'; 11] }
    }
}

impl<const N: usize> Debug for Burrow<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str("\n#############\n#");
        self.hallway.into_iter().for_each(|h| result.push(h as char));
        for slot in (0..N).rev() {
            result.push_str("#\n###");
            for r in 0..4 {
                result.push(self.rooms[r][slot] as char);
                result.push('#');
            }
        }
        result.push('\n');
        result.push_str("  #########\n");

        write!(f, "{}", result)
    }
}

fn solve_1_impl<const N: usize>(burrow: &mut Burrow<N>, mut total_energy: u32, min_energy: &mut u32) {
    fn check_solved<const N: usize>(burrow: &mut Burrow<N>, move_fn: fn(&mut Burrow<N>) -> u32, total_energy: &mut u32, min_energy: &mut u32) -> (bool, bool) {
        let energy = move_fn(burrow);
        let moved_anything = energy != 0;
        *total_energy += energy;

        if *total_energy >= *min_energy {
            // Total energy already exceeds previous minimum -> stop
            return (moved_anything, true);
        }

        if solved(&burrow.rooms) {
            // dbg!(&burrow);
            // Solved with new energy minimum -> stop
            *min_energy = *total_energy;
            return (moved_anything, true);
        }

        (moved_anything, false)
    }

    loop {
        let result1 = check_solved(burrow, move_between_rooms, &mut total_energy, min_energy);
        // dbg!(&burrow);
        if result1.1 {
            return;
        }

        let result2 = check_solved(burrow, move_from_hallway_into_rooms, &mut total_energy, min_energy);
        // dbg!(&burrow);
        if result2.1 {
            return;
        }

        if !result1.0 && !result2.0 {
            break;
        }
    }

    for r in 0..4 {
        for slot in (0..N).rev() {
            if burrow.rooms[r][slot] == b'.' {
                continue;
            }

            for hallway_ix in (0..burrow.hallway.len()).filter(|i| *i < 2 || *i > 8 || (*i - 2) % 2 != 0) {
                let mut new_burrow = *burrow;
                let energy = move_into_hallway(&mut new_burrow, r, slot, hallway_ix);
                // dbg!(&new_burrow);
                if energy != 0 {
                    solve_1_impl(&mut new_burrow, total_energy + energy, min_energy);
                }
            }
        }
    }
}

fn move_between_rooms<const N: usize>(burrow: &mut Burrow<N>) -> u32 {
    let mut total_energy = 0u32;
    loop {
        let mut moved_something = false;
        for r in 0..4 {
            for slot in (0..N).rev() {
                let pod = burrow.rooms[r][slot];

                if pod == b'.' {
                    // Cannot move empty slot
                    continue;
                }

                let target_room = (pod - b'A') as usize;
                if r == target_room {
                    // Pod already at correct position
                    continue;
                }

                if !can_move_out(&burrow.rooms[r], slot, r) {
                    // If we are handling the back slot, make sure front slot is empty.
                    // Otherwise, the ampihpod cannot get out.
                    continue;
                }

                if let Some(target_slot) = can_move_into(&burrow.rooms[target_room], pod) {
                    if is_way_between_rooms_free(&burrow.hallway, r, target_room) {
                        moved_something = true;
                        burrow.rooms[r][slot] = b'.';
                        burrow.rooms[target_room][target_slot] = pod;

                        let energy_for_moving_out_of_room = (N - slot) as u32;
                        let energy_for_moving_between_rooms = (room_to_hallway_index(target_room) as i32 - room_to_hallway_index(r) as i32).abs() as u32;
                        let energy_for_moving_into_room = (N - target_slot) as u32;
                        total_energy += energy_per_step(target_room) * (energy_for_moving_out_of_room + energy_for_moving_between_rooms + energy_for_moving_into_room);
                    }
                }
            }
        }

        if !moved_something {
            break;
        }
    }

    total_energy
}

fn can_move_into<const N: usize>(room: &[u8; N], pod: u8) -> Option<usize> {
    for item in room.iter().enumerate() {
        if *item.1 == b'.' {
            // Found first empty slot
            return Some(item.0);
        }

        if *item.1 != pod {
            // Found a slot with a different pod -> cannot move into
            return None;
        }
    }

    None
}

fn can_move_out<const N: usize>(room: &[u8; N], slot: usize, from_room: usize) -> bool {
    let way_out_is_free = slot == N - 1 || (slot + 1..N).all(|s| room[s] == b'.');
    let already_in_right_spot = (room[slot] - b'A') as usize == from_room;
    let foreign_pods_behind = (0..slot).any(|s| room[s] != room[slot]);
    way_out_is_free && (!already_in_right_spot || foreign_pods_behind)
}

fn move_from_hallway_into_rooms<const N: usize>(burrow: &mut Burrow<N>) -> u32 {
    let mut total_energy = 0u32;
    loop {
        let mut moved_something = false;
        let items_in_hallway: Vec<(usize, u8)> = burrow.hallway.into_iter().enumerate().filter(|h| h.1 != b'.').collect();
        for h in items_in_hallway {
            let pod = h.1;
            let target_room = (pod - b'A') as usize;

            if let Some(target_slot) = can_move_into(&burrow.rooms[target_room], pod) {
                if is_way_from_hallway_into_room_free(&burrow.hallway, h.0, target_room) {
                    moved_something = true;
                    burrow.hallway[h.0] = b'.';
                    burrow.rooms[target_room][target_slot] = pod;

                    let energy_for_moving_to_room = (room_to_hallway_index(target_room) as i32 - h.0 as i32).abs() as u32;
                    let energy_for_moving_into_room = (N - target_slot) as u32;
                    total_energy += energy_per_step(target_room) * (energy_for_moving_to_room + energy_for_moving_into_room);
                }
            }
        }

        if !moved_something {
            break;
        }
    }

    total_energy
}

fn move_into_hallway<const N: usize>(burrow: &mut Burrow<N>, room: usize, slot: usize, hallway_ix: usize) -> u32 {
    let pod = burrow.rooms[room][slot];

    if pod == b'.' {
        // Cannot move empty slot
        return 0;
    }

    if !can_move_out(&burrow.rooms[room], slot, room) {
        // If we are handling the back slot, make sure front slot is empty.
        // Otherwise, the ampihpod cannot get out.
        return 0;
    }

    let target_room = (pod - b'A') as usize;
    if room == target_room && slot == 0 {
        // Pod already at correct position
        return 0;
    }

    if burrow.hallway[hallway_ix] != b'.' {
        // Target index in hallway allready occupied
        return 0;
    }

    if !is_way_from_hallway_into_room_free(&burrow.hallway, hallway_ix, room) {
        return 0;
    }

    burrow.rooms[room][slot] = b'.';
    burrow.hallway[hallway_ix] = pod;

    let energy_for_moving_out_of_room = (N - slot) as u32;
    let energy_for_moving_from_room = (room_to_hallway_index(room) as i32 - hallway_ix as i32).abs() as u32;
    energy_per_step((pod - b'A') as usize) * (energy_for_moving_out_of_room + energy_for_moving_from_room) as u32
}

fn energy_per_step(target_room: usize) -> u32 {
    10u32.pow(target_room as u32)
}

fn room_to_hallway_index(room: usize) -> usize {
    2 + room * 2
}

fn is_way_between_rooms_free(hallway: &[u8; 11], from: usize, to: usize) -> bool {
    hallway[room_to_hallway_index(min(from, to))..=room_to_hallway_index(max(from, to))].iter().all(|h| *h == b'.')
}

fn is_way_from_hallway_into_room_free(hallway: &[u8; 11], mut from: usize, to_room: usize) -> bool {
    let to = room_to_hallway_index(to_room);
    if to < from {
        from -= 1;
    } else {
        from += 1;
    }

    hallway[min(from, to)..=max(from, to)].iter().all(|h| *h == b'.')
}

fn solved<const N: usize>(rooms: &[[u8; N]; 4]) -> bool {
    for room in rooms.iter().enumerate() {
        for i in 0..N {
            if room.1[i] != b'A' + room.0 as u8 {
                return false;
            }
        }
    }

    true
}

fn main() {
    print_day_header(23);

    // Star 1
    let mut b = Burrow::new([[b'C', b'A'], [b'C', b'D'], [b'D', b'A'], [b'B', b'B']]);
    let mut min_energy = u32::MAX;
    solve_1_impl(&mut b, 0, &mut min_energy);
    println!("  Result Star 1: {:?}", min_energy);

    // Star 2
    let mut b = Burrow::new([[b'C', b'D', b'D', b'A'], [b'C', b'B', b'C', b'D'], [b'D', b'A', b'B', b'A'], [b'B', b'C', b'A', b'B']]);
    let mut min_energy = u32::MAX;
    solve_1_impl(&mut b, 0, &mut min_energy);
    println!("  Result Star 2: {:?}", min_energy);
}

/// Tests for star 1
#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test_not_solved() {
        let rooms = [[b'A', b'B'], [b'D', b'C'], [b'C', b'B'], [b'A', b'D']];
        assert!(!solved(&rooms));
    }

    #[test]
    fn test_not_solved_order() {
        let rooms = [[b'A', b'A'], [b'C', b'C'], [b'B', b'B'], [b'D', b'D']];
        assert!(!solved(&rooms));
    }

    #[test]
    fn test_solved() {
        let rooms = [[b'A', b'A'], [b'B', b'B'], [b'C', b'C'], [b'D', b'D']];
        assert!(solved(&rooms));
    }

    #[test]
    fn test_room_to_hallway_index() {
        assert_eq!(2, room_to_hallway_index(0));
        assert_eq!(4, room_to_hallway_index(1));
        assert_eq!(6, room_to_hallway_index(2));
        assert_eq!(8, room_to_hallway_index(3));
    }

    #[test]
    fn test_is_way_free() {
        let mut hallway = [b'.'; 11];
        assert!(is_way_between_rooms_free(&hallway, 0, 3));

        hallway[2] = b'B';
        assert!(!is_way_between_rooms_free(&hallway, 0, 1));
        assert!(is_way_between_rooms_free(&hallway, 2, 3));

        hallway[8] = b'B';
        assert!(!is_way_between_rooms_free(&hallway, 2, 3));
    }

    #[test]
    fn test_is_way_from_hallway_free() {
        let mut hallway = [b'.'; 11];
        assert!(is_way_from_hallway_into_room_free(&hallway, 0, 3));

        hallway[3] = b'B';
        assert!(!is_way_from_hallway_into_room_free(&hallway, 1, 1));
        assert!(is_way_from_hallway_into_room_free(&hallway, 5, 3));

        hallway[8] = b'B';
        assert!(!is_way_from_hallway_into_room_free(&hallway, 10, 2));
    }

    #[test]
    fn test_is_way_from_hallway_free_2() {
        let mut hallway = [b'.'; 11];
        hallway[5] = b'D';
        hallway[7] = b'A';
        hallway[9] = b'D';
        assert!(!is_way_from_hallway_into_room_free(&hallway, 5, 3));
        assert!(!is_way_from_hallway_into_room_free(&hallway, 7, 0));
    }

    #[test]
    fn test_energy_per_step() {
        assert_eq!(1, energy_per_step(0));
        assert_eq!(10, energy_per_step(1));
        assert_eq!(100, energy_per_step(2));
        assert_eq!(1000, energy_per_step(3));
    }

    #[test]
    fn test_move_into_rooms_1() {
        let mut b = Burrow::new([[b'A', b'B'], [b'D', b'C'], [b'C', b'.'], [b'A', b'D']]);
        let total_energy = move_between_rooms(&mut b);
        assert_eq!(400, total_energy);
        assert_eq!([[b'A', b'B'], [b'D', b'.'], [b'C', b'C'], [b'A', b'D']], b.rooms);
    }

    #[test]
    fn test_move_into_rooms_2() {
        let mut b = Burrow::new([[b'A', b'.'], [b'B', b'B'], [b'D', b'D'], [b'.', b'.']]);
        let total_energy = move_between_rooms(&mut b);
        assert_eq!(10_000, total_energy);
        assert_eq!([[b'A', b'.'], [b'B', b'B'], [b'.', b'.'], [b'D', b'D']], b.rooms);
    }

    #[test]
    fn test_move_from_halloway_into_room_1() {
        let mut b = Burrow::new([[b'A', b'.'], [b'B', b'B'], [b'C', b'C'], [b'.', b'.']]);
        b.hallway[5] = b'D';
        b.hallway[7] = b'D';
        b.hallway[9] = b'A';
        let total_energy = move_from_hallway_into_rooms(&mut b);
        assert_eq!(7_000 + 8, total_energy);
        assert_eq!([[b'A', b'A'], [b'B', b'B'], [b'C', b'C'], [b'D', b'D']], b.rooms);
        assert!(b.hallway.into_iter().all(|h| h == b'.'));
    }

    #[test]
    fn test_move_from_halloway_into_room_2() {
        let mut b = Burrow::new([[b'A', b'B'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']]);
        b.hallway[5] = b'A';
        let total_energy = move_from_hallway_into_rooms(&mut b);
        assert_eq!(0, total_energy);
        assert_eq!([[b'A', b'B'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']], b.rooms);
        assert_eq!(b'A', b.hallway[5]);
    }

    #[test]
    fn test_move_into_hallway_1() {
        let mut b = Burrow::new([[b'A', b'B'], [b'D', b'C'], [b'C', b'B'], [b'A', b'D']]);
        let total_energy = move_into_hallway(&mut b, 2, 1, 3);
        assert_eq!(40, total_energy);
        assert_eq!([[b'A', b'B'], [b'D', b'C'], [b'C', b'.'], [b'A', b'D']], b.rooms);
        assert_eq!(b'B', b.hallway[3]);
    }

    #[test]
    fn test_move_into_hallway_2() {
        let mut b = Burrow::new([[b'A', b'B'], [b'D', b'C'], [b'C', b'B'], [b'A', b'D']]);
        let total_energy = move_into_hallway(&mut b, 2, 0, 3);
        assert_eq!(0, total_energy);
        assert_eq!([[b'A', b'B'], [b'D', b'C'], [b'C', b'B'], [b'A', b'D']], b.rooms);
        assert!(b.hallway.into_iter().all(|h| h == b'.'));
    }

    #[test]
    fn test_move_into_hallway_3() {
        let mut b = Burrow::new([[b'A', b'.'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']]);
        b.hallway[0] = b'B';
        let total_energy = move_into_hallway(&mut b, 0, 0, 0);
        assert_eq!(0, total_energy);
        assert_eq!([[b'A', b'.'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']], b.rooms);
        assert_eq!(b'B', b.hallway[0]);
    }

    #[test]
    fn test_move_into_hallway_4() {
        let mut b = Burrow::new([[b'A', b'.'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']]);
        let total_energy = move_into_hallway(&mut b, 0, 0, 0);
        assert_eq!(0, total_energy);
    }

    #[test]
    fn test_move_into_hallway_5() {
        let mut b = Burrow::new([[b'A', b'A'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']]);
        let total_energy = move_into_hallway(&mut b, 0, 0, 1);
        assert_eq!(0, total_energy);
    }

    #[test]
    fn test_move_into_hallway_6() {
        let mut b = Burrow::new([[b'B', b'A'], [b'.', b'B'], [b'C', b'C'], [b'.', b'.']]);
        let total_energy = move_into_hallway(&mut b, 0, 1, 0);
        assert!(total_energy > 0);
    }

    #[test]
    fn test_solve_1() {
        let mut b = Burrow::new([[b'A', b'B'], [b'D', b'C'], [b'C', b'B'], [b'A', b'D']]);
        let mut min_energy = u32::MAX;
        solve_1_impl(&mut b, 0, &mut min_energy);
        assert_eq!(12521, min_energy);
    }

    #[test]
    fn test_can_move_into() {
        let r = [b'.', b'.', b'.', b'.'];
        assert_eq!(Some(0), can_move_into(&r, b'A'));

        let r = [b'A', b'.', b'.', b'.'];
        assert_eq!(Some(1), can_move_into(&r, b'A'));

        let r = [b'B', b'.', b'.', b'.'];
        assert_eq!(None, can_move_into(&r, b'A'));

        let r = [b'A', b'A', b'A', b'A'];
        assert_eq!(None, can_move_into(&r, b'A'));
    }

    #[test]
    fn test_can_move_out() {
        let r = [b'A', b'.', b'.', b'.'];
        assert!(can_move_out(&r, 0, 1));

        let r = [b'A', b'B', b'.', b'.'];
        assert!(!can_move_out(&r, 0, 1));

        let r = [b'A', b'A', b'A', b'B'];
        assert!(can_move_out(&r, 3, 2));

        let r = [b'A', b'A', b'A', b'.'];
        assert!(!can_move_out(&r, 2, 0));

        let r = [b'D', b'D'];
        assert!(!can_move_out(&r, 1, 3));
    }
}

/// Tests for star 2
#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test_solve_2_1() {
        let mut b = Burrow::new([[b'A', b'D', b'D', b'B'], [b'D', b'B', b'C', b'C'], [b'C', b'A', b'B', b'B'], [b'A', b'C', b'A', b'D']]);
        let mut min_energy = u32::MAX;
        solve_1_impl(&mut b, 0, &mut min_energy);
        assert_eq!(44169, min_energy);
    }

    #[test]
    fn test_solve_2_2() {
        let mut b = Burrow::new([[b'A', b'D', b'.', b'.'], [b'B', b'B', b'B', b'B'], [b'C', b'C', b'C', b'C'], [b'D', b'D', b'.', b'.']]);
        b.hallway[10] = b'D';
        b.hallway[9] = b'A';
        b.hallway[0] = b'A';
        b.hallway[1] = b'A';
        let mut min_energy = u32::MAX;
        solve_1_impl(&mut b, 0, &mut min_energy);
        assert_eq!(3000 + 8 + 7000 + 4 + 4 + 4000, min_energy);
    }
}
