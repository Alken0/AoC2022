use std::fs::read_to_string;

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");
    let games = content.split_terminator("\n");

    let mut total = 0;
    for g in games {
        let opponent = g.chars().nth(0).expect("opponent has no action");
        let outcome = g.chars().nth(2).expect("you have no action");

        let action_points = match outcome {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("incorrect selected outcome"),
        };

        let win_points = match (opponent, outcome) {
            // opponent: rock
            ('A', 'X') => 3,
            ('A', 'Y') => 1,
            ('A', 'Z') => 2,

            //opponent: paper
            ('B', 'X') => 1,
            ('B', 'Y') => 2,
            ('B', 'Z') => 3,

            //opponent: scissors
            ('C', 'X') => 2,
            ('C', 'Y') => 3,
            ('C', 'Z') => 1,

            (_, _) => panic!("incorrect round"),
        };
        total += action_points + win_points;
    }
    println!("total points: {total}")
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn task_description() {
        solve("src/two/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/two/puzzle.txt");
    }
}
