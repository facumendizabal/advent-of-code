use std::{fs, str::FromStr};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Move {
    fn value(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn losses_to(&self) -> Self {
        self.beats().beats()
    }
}

fn calc_scores(move_1: Move, move_2: Move) -> (i32, i32) {
    let mut score_1 = move_1.value();
    let mut score_2 = move_2.value();

    if move_1 == move_2 {
        score_1 += 3;
        score_2 += 3;
    } else if move_1.beats() == move_2 {
        score_1 += 6;
    } else if move_2.beats() == move_1 {
        score_2 += 6;
    }

    (score_1, score_2)
}

fn main() {
    let file_path = "inputs/in-d2.txt";
    let input = fs::read_to_string(file_path).expect("Error reading file");

    let rounds: Vec<Vec<&str>> = input
        .split("\n")
        .map(|round| round.split(" ").collect())
        .collect();

    let total_score: i32 = rounds.iter().fold(0, |final_score, round| {
        if let [opponents_move_str, my_move_str] = round[..] {
            let my_move = Move::from_str(my_move_str).unwrap();
            let opponents_move = Move::from_str(opponents_move_str).unwrap();

            let (my_score, _) = calc_scores(my_move, opponents_move);

            final_score + my_score
        } else {
            panic!();
        }
    });

    let total_score_2: i32 = rounds.iter().fold(0, |final_score, round| {
        if let [opponents_move_str, required_result] = round[..] {
            let opponents_move = Move::from_str(opponents_move_str).unwrap();

            let my_move = match required_result {
                "X" => opponents_move.beats(),
                "Y" => opponents_move.clone(),
                "Z" => opponents_move.losses_to(),
                _ => panic!(),
            };

            let (my_score, _) = calc_scores(my_move, opponents_move);

            final_score + my_score
        } else {
            panic!();
        }
    });

    println!("Total score 1 -> {}", total_score);
    println!("Total score 2 -> {}", total_score_2);
}
