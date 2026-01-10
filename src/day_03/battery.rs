use std::panic;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Battery {
    joltage: u64,
}

impl Battery {
    fn new(joltage: u64) -> Self {
        Battery { joltage }
    }

    fn combine(self, other: Self) -> Self {
        Battery::new(self.combined_joltage(other))
    }

    fn combined_joltage(self, other: Self) -> u64 {
        self.joltage * 10 + other.joltage
    }
}

impl From<char> for Battery {
    fn from(input: char) -> Battery {
        match input.to_digit(10) {
            Some(digit) => Battery {
                joltage: digit.into(),
            },
            None => panic!("Given char '{input}' is not a digit!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bank {
    pub batteries: Vec<Battery>,
}

#[derive(Debug, PartialEq)]
pub struct BankParseError;

impl Bank {
    pub fn max_joltage(&self, number_of_batteries: usize) -> u64 {
        let mut batteries_to_use: Vec<Battery> = Vec::new();

        let mut start_search_index = 0;
        let mut end_search_index = self.batteries.len() - number_of_batteries;

        while batteries_to_use.len() != number_of_batteries {
            let (index, battery) = self
                .batteries
                .iter()
                .enumerate()
                .filter(|(idx, _)| (start_search_index..=end_search_index).contains(idx))
                .rev()
                .max_by_key(|(_, bat)| *bat)
                .expect("Could not find a max battery");

            batteries_to_use.push(*battery);

            start_search_index = index + 1;
            end_search_index += 1;
        }

        batteries_to_use
            .into_iter()
            .fold(Battery::new(0), Battery::combine)
            .joltage
    }
}

impl FromStr for Bank {
    type Err = BankParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        panic::catch_unwind(|| Bank {
            batteries: input.chars().map(Battery::from).collect(),
        })
        .map_err(|_| BankParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Given char 'q' is not a digit!")]
    fn test_battery_from_char_err() {
        let _ = Battery::from('q');
    }

    #[test]
    fn test_battery_from_char_success() {
        assert_eq!(Battery::from('9'), Battery { joltage: 9 });
    }

    #[test]
    fn test_battery_combine() {
        let battery = Battery::new(987_654);

        assert_eq!(battery.combine(Battery::new(3)), Battery::new(9_876_543));
    }

    #[test]
    fn test_battery_combined_joltage() {
        let battery = Battery::new(9);

        assert_eq!(battery.combined_joltage(Battery::new(8)), 98);
    }

    #[test]
    fn test_bank_from_str_err() {
        assert_eq!("987q".parse::<Bank>(), Err(BankParseError));
    }

    #[test]
    fn test_bank_from_str_ok() {
        assert_eq!(
            "9876".parse(),
            Ok(Bank {
                batteries: vec![
                    Battery::new(9),
                    Battery::new(8),
                    Battery::new(7),
                    Battery::new(6),
                ]
            })
        );
    }

    #[test]
    fn test_bank_max_joltage_2_batteries() {
        let banks: Vec<Bank> = vec![
            "987654321111111".parse().unwrap(),
            "811111111111119".parse().unwrap(),
            "234234234234278".parse().unwrap(),
            "818181911112111".parse().unwrap(),
        ];

        let max_joltages: Vec<u64> = banks.iter().map(|bank| bank.max_joltage(2)).collect();

        assert_eq!(max_joltages, vec![98, 89, 78, 92]);
    }

    #[test]
    fn test_bank_max_joltage_12_batteries() {
        let banks: Vec<Bank> = vec![
            "987654321111111".parse().unwrap(),
            "811111111111119".parse().unwrap(),
            "234234234234278".parse().unwrap(),
            "818181911112111".parse().unwrap(),
        ];

        let max_joltages: Vec<u64> = banks.iter().map(|bank| bank.max_joltage(12)).collect();

        assert_eq!(
            max_joltages,
            vec![
                987_654_321_111,
                811_111_111_119,
                434_234_234_278,
                888_911_112_111
            ]
        );
    }
}
