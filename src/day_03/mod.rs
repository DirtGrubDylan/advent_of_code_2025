mod battery;

use crate::util::file_reader::to_string_vector;

use battery::Bank;

pub fn run() {
    let input = to_string_vector("inputs/day_03.txt");

    println!("Day 1 Part 1: {:?}", part_1(&input));
    println!("Day 1 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> u64 {
    let banks: Vec<Bank> = input.iter().filter_map(|line| line.parse().ok()).collect();

    banks
        .into_iter()
        .fold(0, |acc, bank| acc + bank.max_joltage(2))
}

fn part_2(input: &[String]) -> u64 {
    let banks: Vec<Bank> = input.iter().filter_map(|line| line.parse().ok()).collect();

    banks
        .into_iter()
        .fold(0, |acc, bank| acc + bank.max_joltage(12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_03.txt");

        assert_eq!(part_1(&input), 357);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_03.txt");

        assert_eq!(part_2(&input), 3_121_910_778_619);
    }
}
