use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Row {
    count: usize,
    from: usize,
    to: usize,
}

impl Row {
    fn new(line: &str) -> Self {
        let numbers = Regex::new(r"\d+").expect("regex");
        let numbers: Vec<usize> = numbers
            .find_iter(line)
            .map(|e| e.as_str().parse::<usize>().expect("number"))
            .collect();

        Self {
            count: *numbers.get(0).unwrap(),
            from: *numbers.get(1).unwrap(),
            to: *numbers.get(2).unwrap(),
        }
    }
}
lazy_static! {
    static ref DESCRIPTION: Vec<Vec<&'static str>> =
        vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];
    static ref PUZZLE: Vec<Vec<&'static str>> = vec![
        vec!["Q", "S", "W", "C", "Z", "V", "F", "T"],
        vec!["Q", "R", "B"],
        vec!["B", "Z", "T", "Q", "P", "M", "S"],
        vec!["D", "V", "F", "R", "Q", "H"],
        vec!["J", "G", "L", "D", "B", "S", "T", "P"],
        vec!["W", "R", "T", "Z"],
        vec!["H", "Q", "M", "N", "S", "F", "R", "J"],
        vec!["R", "N", "F", "H", "W"],
        vec!["J", "Z", "T", "Q", "P", "R", "B"]
    ];
}

fn solve(path: &str, container: Vec<Vec<&str>>) {
    let mut container = container;
    let content = read_to_string(path).expect("error reading file");
    content.split_terminator("\n").map(Row::new).for_each(|e| {
        let from_stack = container.get_mut(e.from - 1).expect("from");
        let mut elements = from_stack.split_off(from_stack.len() - e.count);
        let to_stack = container.get_mut(e.to - 1).expect("to");
        to_stack.append(&mut elements);
    });

    for e in 0..container.len() {
        println!("{:?}", container.get(e).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn task_description() {
        solve("src/day5/data/description.txt", DESCRIPTION.to_vec());
    }

    #[test]
    fn puzzle_one() {
        solve("src/day5/data/puzzle.txt", PUZZLE.to_vec());
    }
}
