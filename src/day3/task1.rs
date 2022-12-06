use std::fs::read_to_string;

use crate::util;

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");
    let total: u32 = content
        .split_terminator("\n")
        .map(util::string::halves)
        .map(|(a, b)| util::string::intersecting_unique_chars(a, b))
        .map(|e| e.iter().map(|g| convert_to_score(*g)).sum::<u32>())
        .sum();
    println!("{total}")
}

pub fn convert_to_score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - 96;
    }
    if c.is_ascii_uppercase() {
        return c as u32 - 38;
    }
    println!("invalid char: '{}'", c);
    return 0;
}

#[cfg(test)]
mod test {
    use super::{convert_to_score, solve};

    #[test]
    fn task_description() {
        solve("src/day3/data/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/day3/data/puzzle.txt");
    }

    #[test]
    fn test_convert_to_score() {
        assert_eq!(convert_to_score('a'), 1);
        assert_eq!(convert_to_score('z'), 26);
        assert_eq!(convert_to_score('A'), 27);
        assert_eq!(convert_to_score('Z'), 52);
        assert_eq!(convert_to_score('M'), 39);
    }
}
