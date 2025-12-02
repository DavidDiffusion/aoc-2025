use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::path::Path;

pub fn read_data(path: &Path) -> Vec<RangeInclusive<u64>> {
    let file = File::open(path).unwrap();
    let mut rdr = std::io::BufReader::new(file);
    let mut buf = String::new();

    rdr.read_to_string(&mut buf).unwrap();

    serialize(&buf)
}

fn serialize(buf: &str) -> Vec<RangeInclusive<u64>> {
    let re = regex::Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut data = Vec::new();

    for (_, [start, end]) in re.captures_iter(buf).map(|cap| cap.extract()) {
        let range = RangeInclusive::new(start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
        data.push(range);
    }

    data
}

/// The main logic of part 1 of the puzzle
/// Checks if an ID's string representation can be decomposed into a twice repeated substring.
///
/// Naive logic:
/// first, short circuit if the string's length is not even,
/// then compare the first substring half with the second substring half (this also short circuits).
fn predicate_twice_repeating_sequence(id: u64) -> Option<u64> {
    let digit_string = id.to_string();
    let n_digits = digit_string.len();
    let mid = n_digits / 2;

    if (n_digits % 2 == 0) && (digit_string[..mid] == digit_string[mid..]) {
        Some(id)
    } else {
        None
    }
}

/// The main logic of part 2 of the puzzle
/// Checks if an ID's string representation can be decomposed into a k-repeating substring.
///
/// More sophisticated string matching logic is required for an acceptable runtime complexity.
/// Formally, the problem statement is:
/// Check if any substring length L exists (where L < n and L divides n)
/// such that repeating S[0:L] exactly n/L times reconstructs S.
/// We want to find if the string has some periodicity, not necessarily what it is exactly.
/// We can use the rotation trick and then rely on a std lib function.
fn predicate_k_repeating_sequence(id: u64) -> Option<u64> {
    let digit_string = id.to_string();
    let n_digits = digit_string.len();
    let rotated = format!("{}{}", &digit_string[1..], &digit_string[0..n_digits - 1]);

    if rotated.contains(&digit_string) {
        Some(id)
    } else {
        None
    }
}

/// Helper function to sum the IDs of a given range that satisfy a given predicate.
fn sum_invalid_ids(range: &RangeInclusive<u64>, predicate: fn(u64) -> Option<u64>) -> u64 {
    range.clone().filter_map(predicate).sum()
}

pub fn solve_part_1(data: &[RangeInclusive<u64>]) -> u64 {
    data.iter().fold(0, |acc, range| {
        acc + sum_invalid_ids(range, predicate_twice_repeating_sequence)
    })
}

pub fn solve_part_2(data: &[RangeInclusive<u64>]) -> u64 {
    data.iter().fold(0, |acc, range| {
        acc + sum_invalid_ids(range, predicate_k_repeating_sequence)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLESET: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_solution_part_1() {
        let data = serialize(SAMPLESET);
        let solution = solve_part_1(&data);
        let expected = 1227775554;
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_solution_part_2() {
        let data = serialize(SAMPLESET);
        let solution = solve_part_2(&data);
        let expected = 4174379265;
        assert_eq!(solution, expected);
    }
}
