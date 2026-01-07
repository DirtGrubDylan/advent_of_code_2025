mod product;

use crate::util::file_reader::to_string_vector;

use product::IdRange;

pub fn run() {
    let input = to_string_vector("inputs/day_02.txt")
        .first()
        .cloned()
        .expect("Input should have a single line!");

    println!("Day 1 Part 1: {:?}", part_1(&input));
    println!("Day 1 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &str) -> u64 {
    let ranges: Vec<IdRange> = input
        .split(',')
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse {value} in an IdRange!"))
        })
        .collect();

    ranges
        .iter()
        .flat_map(|range| range.invalid_ids_within(2).into_iter())
        .fold(0, |acc, id| acc + u64::from(id))
}

fn part_2(input: &str) -> u64 {
    let ranges: Vec<IdRange> = input
        .split(',')
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse {value} in an IdRange!"))
        })
        .collect();

    ranges
        .iter()
        .flat_map(|range| range.all_invalid_ids_within().into_iter())
        .fold(0, |acc, id| acc + u64::from(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_02.txt")
            .first()
            .cloned()
            .unwrap();

        assert_eq!(part_1(&input), 1_227_775_554);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_02.txt")
            .first()
            .cloned()
            .unwrap();

        assert_eq!(part_2(&input), 4_174_379_265);
    }
}
