use std::fs::read_to_string;

fn solve(path: &str) {
    let content = read_to_string(path).expect("error reading file");
    let games = content.split_terminator("\n");

    let mut total = 0;
    for g in games {
        let opponent = g.chars().nth(0).expect("opponent has no action");
        let action = g.chars().nth(2).expect("you have no action");

        let action_points = match action {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("incorrect selected action"),
        };

        let win_points = match (opponent, action) {
            // opponent: rock
            ('A', 'X') => 3,
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,

            //opponent: paper
            ('B', 'X') => 0,
            ('B', 'Y') => 3,
            ('B', 'Z') => 6,

            //opponent: scissors
            ('C', 'X') => 6,
            ('C', 'Y') => 0,
            ('C', 'Z') => 3,

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
        solve("src/day2/data/description.txt");
    }

    #[test]
    fn puzzle_one() {
        solve("src/day2/data/puzzle.txt");
    }
}
