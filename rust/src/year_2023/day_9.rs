use itertools::Itertools;

pub fn part_1(input: String) -> String {
    let sequences = parse(input);
    let mut sum = 0i128;

    for sequence in sequences {
        sum += predict(sequence);
    }

    sum.to_string()
}

pub fn part_2(input: String) -> String {
    let sequences = parse(input);
    let mut sum = 0i128;

    for sequence in sequences {
        sum += predict_backwards(sequence);
    }

    sum.to_string()
}

fn parse(input: String) -> Vec<Vec<i128>> {
    let mut output = Vec::new();
    for line in input.lines() {
        output.push(line.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect());
    }
    output
}

fn predict(sequence: Vec<i128>) -> i128 {
    let diffs: Vec<i128> = sequence.iter().tuple_windows().map(|(curr, next)| next-curr).collect();
    if is_zero(&sequence) {
        return 0;
    }
    sequence.last().unwrap() + predict(diffs)
}

fn predict_backwards(sequence: Vec<i128>) -> i128 {
    let diffs: Vec<i128> = sequence.iter().tuple_windows().map(|(curr, next)| next-curr).collect();
    if is_zero(&sequence) {
        return 0;
    }
    sequence.first().unwrap() - predict_backwards(diffs)
}

fn is_zero(sequence: &Vec<i128>) -> bool {
    for num in sequence {
        if *num != 0 {
            return false
        }
    }
    true
}

#[cfg(test)] 
mod test {
    use super::*;
    #[test]
    fn test_predict_zeroes() {
        let input = vec![0,0,0,0];
        let expected = 0;
        assert_eq!(predict(input), expected);
    }

    #[test]
    fn test_predict_all_threes() {
        let input = vec![3,3,3,3,3,3];
        let expected = 3;
        assert_eq!(predict(input), expected);
    }

    #[test]
    fn test_predict_all_asdfasdf() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = 18;
        assert_eq!(predict(input), expected);
    }
}
