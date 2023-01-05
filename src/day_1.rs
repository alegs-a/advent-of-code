use std::fs;

pub fn day_one() {
    println!("DAY 1 ==============");

    let input = fs::read_to_string("input/1.txt").expect("Problem reading the input file.");
    // let elves: Vec<str> = input.split("\n\n").collect();
    let mut total_calories = 0;
    let mut totals: Vec<i32> = Vec::new();
    for line in input.lines() {
        match line {
            "" => {
                println!("{}", &total_calories);
                totals.push(total_calories);
                total_calories = 0;
            }
            _ => {
                let calories: i32 = line.parse().unwrap();
                total_calories += calories;
            }
        }
    }

    println!("highest calorie count: {}", &totals.iter().max().unwrap());

    totals.sort();
    totals.reverse();
    println!(
        "The calories carried by the three elves with the highest totals is: {}",
        totals[0] + totals[1] + totals[2]
    );
}
