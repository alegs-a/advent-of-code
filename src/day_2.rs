use std::fs;

#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy)]
struct Game {
    your_play: Play,
    opponent_play: Play,
}

pub fn day_two() {
    day_two_part_1();
    day_two_part_2();
}

fn day_two_part_1() {
    println!("DAY 2 ============");

    let input = fs::read_to_string("input/2.txt").expect("Problem reading the input file");

    let mut score = 0;

    for line in input.lines() {
        let mut line_chars = line.chars();
        let opponent_play = line_chars.next();
        let _ = line_chars.next();
        let your_play = line_chars.next();

        let game = Game {
            your_play: parse_input_play(your_play),
            opponent_play: parse_input_play(opponent_play),
        };

        let outcome = play_game(game);

        match outcome {
            GameResult::Win => score += 6,
            GameResult::Draw => score += 3,
            GameResult::Lose => (),
        }
        match game.your_play {
            Play::Rock => score += 1,
            Play::Paper => score += 2,
            Play::Scissors => score += 3,
        }
    }
    println!("PART 1: Your total score is {}", score);
}

fn day_two_part_2() {
    let input = fs::read_to_string("input/2.txt").expect("Problem reading the input file");

    let mut score = 0;

    for line in input.lines() {
        let mut line_chars = line.chars();
        let opponent_play = line_chars.next();
        let _ = line_chars.next();
        let desired_result = line_chars.next();

        let opponent_play = parse_input_play(opponent_play);
        let desired_result = parse_input_game(desired_result);

        let game = Game {
            opponent_play,
            your_play: play_game_backwards(opponent_play, desired_result),
        };

        match game.your_play {
            Play::Rock => score += 1,
            Play::Paper => score += 2,
            Play::Scissors => score += 3,
        }
        match play_game(game) {
            GameResult::Win => score += 6,
            GameResult::Draw => score += 3,
            GameResult::Lose => (),
        }
    }
    println!("PART 2: Your total score is: {}", score);
}

fn parse_input_play(input: Option<char>) -> Play {
    match input.unwrap() {
        'A' | 'X' => Play::Rock,
        'B' | 'Y' => Play::Paper,
        'C' | 'Z' => Play::Scissors,
        _ => {
            panic!("Input is broken: contains characters other than ABCXYZ.");
        }
    }
}

fn parse_input_game(input: Option<char>) -> GameResult {
    match input.unwrap() {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => {
            panic!("Input is broken: contains characters other than ABCXYZ.");
        }
    }
}

fn play_game(game: Game) -> GameResult {
    match game.your_play {
        Play::Rock => match game.opponent_play {
            Play::Rock => GameResult::Draw,
            Play::Paper => GameResult::Lose,
            Play::Scissors => GameResult::Win,
        },
        Play::Paper => match game.opponent_play {
            Play::Rock => GameResult::Win,
            Play::Paper => GameResult::Draw,
            Play::Scissors => GameResult::Lose,
        },
        Play::Scissors => match game.opponent_play {
            Play::Rock => GameResult::Lose,
            Play::Paper => GameResult::Win,
            Play::Scissors => GameResult::Draw,
        },
    }
}

fn play_game_backwards(opponent_play: Play, desired_result: GameResult) -> Play {
    match opponent_play {
        Play::Rock => match desired_result {
            GameResult::Win => Play::Paper,
            GameResult::Draw => Play::Rock,
            GameResult::Lose => Play::Scissors,
        },
        Play::Paper => match desired_result {
            GameResult::Win => Play::Scissors,
            GameResult::Draw => Play::Paper,
            GameResult::Lose => Play::Rock,
        },
        Play::Scissors => match desired_result {
            GameResult::Win => Play::Rock,
            GameResult::Draw => Play::Scissors,
            GameResult::Lose => Play::Paper,
        },
    }
}
