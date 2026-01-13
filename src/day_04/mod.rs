mod warehouse;

use crate::util::file_reader::to_string_vector;

use warehouse::Warehouse;

pub fn run() {
    let input = to_string_vector("inputs/day_04.txt");

    println!("Day 1 Part 1: {:?}", part_1(&input));
    println!("Day 1 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let warehouse = Warehouse::from(input);

    warehouse.number_of_paper_rolls_accessible()
}

fn part_2(input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_04.txt");

        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_04.txt");

        assert_eq!(part_2(&input), 3_121_910_778_619);
    }
}
