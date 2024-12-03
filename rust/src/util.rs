/// Returns all the integers in a line. Assumes they are positive.
fn get_ints(line: &str) -> Vec<i32> {
    let mut nums: Vec<Option<i32>> = Vec::new();
    let mut cur = String::new();
    for char in line.chars() {
        if !char.is_digit(10) {
            if cur.len() > 0 {
                nums.push(Some(cur.parse().unwrap()))
            }
            cur = String::new();
        }
    }

    nums.iter()
        .filter(|x| !x.is_none())
        .map(|val| match val {
            Some(x) => x.clone(),
            None => unreachable!(),
        })
        .collect()
}
