use std::fs::read_to_string;

use crate::util;

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");

    let sorted_by_elves: Vec<&str> = content.split_terminator("\n\n").collect();

    let calories_by_elve: Vec<i32> = sorted_by_elves
        .into_iter()
        .map(|e| e.split_whitespace())
        .map(|e| e.map(|g| g.parse::<i32>().unwrap()))
        .map(|e| e.sum())
        .collect();

    let maxis = util::arg3max(calories_by_elve);
    println!("index+1!: {:?}", maxis);
    println!(
        "total: {:?}",
        maxis[0].value + maxis[1].value + maxis[2].value
    )
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn task_description() {
        solve("src/one/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/one/puzzle.txt");
    }
}
