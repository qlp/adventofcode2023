const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "114");
    print_answer("one", &one(INPUT), "1743490457");
    print_answer("two (example)", &two(EXAMPLE), "2");
    print_answer("two", &two(INPUT), "1053");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    parse(input)
        .values
        .iter()
        .map(|v| v.next())
        .sum::<i64>()
        .to_string()
}

fn two(input: &str) -> String {
    parse(input)
        .values
        .iter()
        .map(|v| v.prev())
        .sum::<i64>()
        .to_string()
}

fn parse(input: &str) -> World {
    World {
        values: input
            .lines()
            .map(|l| Value {
                history: l
                    .split_whitespace()
                    .map(|n| n.parse().expect("number"))
                    .collect(),
            })
            .collect(),
    }
}

#[derive(Debug, Clone)]
struct World {
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Value {
    history: Vec<i64>,
}

impl Value {
    fn diffs(&self) -> Vec<Vec<i64>> {
        let mut diffs: Vec<Vec<i64>> = Vec::new();
        diffs.push(self.history.clone());

        while !diffs.last().expect("not empty").iter().all(|v| *v == 0) {
            let last = diffs.last().expect("not empty");
            let next: Vec<i64> = (1..last.len()).map(|i| last[i] - last[i - 1]).collect();

            diffs.push(next)
        }

        diffs
    }

    fn next(&self) -> i64 {
        self.diffs()
            .iter()
            .map(|d| d.last().expect("at least one"))
            .sum()
    }

    fn prev(&self) -> i64 {
        self.diffs()
            .iter()
            .rev()
            .map(|d| *d.first().expect("at least one"))
            .reduce(|acc, i| i - acc)
            .expect("at least one")
    }
}
