use std::ops::{Add, AddAssign, Range};
use std::str::FromStr;

use crate::util::math::divmod;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Id {
    value: u64,
}

#[derive(Debug, PartialEq)]
pub struct IdParseError;

impl Id {
    fn is_invalid(&self) -> bool {
        let number_of_digits = Id::number_of_digits(self.value);

        if (number_of_digits % 2) == 0 {
            let (first, second) = Id::split(self.value);

            first == second
        } else {
            false
        }
    }

    fn next_invalid_id(&self) -> Id {
        let number_of_digits = Id::number_of_digits(self.value);

        let (first, second) = if (number_of_digits % 2) == 0 {
            Id::split(self.value)
        } else {
            Id::split(10_u64.pow(number_of_digits.try_into().unwrap()))
        };

        if first > second {
            Id::new_from_half_mirrored(first)
        } else {
            Id::new_from_half_mirrored(first + 1)
        }
    }

    fn split(input: u64) -> (u64, u64) {
        let number_of_digits = Id::number_of_digits(input);

        if (number_of_digits % 2) == 1 {
            panic!("{input} does not have an even amount of digits!");
        }

        let divisor = 10_u64.pow((number_of_digits / 2).try_into().unwrap());

        divmod(input, divisor)
    }

    fn number_of_digits(input: u64) -> u64 {
        match input {
            0 => 1,
            _ => (input.ilog10() + 1).into(),
        }
    }

    fn new_from_half_mirrored(input: u64) -> Id {
        let number_of_digits = Id::number_of_digits(input);

        let multiplier = 10_u64.pow(number_of_digits.try_into().unwrap());

        Id::from(input * (multiplier + 1))
    }
}

impl FromStr for Id {
    type Err = IdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .parse::<u64>()
            .map(|value| Id::from(value))
            .map_err(|_| IdParseError)
    }
}

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        Id { value }
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.value
    }
}

impl Add for Id {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Id::from(self.value + rhs.value)
    }
}

impl AddAssign for Id {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

#[derive(Debug, PartialEq)]
pub struct IdRange {
    range: Range<u64>,
}

#[derive(Debug, PartialEq)]
pub struct IdRangeParseError;

impl IdRange {
    fn new(start_inc: u64, end_exc: u64) -> IdRange {
        IdRange {
            range: (start_inc..end_exc),
        }
    }

    pub fn invalid_ids_within(&self) -> Vec<Id> {
        let mut result = Vec::new();
        let mut current_id = Id::from(self.range.start);

        while self.contains(current_id) {
            if current_id.is_invalid() {
                result.push(current_id);
            }

            current_id = current_id.next_invalid_id();
        }

        result
    }

    fn contains(&self, id: Id) -> bool {
        self.range.contains(&id.value)
    }
}

impl FromStr for IdRange {
    type Err = IdRangeParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (start, end) = input.split_once('-').ok_or(IdRangeParseError)?;

        let start_value = start.parse::<u64>().map_err(|_| IdRangeParseError)?;
        let end_value = end.parse::<u64>().map_err(|_| IdRangeParseError)?;

        Ok(IdRange::new(start_value, end_value + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_from_str_err() {
        assert_eq!("".parse::<Id>(), Err(IdParseError));
        assert_eq!("-1".parse::<Id>(), Err(IdParseError));
        assert_eq!("a".parse::<Id>(), Err(IdParseError));
        assert_eq!("12-34".parse::<Id>(), Err(IdParseError));
    }

    #[test]
    fn test_id_from_str_ok() {
        assert_eq!("1".parse(), Ok(Id::from(1)));
        assert_eq!("1234".parse(), Ok(Id::from(1_234)));
    }

    #[test]
    fn test_id_from_u64() {
        assert_eq!(Id::from(0), Id::from(0));
        assert_eq!(Id::from(1), Id::from(1));
        assert_eq!(Id::from(1_234), Id::from(1_234));
    }

    #[test]
    fn test_id_to_u64() {
        assert_eq!(u64::from(Id::from(0)), 0);
        assert_eq!(u64::from(Id::from(1)), 1);
        assert_eq!(u64::from(Id::from(1_234)), 1_234);
    }

    #[test]
    fn test_id_is_invalid() {
        assert_eq!(Id::from(0).is_invalid(), false);
        assert_eq!(Id::from(11).is_invalid(), true);
        assert_eq!(Id::from(14).is_invalid(), false);
        assert_eq!(Id::from(100).is_invalid(), false);
        assert_eq!(Id::from(763).is_invalid(), false);
        assert_eq!(Id::from(1514).is_invalid(), false);
        assert_eq!(Id::from(1515).is_invalid(), true);
        assert_eq!(Id::from(1516).is_invalid(), false);
        assert_eq!(Id::from(1188511880).is_invalid(), false);
        assert_eq!(Id::from(1188511885).is_invalid(), true);
    }

    #[test]
    fn test_id_next_invalid_id() {
        assert_eq!(Id::from(0).next_invalid_id(), Id::from(11));
        assert_eq!(Id::from(11).next_invalid_id(), Id::from(22));
        assert_eq!(Id::from(14).next_invalid_id(), Id::from(22));
        assert_eq!(Id::from(100).next_invalid_id(), Id::from(1010));
        assert_eq!(Id::from(763).next_invalid_id(), Id::from(1010));
        assert_eq!(Id::from(1514).next_invalid_id(), Id::from(1515));
        assert_eq!(Id::from(1515).next_invalid_id(), Id::from(1616));
        assert_eq!(Id::from(1516).next_invalid_id(), Id::from(1616));
        assert_eq!(Id::from(1188511880).next_invalid_id(), Id::from(1188511885));
    }

    #[test]
    fn test_id_split() {
        assert_eq!(Id::split(12), (1, 2));
        assert_eq!(Id::split(1234), (12, 34));
        assert_eq!(Id::split(123456), (123, 456));
    }

    #[test]
    fn test_id_number_of_digits() {
        assert_eq!(Id::number_of_digits(0), 1);
        assert_eq!(Id::number_of_digits(1), 1);
        assert_eq!(Id::number_of_digits(1234), 4);
        assert_eq!(Id::number_of_digits(123_456_789), 9);
    }

    #[test]
    fn test_id_new_from_half_mirrored() {
        assert_eq!(Id::new_from_half_mirrored(0), Id::from(0));
        assert_eq!(Id::new_from_half_mirrored(1), Id::from(11));
        assert_eq!(Id::new_from_half_mirrored(12), Id::from(1212));
        assert_eq!(Id::new_from_half_mirrored(11885), Id::from(1188511885));
    }

    #[test]
    fn test_id_range_from_str_err() {
        assert_eq!("0-".parse::<IdRange>(), Err(IdRangeParseError));
        assert_eq!("-100".parse::<IdRange>(), Err(IdRangeParseError));
        assert_eq!(",0-100".parse::<IdRange>(), Err(IdRangeParseError));
        assert_eq!("0-100,".parse::<IdRange>(), Err(IdRangeParseError));
        assert_eq!(",0-100,".parse::<IdRange>(), Err(IdRangeParseError));
        assert_eq!("-100-100".parse::<IdRange>(), Err(IdRangeParseError));
    }

    #[test]
    fn test_id_range_from_str_ok() {
        assert_eq!("0-100".parse(), Ok(IdRange::new(0, 101)));
        assert_eq!(
            "5858546565-5858614010".parse(),
            Ok(IdRange::new(5_858_546_565, 5_858_614_011))
        );
    }

    #[test]
    fn test_id_range_contains() {
        let range = IdRange::new(0, 101);

        assert!(range.contains(Id::from(0)));
        assert!(range.contains(Id::from(5)));
        assert!(range.contains(Id::from(100)));
    }

    #[test]
    fn test_id_range_does_not_contain() {
        let range = IdRange::new(0, 101);

        assert!(!range.contains(Id::from(101)));
    }

    #[test]
    fn test_id_range_invalid_ids_within() {
        assert_eq!(
            IdRange::new(0, 100).invalid_ids_within(),
            vec![
                Id::from(11),
                Id::from(22),
                Id::from(33),
                Id::from(44),
                Id::from(55),
                Id::from(66),
                Id::from(77),
                Id::from(88),
                Id::from(99),
            ]
        );
        assert_eq!(
            IdRange::new(1_188_511_880, 1_188_511_890).invalid_ids_within(),
            vec![Id::from(1188511885),]
        );
        assert_eq!(IdRange::new(10_000, 100_000).invalid_ids_within(), vec![],);
    }
}
