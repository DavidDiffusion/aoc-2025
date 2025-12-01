use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::ops::AddAssign;
use std::path::Path;

/// Wrapper type to represent a single rotation instruction:
/// negative values indicate left rotations, positive values indicate right rotations.
#[derive(Debug)]
pub struct Rotation(i32);

/// Serialization logic for data IO.
impl From<String> for Rotation {
    fn from(s: String) -> Self {
        let mut chars = s.chars();
        let is_add = match chars.next().unwrap() {
            'L' => false,
            'R' => true,
            _ => panic!("Invalid direction"),
        };
        let mut steps = chars.collect::<String>().parse::<i32>().unwrap();
        steps *= if is_add { 1 } else { -1 };

        Rotation(steps)
    }
}

/// Wrapper type to represent the position of the safe's dial
/// around a signed numeric type. Allows for custom arithmetic operations modulo 100,
/// i.e. the range of valid values is [0, 99].
#[derive(Debug)]
struct DialPosition(i32);

/// Custom arithmetic operation for the DialPosition type: add assign a rotation instruction modulo 100.
impl AddAssign<&Rotation> for DialPosition {
    fn add_assign(&mut self, rhs: &Rotation) {
        let value = self.0 + rhs.0;
        self.0 = ((value % 100) + 100) % 100;
    }
}

/// Custom arithmetic operation for the DialPosition type: add a rotation instruction modulo 100,
/// returning the number of times the value over- or underflowed the range [0, 99].
impl DialPosition {
    fn addmod(&mut self, rhs: &Rotation) -> u32 {
        let value = self.0 + rhs.0;

        let mut overflows = (value / 100).unsigned_abs();
        if value <= 0 && self.0 > 0 {
            overflows += 1;
        }

        self.0 = ((value % 100) + 100) % 100;
        overflows
    }
}

pub fn read_data(path: &Path) -> Result<Vec<Rotation>, Box<dyn Error>> {
    let file = File::open(path)?;
    let rdr = std::io::BufReader::new(file);

    let data = rdr
        .lines()
        .map(|line| Rotation::from(line.unwrap()))
        .collect::<Vec<Rotation>>();

    Ok(data)
}

pub fn solve_part_1(data: &Vec<Rotation>) -> Result<u32, Box<dyn Error>> {
    let mut counter: u32 = 0;
    let mut position = DialPosition(50);

    for rotation in data {
        position += rotation;
        if position.0 == 0 {
            counter += 1;
        }
    }

    Ok(counter)
}

pub fn solve_part_2(data: &Vec<Rotation>) -> Result<u32, Box<dyn Error>> {
    let mut counter: u32 = 0;
    let mut position = DialPosition(50);

    for rotation in data {
        counter += position.addmod(rotation);
    }

    Ok(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sample data from the problem description.
    const SAMPLES: [&'static str; 10] = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test_solution_part_1() {
        let data = SAMPLES
            .iter()
            .map(|s| Rotation::from(s.to_string()))
            .collect::<Vec<Rotation>>();

        let expected = 3;
        assert_eq!(solve_part_1(&data).unwrap(), expected);
    }

    #[test]
    fn test_solution_part_2() {
        let data = SAMPLES
            .iter()
            .map(|s| Rotation::from(s.to_string()))
            .collect::<Vec<Rotation>>();

        let expected = 6;
        assert_eq!(solve_part_2(&data).unwrap(), expected);
    }
}
