use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Lock {
    pub current_position: isize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LockRotation {
    Left(isize),
    Right(isize),
}

#[derive(Debug, PartialEq)]
pub struct LockRotationParseError;

impl Lock {
    pub fn rotate_mut(&mut self, rotation: LockRotation) -> isize {
        let mut zeros_seen = 0;

        let mut new_position = match rotation {
            LockRotation::Left(val) => self.current_position - val,
            LockRotation::Right(val) => self.current_position + val,
        };

        zeros_seen += new_position.abs() / 100;

        if new_position == 0 {
            zeros_seen += 1;
        } else if new_position < 0 {
            if self.current_position != 0 {
                zeros_seen += 1;
            }

            new_position = 100 - (new_position.abs() % 100);
        }

        self.current_position = new_position % 100;

        zeros_seen
    }
}

impl Default for Lock {
    fn default() -> Self {
        Lock {
            current_position: 50,
        }
    }
}

impl FromStr for LockRotation {
    type Err = LockRotationParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split_input = input
            .split_at_checked(1)
            .map(|(dir, val)| (dir, val.parse::<isize>()));

        match split_input {
            Some(("L", Ok(val))) => Ok(LockRotation::Left(val)),
            Some(("R", Ok(val))) => Ok(LockRotation::Right(val)),
            _ => Err(LockRotationParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_rotation_from_str_err() {
        let input = "55";

        let result = input.parse::<LockRotation>();

        assert_eq!(result, Err(LockRotationParseError));
    }

    #[test]
    fn test_lock_rotation_from_str_left() {
        let input = "L997";

        let result = input.parse();

        assert_eq!(result, Ok(LockRotation::Left(997)));
    }

    #[test]
    fn test_lock_rotation_from_str_right() {
        let input = "R1";

        let result = input.parse();

        assert_eq!(result, Ok(LockRotation::Right(1)));
    }

    #[test]
    fn test_lock_rotate_mut_left() {
        let mut lock = Lock::default();

        let result = lock.rotate_mut(LockRotation::Left(68));

        assert_eq!(result, 1);
        assert_eq!(lock.current_position, 82);
    }

    #[test]
    fn test_lock_rotate_mut_right() {
        let mut lock = Lock::default();

        let result = lock.rotate_mut(LockRotation::Right(1000));

        assert_eq!(result, 10);
        assert_eq!(lock.current_position, 50);
    }

    #[test]
    fn test_lock_rotate_mut_array() {
        let input = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let mut lock = Lock::default();
        let mut password = 0;
        let mut zeros_seen = 0;

        for value in input {
            let rotation = value.parse().unwrap();

            let number_of_clicks = lock.rotate_mut(rotation);

            zeros_seen += number_of_clicks;

            if lock.current_position == 0 {
                password += 1;
            }
        }

        assert_eq!(password, 3);
        assert_eq!(zeros_seen, 6);
        assert_eq!(lock.current_position, 32);
    }

    #[test]
    fn test_lock_rotate_mut_array_large() {
        let input = [
            "R1000", "L1000", "L50", "R1", "L1", "L1", "R1", "R100", "R1",
        ];

        let mut lock = Lock::default();
        let mut password = 0;
        let mut zeros_seen = 0;

        for value in input {
            let rotation = value.parse().unwrap();

            let number_of_clicks = lock.rotate_mut(rotation);

            zeros_seen += number_of_clicks;

            if lock.current_position == 0 {
                password += 1;
            }
        }

        assert_eq!(password, 4);
        assert_eq!(zeros_seen, 24);
        assert_eq!(lock.current_position, 1);
    }
}
