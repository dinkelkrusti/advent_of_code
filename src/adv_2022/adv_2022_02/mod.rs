use std::fs;

pub fn strategy_score_half_info(input_file: &str) -> usize {
    let strategy_guide = fs::read_to_string(input_file).unwrap();
    let games = strategy_guide.lines();

    let mut score_sum = 0;
    games.for_each(|m| score_sum += calc_game_score_half_info(m));

    println!("The given strategy guide would generate a score of {} points", score_sum);
    return score_sum;
}

fn calc_game_score_half_info(game: &str) -> usize {
    let inputs: Vec<&str> = game.split(" ").collect();
    sign_score(inputs[1]) + game_score(inputs[0], inputs[1])
}

pub fn strategy_score_full_info(input_file: &str) -> usize {
    let strategy_guide = fs::read_to_string(input_file).unwrap();
    let games = strategy_guide.lines();

    let mut score_sum = 0;
    games.for_each(|m| score_sum += calc_game_score_full_info(m));

    println!("The given strategy guide would generate a score of {} points", score_sum);
    return score_sum;
}

fn calc_game_score_full_info(game: &str) -> usize {
    let inputs: Vec<&str> = game.split(" ").collect();
    let our_sign = required_sign(Outcome::from(inputs[1]), inputs[0]);
    sign_score(our_sign) + game_score(inputs[0], our_sign)
}

fn sign_score(sign: &str) -> usize {
    match sign {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0
    }
}

fn game_score(their_sign: &str, our_sign: &str) -> usize {
    match their_sign {
        "A" => match our_sign {
            "X" => 3,
            "Y" => 6,
            &_ => 0
        },

        "B" => match our_sign {
            "Y" => 3,
            "Z" => 6,
            &_ => 0,
        },

        "C" => match our_sign {
            "Z" => 3,
            "X" => 6,
            &_ => 0,
        },

        &_ => panic!("Bricked input.")
    }
}

fn required_sign(outcome: Outcome, their_sign: &str) -> &str {
    match outcome {
        Outcome::Win => match their_sign {
            "A" => "Y",
            "B" => "Z",
            "C" => "X",
            &_ => panic!("Bricked input.")
        },

        Outcome::Draw => match their_sign {
            "A" => "X",
            "B" => "Y",
            "C" => "Z",
            &_ => panic!("Bricked input.")
        },

        Outcome::Loss => match their_sign {
            "A" => "Z",
            "B" => "X",
            "C" => "Y",
            &_ => panic!("Bricked input.")
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Bricked input.")
        }
    }
}