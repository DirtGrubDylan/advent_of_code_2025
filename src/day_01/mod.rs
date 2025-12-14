mod safe;

use crate::util::file_reader::to_string_vector;
use safe::{Lock, LockRotation};

pub fn run() {
    let input = to_string_vector("inputs/day_01.txt");

    let rotations: Vec<LockRotation> = input
        .iter()
        .map(|line| line.parse().expect(&format!("Cannot parse line: {}", line)))
        .collect();

    println!("Day 1 Part 1: {:?}", part_1(&rotations));
    println!("Day 1 Part 2: {:?}", part_2(&rotations));
}

fn part_1(lock_rotations: &[LockRotation]) -> isize {
    let mut result = 0;
    let mut lock = Lock::default();

    for rotation in lock_rotations {
        lock.rotate_mut(*rotation);

        if lock.current_position == 0 {
            result += 1;
        }
    }

    result
}

fn part_2(_input: &[LockRotation]) -> isize {
    unimplemented!("Not yet implemented!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<LockRotation> = to_string_vector("test_inputs/day_01.txt")
            .iter()
            .map(|line| line.parse().expect(&format!("Cannot parse line: {}", line)))
            .collect();

        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn test_part_2() {
        let _input: Vec<LockRotation> = to_string_vector("test_inputs/day_01.txt")
            .iter()
            .map(|line| line.parse().expect(&format!("Cannot parse line: {}", line)))
            .collect();

        unimplemented!("Not yet implemented!")
    }
}
