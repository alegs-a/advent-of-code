use std::fs;

pub fn day_eight() {
    let raw_input = fs::read_to_string("input/8.txt").expect("error reading input file");

    println!("DAY 8 ============");
    println!("Part 1: {}", part_one(raw_input.clone()));
    println!("Part 2: {}", part_two(raw_input.clone()));
}

pub fn part_1(raw_input: String) -> String {
    let grid = make_grid(raw_input);
    let height = grid.len();
    let width = grid[0].len();

    // FINDING ITEMS IS grid[y][x] !!!!!!!!!!!
    // Y COMES FIRST!!!!!!!!!

    let mut tallest_trees: Vec<Tree> = Vec::new();

    // West view
    for y in 0..height {
        let mut tallest_so_far = Tree {
            height: -1,
            x: 0,
            y,
        };
        for x in 0..width {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // East view
    for y in 0..height {
        let mut tallest_so_far = Tree {
            height: -1,
            x: width - 1,
            y,
        };
        for x in (0..width).rev() {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // North view
    for x in 0..width {
        let mut tallest_so_far = Tree {
            height: -1,
            x,
            y: 0,
        };
        for y in 0..height {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // South view
    for x in 0..width {
        let mut tallest_so_far = Tree {
            height: -1,
            x,
            y: height - 1,
        };
        for y in (0..height).rev() {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    tallest_trees.len().to_string()
}

pub fn part_2(raw_input: String) -> String {
    let grid = make_grid(raw_input);
    let height = grid.len();
    let width = grid[0].len();

    let mut highest_score = 0;

    for y in 0..height {
        for x in 0..width {
            let score = scenic_score(&grid, x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    highest_score.to_string()
}

fn part_one(raw_input: String) -> i32 {
    let grid = make_grid(raw_input);
    let height = grid.len();
    let width = grid[0].len();

    // FINDING ITEMS IS grid[y][x] !!!!!!!!!!!
    // Y COMES FIRST!!!!!!!!!

    let mut tallest_trees: Vec<Tree> = Vec::new();

    // West view
    for y in 0..height {
        let mut tallest_so_far = Tree {
            height: -1,
            x: 0,
            y,
        };
        for x in 0..width {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // East view
    for y in 0..height {
        let mut tallest_so_far = Tree {
            height: -1,
            x: width - 1,
            y,
        };
        for x in (0..width).rev() {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // North view
    for x in 0..width {
        let mut tallest_so_far = Tree {
            height: -1,
            x,
            y: 0,
        };
        for y in 0..height {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // South view
    for x in 0..width {
        let mut tallest_so_far = Tree {
            height: -1,
            x,
            y: height - 1,
        };
        for y in (0..height).rev() {
            if grid[y][x] > tallest_so_far.height {
                let current_tree = Tree {
                    height: grid[y][x],
                    x,
                    y,
                };
                if !tallest_trees.contains(&current_tree) {
                    tallest_trees.push(current_tree);
                }
                tallest_so_far = current_tree;
            }
        }
    }

    // dbg!(&tallest_trees);
    tallest_trees.len() as i32
}

fn part_two(raw_input: String) -> i32 {
    let grid = make_grid(raw_input);
    let height = grid.len();
    let width = grid[0].len();

    let mut highest_score = 0;

    for y in 0..height {
        for x in 0..width {
            let score = scenic_score(&grid, x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    highest_score
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Tree {
    height: i32,
    x: usize,
    y: usize,
}

fn make_grid(raw_input: String) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let lines: Vec<&str> = raw_input.lines().collect();

    for line in lines {
        grid.push(
            line.chars()
                .map(|x| {
                    x.to_digit(10)
                        .unwrap()
                        .try_into()
                        .expect("parsing char '{x}' gone wrong")
                })
                .collect(),
        )
    }
    grid
}

fn scenic_score(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut scenic_score: i32 = 1;

    if x == 0 || y == 0 || y == grid.len() || x == grid[0].len() {
        return 0;
    }

    for east_offset in 1..(grid[0].len() - x + 1) {
        if east_offset == grid[0].len() - x {
            scenic_score *= east_offset as i32;
            break;
        }
        if grid[y][x] <= grid[y][x + east_offset] {
            // Catch edge of map - if east_offset == x or
            // smth
            scenic_score *= east_offset as i32;
            // dbg!(x, y, east_offset);
            break;
        }
    }

    for west_offset in 1..x {
        // dbg!(x, y, west_offset);
        if grid[y][x] <= grid[y][x - west_offset] {
            scenic_score *= west_offset as i32;
            // dbg!(x, y, west_offset);
            break;
        }
    }

    for north_offset in 1..grid.len() - y {
        if grid[y][x] <= grid[y + north_offset][x] {
            scenic_score *= north_offset as i32;
            // dbg!(x, y, north_offset);
            break;
        }
    }

    for south_offset in 1..y {
        if grid[y][x] <= grid[y - south_offset][x] {
            scenic_score *= south_offset as i32;
            // dbg!(x, y, south_offset);
            break;
        }
    }
    // dbg!(scenic_score);
    scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from(
            "30373
25512
65332
33549
35390",
        );
        assert_eq!(part_one(input), 21);
    }

    //     #[test]
    //     fn test_part_two() {
    //         let input = String::from("30373
    // 25512
    // 65332
    // 33549
    // 35390");
    //         assert_eq!(part_two(input), 8);
    //     }
}
