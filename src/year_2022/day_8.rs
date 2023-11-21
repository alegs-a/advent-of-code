enum Direction {
    Up,
    Down,
    Left,
    Right,
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

    if x == 0 || y == 0 || y == grid.len()-1 || x == grid[0].len()-1 {
        return 0;
    }

    let right_distance = look(Direction::Right, &grid, x, y);
    let down_distance = look(Direction::Down, &grid, x, y);
    let left_distance = look(Direction::Left, &grid, x, y);
    let up_distance = look(Direction::Up, &grid, x, y);

    let scenic_score = right_distance * left_distance * up_distance * down_distance;
    scenic_score
}

fn look(dir: Direction, grid: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let x_delta: i32 = match dir {
        Direction::Left => -1,
        Direction::Right => 1,
        Direction::Up | Direction::Down => 0,
    };
    let y_delta: i32 = match dir {
        Direction::Down => 1,
        Direction::Up => -1,
        Direction::Left | Direction::Right => 0,
    };
    let mut trees_visible = 0;
    let house_height = grid[y][x];
    let mut x_looking = Some(x);
    let mut y_looking = Some(y);

    loop {
        x_looking = update_inside_bounds(x_looking, x_delta, grid[0].len()-1);
        y_looking = update_inside_bounds(y_looking, y_delta, grid.len()-1);
        if let Some(y) = y_looking {
            if let Some(x) = x_looking {
                // There is a tree (we haven't reached the edge of the map)
                trees_visible += 1;
                let tree = grid[y][x];
                if tree >= house_height {
                    return trees_visible;
                }
            } else {
                return trees_visible;
            }
        } else {
            return trees_visible;
        }
    }
}

fn update_inside_bounds(old: Option<usize>, delta: i32, max: usize) -> Option<usize> {
    let old = old.unwrap();
    let old = old as i32;
    let new = old + delta;
    if new > max as i32 || new < 0 {
        return None;
    }
    Some(new as usize)
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
        assert_eq!(part_1(input), "21");
    }

        #[test]
        fn test_part_two() {
            let input = String::from("30373
25512
65332
33549
35390");
            assert_eq!(part_2(input), "8");
        }

        #[test]
        fn test_part_two_1() {
            let input = String::from("51
11");
            assert_eq!(part_2(input), "0");
        }

        #[test]
        fn test_part_two_2() {
            let input = String::from("111
121
111");
            assert_eq!(part_2(input), "1");
        }

        #[test]
        fn test_look_simple() {
            let grid = vec!(
                vec!(2, 1),
                vec!(2, 1),
                );
            let right_distance = look(Direction::Right, &grid, 0, 0);
            assert_eq!(right_distance, 1);

            let down_distance = look(Direction::Right, &grid, 0, 0);
            assert_eq!(down_distance, 1);
        }
}
