use std::collections::HashMap;

pub fn part_1(input: String) -> String {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    String::new()
}

pub fn part_2(input: String) -> String {
    String::new()
}

//fn parse_workflows(input: &str) -> HashMap<&str, Vec<Test>> {
//    for line in input.lines() {
//        let (name, workflow) = line.split_once("{").unwrap();
//        let tests = workflow.split(",").map(|x| {
//
//        })
//    }
//}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
//    fn new(input: &str) -> Part {
//        let input = input.replace("{", "");
//        let input = input.replace("}", "");
//        let stats: Vec<i32> = input
//            .split(",")
//            .map(|x| {
//                let (_, val) = x.split_once("=").unwrap();
//                val.parse::<i32>()
//            })
//            .collect();
//        let x = stats[0];
//        let m = stats[1];
//        let a = stats[2];
//        let s = stats[3];
//        Part { x, m, a, s }
//    }
}

enum PartType {
    X,
    M,
    A,
    S,
}

enum TestType {
    LessThan,
    GreaterThan,
}

enum Destination<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

struct Test<'a> {
    param: PartType,
    test: TestType,
    destination: Destination<'a>,
}

impl Test<'_> {
//    fn new(input: &str) -> Test {
//        let (test, dest) = input.split_once(":").unwrap();
//        let chars: Vec<char> = test.chars().collect();
//        let part_type = chars[0];
//        let test = chars[1];
//        //let 
//    }
}
