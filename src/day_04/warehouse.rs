use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Empty,
    PaperRoll,
}

impl From<char> for Item {
    fn from(input: char) -> Item {
        match input {
            '.' => Item::Empty,
            '@' => Item::PaperRoll,
            _ => panic!("Could not parse '{input}' to an Item!"),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_value = match self {
            Item::Empty => String::from("."),
            Item::PaperRoll => String::from("@"),
        };

        write!(f, "{string_value}")
    }
}

#[derive(Debug, PartialEq)]
pub struct Warehouse {
    map: Grid<Item>,
}

impl Warehouse {
    pub fn number_of_paper_rolls_accessible(&self) -> usize {
        self.map
            .iter()
            .filter(|&(_, &item)| item == Item::PaperRoll)
            .map(|(&point, _)| self.number_of_paper_rolls_around(point))
            .filter(|&number_of_paper_rolls| number_of_paper_rolls < 4)
            .count()
    }

    pub fn number_of_paper_rolls_around(&self, point: Point2d<i32>) -> usize {
        Direction::VALUES
            .iter()
            .map(|direction| point + direction.as_offset())
            .filter_map(|new_point| self.map.get(new_point))
            .filter(|&&item| item == Item::PaperRoll)
            .count()
    }
}

impl<const N: usize> From<&[&str; N]> for Warehouse {
    fn from(input: &[&str; N]) -> Self {
        Warehouse::from(input.as_slice())
    }
}

impl From<&[&str]> for Warehouse {
    fn from(input: &[&str]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Warehouse::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Warehouse {
    fn from(input: &[String]) -> Self {
        let mut map = Grid::default();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                map.insert(point, &Item::from(c));
            }
        }

        Warehouse { map }
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Could not parse 'q' to an Item!")]
    fn test_item_from_char_panic() {
        let input = ".q@.@";

        let _: Vec<Item> = input.chars().map(Item::from).collect();
    }

    #[test]
    fn test_item_from_char_success() {
        let input = "..@.@";

        let result: Vec<Item> = input.chars().map(Item::from).collect();

        assert_eq!(
            result,
            vec![
                Item::Empty,
                Item::Empty,
                Item::PaperRoll,
                Item::Empty,
                Item::PaperRoll,
            ]
        );
    }

    #[test]
    fn test_warehouse_from_str_array() {
        let expected_grid = Grid::from([
            (Point2d::new(0, 0), Item::Empty),
            (Point2d::new(1, 0), Item::PaperRoll),
            (Point2d::new(2, 0), Item::Empty),
            (Point2d::new(0, 1), Item::PaperRoll),
            (Point2d::new(1, 1), Item::PaperRoll),
            (Point2d::new(2, 1), Item::Empty),
            (Point2d::new(0, 2), Item::Empty),
            (Point2d::new(1, 2), Item::PaperRoll),
            (Point2d::new(2, 2), Item::Empty),
            (Point2d::new(0, 3), Item::PaperRoll),
            (Point2d::new(1, 3), Item::PaperRoll),
            (Point2d::new(2, 3), Item::Empty),
        ]);

        let expected = Warehouse { map: expected_grid };

        let result = Warehouse::from(&[".@.", "@@.", ".@.", "@@."]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_warehouse_number_of_paper_rolls_around() {
        let warehouse = Warehouse::from(&[".@.", "@@.", "@..", "@@@"]);

        assert_eq!(
            warehouse.number_of_paper_rolls_around(Point2d::new(1, 0)),
            2
        );
        assert_eq!(
            warehouse.number_of_paper_rolls_around(Point2d::new(0, 1)),
            3
        );
        assert_eq!(
            warehouse.number_of_paper_rolls_around(Point2d::new(0, 2)),
            4
        );
        assert_eq!(
            warehouse.number_of_paper_rolls_around(Point2d::new(2, 3)),
            1
        );
    }

    #[test]
    fn test_warehouse_number_of_paper_rolls_accessible() {
        let warehouse = Warehouse::from(&[".@.", "@@.", "@@.", "@@."]);

        assert_eq!(warehouse.number_of_paper_rolls_accessible(), 3);
    }
}
