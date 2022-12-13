use regex::Regex;

use std::fs::read_to_string;

#[derive(Debug)]
struct Row {
    fs: u32,
    fe: u32,
    ss: u32,
    se: u32,
}

impl Row {
    fn new(line: &str) -> Self {
        let numbers = Regex::new(r"\d+").expect("regex");
        let numbers: Vec<u32> = numbers
            .find_iter(line)
            .map(|e| e.as_str().parse::<u32>().expect("msg"))
            .collect();

        Self {
            fs: *numbers.get(0).unwrap(),
            fe: *numbers.get(1).unwrap(),
            ss: *numbers.get(2).unwrap(),
            se: *numbers.get(3).unwrap(),
        }
    }

    fn is_second_fully_contained(&self) -> bool {
        let second_in_first = self.fs <= self.ss && self.fe >= self.se;
        let first_in_second = self.ss <= self.fs && self.se >= self.fe;
        return first_in_second || second_in_first;
    }
}

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");
    let lines = content.split_terminator("\n");
    let fully_contained_rows: Vec<Row> = lines
        .map(Row::new)
        .filter(Row::is_second_fully_contained)
        .collect();
    println!("{}", fully_contained_rows.len())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_description() {
        solve("src/day4/data/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/day4/data/puzzle.txt");
    }
}
