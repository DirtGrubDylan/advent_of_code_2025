use std::collections::HashSet;
use std::convert::Into;
use std::ops::{Add, AddAssign, Range};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Id {
    number: u64,
    digits: Vec<u64>,
}

#[derive(Debug, PartialEq)]
pub struct IdParseError;

impl Id {
    fn is_invalid(&self, number_of_chunks: usize) -> bool {
        if let Some((first, rest)) = self.as_chunks(number_of_chunks).split_first() {
            rest.iter().all(|x| x == first)
        } else {
            false
        }
    }

    fn next_invalid_id(&self, number_of_chunks: usize) -> Id {
        let number_of_digits = self.len();

        let current_id = if (number_of_digits % number_of_chunks) == 0 {
            self.clone()
        } else {
            let next_power =
                number_of_chunks - (number_of_digits % number_of_chunks) + number_of_digits - 1;

            Id::from(10_u64.pow(next_power.try_into().unwrap()))
        };

        let chunks = current_id.as_chunks(number_of_chunks);

        let number_of_chunks = chunks.len();

        let Some((first, rest)) = chunks.split_first() else {
            panic!(
                "Could not split {} into {number_of_chunks} chunks!",
                current_id.number
            )
        };

        let mut start_value = *first;

        for &next in rest {
            if start_value == next {
                continue;
            }

            if start_value > next {
                start_value -= 1;
            }

            break;
        }

        Id::new_from_repeated(start_value + 1, chunks.len())
    }

    fn as_chunks(&self, number_of_chunks: usize) -> Vec<u64> {
        if self.len() % number_of_chunks != 0 {
            return Vec::new();
        }

        let chunk_size = self.len() / number_of_chunks;

        self.digits
            .chunks_exact(chunk_size)
            .map(|chunk| chunk.iter().fold(0, |acc, x| acc * 10 + x))
            .collect()
    }

    fn len(&self) -> usize {
        self.digits.len()
    }

    fn new_from_repeated(number: u64, repeated: usize) -> Id {
        number.to_string().repeat(repeated).parse().unwrap()
    }
}

impl FromStr for Id {
    type Err = IdParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<u64>().map(Id::from).map_err(|_| IdParseError)
    }
}

impl From<u64> for Id {
    fn from(number: u64) -> Self {
        let digits: Vec<u64> = number
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(Into::into)
            .collect();

        Id { number, digits }
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.number
    }
}

impl Add for Id {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Id::from(self.number + rhs.number)
    }
}

impl AddAssign for Id {
    fn add_assign(&mut self, rhs: Self) {
        self.number += rhs.number;
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

    pub fn all_invalid_ids_within(&self) -> HashSet<Id> {
        let mut result = HashSet::new();
        let max_number_of_digits = Id::from(self.range.end).len();

        for number_of_chunks in 2..=max_number_of_digits {
            for id in self.invalid_ids_within(number_of_chunks) {
                result.insert(id.clone());
            }
        }

        result
    }

    pub fn invalid_ids_within(&self, number_of_chunks: usize) -> Vec<Id> {
        let mut result = Vec::new();
        let mut current_id = Id::from(self.range.start);

        if !current_id.is_invalid(number_of_chunks) {
            current_id = current_id.next_invalid_id(number_of_chunks);
        }

        while self.contains(&current_id) {
            result.push(current_id.clone());

            current_id = current_id.next_invalid_id(number_of_chunks);
        }

        result
    }

    fn contains(&self, id: &Id) -> bool {
        self.range.contains(&id.number)
    }
}

impl FromStr for IdRange {
    type Err = IdRangeParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (start, end) = input.split_once('-').ok_or(IdRangeParseError)?;

        let start_number = start.parse::<u64>().map_err(|_| IdRangeParseError)?;
        let end_number = end.parse::<u64>().map_err(|_| IdRangeParseError)?;

        Ok(IdRange::new(start_number, end_number + 1))
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
        assert_eq!(
            "1234".parse(),
            Ok(Id {
                number: 1_234,
                digits: vec![1, 2, 3, 4],
            })
        );
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
    fn test_id_is_invalid_chunk_size_2() {
        assert!(!Id::from(0).is_invalid(2));
        assert!(Id::from(11).is_invalid(2));
        assert!(!Id::from(14).is_invalid(2));
        assert!(!Id::from(100).is_invalid(2));
        assert!(!Id::from(763).is_invalid(2));
        assert!(!Id::from(1_514).is_invalid(2));
        assert!(Id::from(1_515).is_invalid(2));
        assert!(!Id::from(1_516).is_invalid(2));
        assert!(!Id::from(1_188_511_880).is_invalid(2));
        assert!(Id::from(1_188_511_885).is_invalid(2));
    }

    #[test]
    fn test_id_next_invalid_id_chunk_size_2() {
        assert_eq!(Id::from(0).next_invalid_id(2), Id::from(11));
        assert_eq!(Id::from(11).next_invalid_id(2), Id::from(22));
        assert_eq!(Id::from(14).next_invalid_id(2), Id::from(22));
        assert_eq!(Id::from(100).next_invalid_id(2), Id::from(1010));
        assert_eq!(Id::from(763).next_invalid_id(2), Id::from(1010));
        assert_eq!(Id::from(1_514).next_invalid_id(2), Id::from(1515));
        assert_eq!(Id::from(1_515).next_invalid_id(2), Id::from(1616));
        assert_eq!(Id::from(1_516).next_invalid_id(2), Id::from(1616));
        assert_eq!(
            Id::from(1_188_511_880).next_invalid_id(2),
            Id::from(1_188_511_885)
        );
    }

    #[test]
    fn test_id_as_chunks() {
        assert_eq!(Id::from(12).as_chunks(2), vec![1, 2]);
        assert_eq!(Id::from(1_234).as_chunks(2), vec![12, 34]);
        assert_eq!(Id::from(123_456).as_chunks(2), vec![123, 456]);

        assert_eq!(Id::from(12).as_chunks(3), vec![]);
        assert_eq!(Id::from(1_234).as_chunks(3), vec![]);
        assert_eq!(Id::from(123_456).as_chunks(3), vec![12, 34, 56]);

        assert_eq!(Id::from(12).as_chunks(4), vec![]);
        assert_eq!(Id::from(1_234).as_chunks(4), vec![1, 2, 3, 4]);
        assert_eq!(Id::from(123_456).as_chunks(4), vec![]);

        assert_eq!(Id::from(123_456).as_chunks(6), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_id_len() {
        assert_eq!(Id::from(0).len(), 1);
        assert_eq!(Id::from(1).len(), 1);
        assert_eq!(Id::from(1_234).len(), 4);
        assert_eq!(Id::from(123_456_789).len(), 9);
    }

    #[test]
    fn test_id_new_from_repeated() {
        assert_eq!(Id::new_from_repeated(0, 2), Id::from(0));
        assert_eq!(Id::new_from_repeated(1, 1), Id::from(1));
        assert_eq!(Id::new_from_repeated(12, 2), Id::from(1_212));
        assert_eq!(Id::new_from_repeated(12, 3), Id::from(121_212));
        assert_eq!(Id::new_from_repeated(11_885, 2), Id::from(1_188_511_885));
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

        assert!(range.contains(&Id::from(0)));
        assert!(range.contains(&Id::from(5)));
        assert!(range.contains(&Id::from(100)));
    }

    #[test]
    fn test_id_range_does_not_contain() {
        let range = IdRange::new(0, 101);

        assert!(!range.contains(&Id::from(101)));
    }

    #[test]
    fn test_id_range_invalid_ids_within_chunk_size_2() {
        assert_eq!(
            IdRange::new(0, 100).invalid_ids_within(2),
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
            IdRange::new(1_188_511_880, 1_188_511_890).invalid_ids_within(2),
            vec![Id::from(1_188_511_885),]
        );
        assert_eq!(IdRange::new(10_000, 100_000).invalid_ids_within(2), vec![],);
    }

    #[test]
    fn test_id_range_invalid_ids_within_chunk_size_3() {
        assert_eq!(
            IdRange::new(0, 1_001).invalid_ids_within(3),
            vec![
                Id::from(111),
                Id::from(222),
                Id::from(333),
                Id::from(444),
                Id::from(555),
                Id::from(666),
                Id::from(777),
                Id::from(888),
                Id::from(999),
            ]
        );
        assert_eq!(
            IdRange::new(1_188_511_880, 1_188_511_890).invalid_ids_within(3),
            vec![]
        );
        assert_eq!(
            IdRange::new(565_654, 565_659).invalid_ids_within(3),
            vec![Id::from(565_656)],
        );
    }

    #[test]
    fn test_id_range_all_invalid_ids_within() {
        assert_eq!(
            IdRange::new(11_111_111, 11_131_114).all_invalid_ids_within(),
            HashSet::from([
                Id::from(11_111_111),
                Id::from(11_121_112),
                Id::from(11_131_113)
            ]),
        );
        assert_eq!(
            IdRange::new(11, 23).all_invalid_ids_within(),
            HashSet::from([Id::from(11), Id::from(22)]),
        );
        assert_eq!(
            IdRange::new(2_121_212_118, 2_121_212_125).all_invalid_ids_within(),
            HashSet::from([Id::from(2_121_212_121)])
        );
        assert_eq!(
            IdRange::new(222_220, 222_225).all_invalid_ids_within(),
            HashSet::from([Id::from(222_222)])
        );
        assert_eq!(
            IdRange::new(998, 1_013).all_invalid_ids_within(),
            HashSet::from([Id::from(999), Id::from(1010)]),
        );
        assert_eq!(
            IdRange::new(69_302, 80_372).all_invalid_ids_within(),
            HashSet::from([Id::from(77_777)]),
        );
        assert_eq!(
            IdRange::new(5_858_546_565, 5_858_614_011).all_invalid_ids_within(),
            HashSet::from([Id::from(5_858_558_585), Id::from(5_858_585_858)]),
        );
    }
}
