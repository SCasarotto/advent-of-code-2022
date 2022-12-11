fn main() {
    // Opponent - Rock, Paper, Scissors
    // A - Rock
    // B - Paper
    // C - Scissors

    // My - Rock, Paper, Scissors
    // X - Rock
    // Y - Paper
    // Z - Scissors
    fn to_selection(selection: &str) -> &str {
        match selection {
            "A" => "Rock",
            "B" => "Paper",
            "C" => "Scissors",
            "X" => "Rock",
            "Y" => "Paper",
            "Z" => "Scissors",
            _ => "error",
        }
    }

    // Scoring - Selection
    // Rock - 1
    // Paper - 2
    // Scissors - 3
    fn selection_to_score(selection: &str) -> i32 {
        match selection {
            "Rock" => 1,
            "Paper" => 2,
            "Scissors" => 3,
            _ => 0,
        }
    }

    // Scoring - Outcome
    // Loss - 0
    // Draw - 3
    // Win - 6
    fn outcome_to_score(outcmoe: &str) -> i32 {
        match outcmoe {
            "loss" => 0,
            "draw" => 3,
            "win" => 6,
            _ => 0,
        }
    }

    // Part 2:
    // Expected Outcome
    // X - Loss
    // Y - Draw
    // Z - Win
    fn to_expected_outcome(outcome: &str) -> &str {
        match outcome {
            "X" => "loss",
            "Y" => "draw",
            "Z" => "win",
            _ => "error",
        }
    }

    // Load data from input file
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    // split input by new line characters and then by spaces
    let split_by_line = input.split("\n").map(|line| line.split(" "));

    // compute total
    let mut total = 0;
    let mut total2 = 0;
    for mut game in split_by_line {
        let first_val = game.nth(0).unwrap();
        let second_val = game.nth(0).unwrap();
        // get opponent selection
        let opponent_selection = to_selection(first_val);
        // get my selection
        let my_selection = to_selection(second_val);
        // deteremine outcome
        let outcome = match (opponent_selection, my_selection) {
            ("Rock", "Rock") => "draw",
            ("Rock", "Paper") => "win",
            ("Rock", "Scissors") => "loss",
            ("Paper", "Rock") => "loss",
            ("Paper", "Paper") => "draw",
            ("Paper", "Scissors") => "win",
            ("Scissors", "Rock") => "win",
            ("Scissors", "Paper") => "loss",
            ("Scissors", "Scissors") => "draw",
            _ => "error",
        };
        // get score for selection
        let selection_score = selection_to_score(my_selection);
        // get score for outcome
        let outcome_score = outcome_to_score(outcome);
        // return score
        total += outcome_score + selection_score;

        // expected outcome
        let expected_outcome = to_expected_outcome(second_val);
        // determine my selection
        let my_selection2 = match (opponent_selection, expected_outcome) {
            ("Rock", "loss") => "Scissors",
            ("Rock", "draw") => "Rock",
            ("Rock", "win") => "Paper",
            ("Paper", "loss") => "Rock",
            ("Paper", "draw") => "Paper",
            ("Paper", "win") => "Scissors",
            ("Scissors", "loss") => "Paper",
            ("Scissors", "draw") => "Scissors",
            ("Scissors", "win") => "Rock",
            _ => "error",
        };
        // get score for selection
        let selection_score2 = selection_to_score(my_selection2);
        // get score for outcome
        let outcome_score2 = outcome_to_score(expected_outcome);
        // return score
        total2 += outcome_score2 + selection_score2;
    }
    println!("Part 1: {}", total);
    // Part 1: 11386
    println!("Part 2: {}", total2);
    // Part 2: 13600
}
