use std::fs;

pub fn day_five() {
    let raw_input = fs::read_to_string("input/5.txt").expect("error reading input file.");

    let lines = raw_input.lines();
    let mut pile: Vec<Vec<char>> = Vec::new();

    let first_8_lines = lines.clone().take(8);
    let first_8_lines: Vec<&str> = first_8_lines.collect();
    let first_8_lines: Vec<&str> = first_8_lines.into_iter().rev().collect();
    let num_of_stacks = ((first_8_lines[0].len() as f64 + 1.0) / 4.0).trunc() as i32;
    for _ in 0..num_of_stacks {
        pile.push(Vec::new());
    }
    let first_8_lines = first_8_lines.into_iter();
    for line in first_8_lines {
        for character in line.chars() {
            dbg!(character);
        }
    }

    // let mut pile = vec![
    //     vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
    //     vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
    //     vec!['L', 'Q', 'V'],
    //     vec!['N', 'B', 'S', 'W', 'R', 'Q'],
    //     vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
    //     vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
    //     vec!['D', 'B', 'Q', 'J'],
    //     vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
    //     vec!['B', 'N', 'H', 'M', 'S'],
    // ];
    // dbg!(pile);
}

// fn do_move(qty: i32, from: usize, to: usize) {}
