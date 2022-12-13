use regex::Regex;

use std::fs::read_to_string;

#[derive(Debug, Default)]
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
    let overlapping_values: Vec<Row> = lines
        .map(Row::new)
        .filter(|e| (e.ss <= e.fe && e.fs <= e.ss) || (e.fs <= e.se && e.fs >= e.ss)) //overlaps
        .collect();
    println!("{:?}", overlapping_values.len())
}

/*
.fold((u32::MAX, u32::MIN), |acc, x| {
            let start = min(max(x.fs, x.ss), acc.0);
            if start == 1 {
                println!("{:?}", x);
            }
            let end = max(min(x.fe, x.se), acc.1);
            (start, end)
        }); */

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
