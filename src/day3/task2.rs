use std::fs::read_to_string;

use crate::{day3::task1::convert_to_score, util};

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");
    let lines: Vec<&str> = content.split_terminator("\n").collect();

    let mut badges: Vec<char> = Vec::new();
    for i in 0..lines.len() / 3 {
        let first = lines.get(i * 3).expect("first out of range");
        let second = lines.get(i * 3 + 1).expect("second out of range");
        let third = lines.get(i * 3 + 2).expect("third out of range");

        let badge = util::string::intersection_char(first, &vec![second, third]);
        badges.push(badge);
    }

    let score: u32 = badges.iter().map(|c| convert_to_score(*c)).sum();
    println!("solution: {}", score);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_description() {
        solve("src/day3/data/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/day3/data/puzzle.txt");
    }
}
