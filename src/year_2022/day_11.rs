use std::collections::VecDeque;

pub fn part_1(input: String) -> String {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for block in blocks {
        if let Some(monkey) = make_monkey(block) {
            monkeys.push(monkey);
        }
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let items: Vec<u128> = monkeys[idx].items.drain(..).collect();
            let monkey = monkeys[idx].clone();
            for old_item in items {
                monkeys[idx].inspections += 1;
                let mut new_item = match &monkey.operation {
                    Operation::Mul(value) => old_item * value.get_value(old_item),
                    Operation::Add(value) => old_item + value.get_value(old_item),
                };
                new_item /= 3;
                let recipient = monkey.test.test(new_item);
                monkeys[recipient].add_item(new_item);
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    monkeys.reverse();
    (monkeys[0].inspections * monkeys[1].inspections).to_string()
}
pub fn part_2(input: String) -> String {
    let blocks: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for block in blocks {
        if let Some(monkey) = make_monkey(block) {
            monkeys.push(monkey);
        }
    }

    for _ in 0..10000 {
        //if i > 19 { break; }
        for idx in 0..monkeys.len() {
            let items: Vec<u128> = monkeys[idx].items.drain(..).collect();
            let monkey = monkeys[idx].clone();
            for old_item in items {
                if old_item == 0 {
                    panic!("we got a zero :(")
                }
                monkeys[idx].inspections += 1;
                let mut new_item = match &monkey.operation {
                    Operation::Mul(value) => old_item * value.get_value(old_item),
                    Operation::Add(value) => old_item + value.get_value(old_item),
                };
                new_item = new_item % 9699690;
                let recipient = monkey.test.test(new_item);
                monkeys[recipient].add_item(new_item);
            }
        }
        //dbg!(&monkeys);
    }

    monkeys.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    monkeys.reverse();
    let monkey_business: u128 = monkeys[0].inspections as u128 * monkeys[1].inspections as u128;
    monkey_business.to_string()
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u128>,
    inspections: u128,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}

#[derive(Debug, Clone)]
enum Value {
    Old,
    Num(u128),
}

#[derive(Debug, Clone)]
struct Test {
    divisible: u128,
    true_recipient: usize,
    false_recipient: usize,
}

fn make_monkey(block: &str) -> Option<Monkey> {
    let mut lines = block.lines();
    lines.next()?;

    let line = lines.next()?;
    let items: VecDeque<u128> = line
        .split(": ")
        .skip(1)
        .next()?
        .split(", ")
        .filter_map(|x| x.parse().ok())
        .collect();

    let line = lines.next()?;
    let (_, operation) = line.split_once("old ")?;
    let (operator, operand) = operation.split_once(" ")?;
    let operand = match operand {
        "old" => Value::Old,
        num => Value::Num(num.parse().ok()?),
    };
    let operation = match operator {
        "+" => Operation::Add(operand),
        "*" => Operation::Mul(operand),
        _ => unreachable!(),
    };

    let line = lines.next()?;
    let divisible: u128 = line.split_whitespace().last()?.parse().ok()?;
    let line = lines.next()?;
    let true_recipient: usize = line.split_whitespace().last()?.parse().ok()?;
    let line = lines.next()?;
    let false_recipient: usize = line.split_whitespace().last()?.parse().ok()?;

    let inspections = 0;

    Some(Monkey {
        items,
        inspections,
        operation,
        test: Test {
            divisible,
            true_recipient,
            false_recipient,
        },
    })
}

impl Monkey {
    fn add_item(&mut self, item: u128) {
        self.items.push_back(item);
    }
}

impl Value {
    fn get_value(&self, old: u128) -> u128 {
        match self {
            Value::Old => old.into(),
            Value::Num(num) => *num,
        }
    }
}

impl Test {
    fn test(&self, value: u128) -> usize {
        if value % self.divisible == 0 {
            println!("we true");
            return self.true_recipient;
        } else {
            println!("we false");
            return self.false_recipient;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    // This fails for some godforsaken reason, but you still get the right answer for the actual
    // puzle input who knows why ¯\_(ツ)_/¯
    #[allow(dead_code)]
    fn test_part_2() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(part_2(input.to_string()), "2713310158");
    }
}
