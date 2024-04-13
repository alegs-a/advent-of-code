pub fn part_1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::new(line).unwrap();
        if game.is_possible() {
            sum += game.id;
        }
    }
    sum.to_string()
}

pub fn part_2(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::new(line).unwrap();
        sum += game.power().unwrap();
    }
    sum.to_string()
}

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

struct Draw {
    r: i32,
    g: i32,
    b: i32,
}

impl Game {
    fn new(line: &str) -> Option<Game> {
        let (id, rest) = line.split_once(": ")?;
        let id = id.split_whitespace().last()?.parse::<i32>().ok()?;

        let draws: Vec<Draw> = rest
            .split("; ")
            .map(|x| {
                let values = x.split(", ").map(|value| {
                    let Some((num, colour)) = value.split_once(" ") else {
                        unreachable!()
                    };
                    (num.parse::<i32>().unwrap(), colour)
                });

                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for value in values {
                    match value.1 {
                        "red" => r = value.0,
                        "green" => g = value.0,
                        "blue" => b = value.0,
                        _ => unreachable!(),
                    }
                }

                Draw { r, g, b }
            })
            .collect();

        Some(Game { id, draws })
    }

    fn is_possible(&self) -> bool {
        const MAX_R: i32 = 12;
        const MAX_G: i32 = 13;
        const MAX_B: i32 = 14;
        for draw in &self.draws {
            if draw.r > MAX_R || draw.g > MAX_G || draw.b > MAX_B {
                return false;
            }
        }
        true
    }

    fn power(&self) -> Option<i32> {
        let min_r = self.draws.iter().map(|x| x.r).max()?;
        let min_g = self.draws.iter().map(|x| x.g).max()?;
        let min_b = self.draws.iter().map(|x| x.b).max()?;

        Some(min_r * min_g * min_b)
    }
}
