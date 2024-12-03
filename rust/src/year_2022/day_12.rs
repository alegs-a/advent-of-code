use std::collections::HashMap;

pub fn part_1(input: String) -> String {
    String::new()
}

pub fn part_2(input: String) -> String {
    String::new()
}

struct Graph {
    edges: HashMap<(i32, i32), Vec<(i32, i32)>>,
    start_node: (i32, i32),
    end_node: (i32, i32),
}

fn make_graph(input: String) -> Graph {
    let input: Vec<Vec<char>> = input.split("\n").map(|x| x.chars().collect()).collect();
    let width: i32 = input[0].len().try_into().unwrap();
    let height: i32 = input.len().try_into().unwrap();
    let mut edges: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut start_node: Option<(i32, i32)> = None;
    let mut end_node: Option<(i32, i32)> = None;
    for (y, row) in input.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            let x: i32 = x.try_into().unwrap();
            let y: i32 = y.try_into().unwrap();
            match &character {
                'S' => start_node = Some((y, x)),
                'E' => end_node = Some((y, x)),
                _ => (),
            }
            if let Some(neighbours) = edges.get_mut(&(y, x)) {
                // node already exists
                let x: usize = x.try_into().unwrap();
                let y: usize = y.try_into().unwrap();

                // get neighbours
                let deltas: Vec<i32> = vec![-1, 1];
                for x_delta in &deltas {
                    let x_looking: i32 = x as i32 + x_delta;
                    if x_looking < 0 || x_looking >= width {
                        continue;
                    }
                    let x_looking: usize = x_looking.try_into().unwrap();
                    let neighbour = input[y][x_looking];
                    // if neighbour > (character + 1) {
                    //     continue;
                    // }
                    neighbours.push((x_looking as i32, y as i32));
                }
                for y_delta in &deltas {
                    let y_looking: i32 = y as i32 + y_delta;
                    if y_looking < 0 || y_looking >= height {
                        continue;
                    }
                }
            } else {
                edges.insert((y, x), Vec::new());
            }
        }
    }

    let start_node = start_node.expect("Didn't find start node");
    let end_node = end_node.expect("Didn't find end node");

    Graph {
        edges,
        start_node,
        end_node,
    }
}
