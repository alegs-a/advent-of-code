use std::fs;

pub fn day_ten() {
    let raw_input = fs::read_to_string("input/10.txt")
        .expect("Failed to read input file");

    println!("PART 1: {}", part_one(raw_input));
}

fn part_one(input: String) -> i32 {
    let mut cycle = 0;
    let mut x = 1;

    let summed_strengths = 0;
    let mut cycles_this_instruction_completed = 0;
    let mut cycles_this_instruction_required = 0;
    let mut value_to_add = 0;
    let mut completed = false;

    let cycles_to_addx = 2;
    let cycles_to_noop = 1;

    let mut lines = input.lines();
    loop {
        if (cycle - 20) % 40 == 0 {
            println!("Cycle: {cycle}, signal strength: {}", cycle * x);
        }
        if cycles_this_instruction_completed == cycles_this_instruction_required {
            if cycles_this_instruction_required == cycles_to_addx {
                x += value_to_add;
            }
            cycles 
        } else {
            cycle += 1;
        }
        match lines.next() {
            Some(line) => {
                let mut words = line.split_whitespace();
                match words.next().unwrap() {
                    "addx" => {
                        // do stuff
                        cycles_this_instruction_completed = 0;
                        cycles_this_instruction_required = 2;
                        value_to_add = words.next().unwrap().parse::<i32>().unwrap();
                    }
                    "noop" => {
                        // do stuff?
                        cycles_this_instruction_completed = 0;
                        cycles_this_instruction_required = 1;
                    }
                    _ => panic!("Line was empty")
                }
            }
            None => {
                // do stuff
                println!("why?");
                break;
            }
        }
    }
    summed_strengths
}

enum Operation {
    /// Contains the value to be added to the x register
    Addx(i32),
}
