use std::cmp::max;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "8");
    print_answer("one", &one(INPUT), "2061");
    print_answer("two (example)", &two(EXAMPLE), "2286");
    print_answer("two", &two(INPUT), "72596");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Subset {
    fn product(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn zero() -> Self {
        Self { red: 0, blue: 0, green: 0 }
    }

    fn se(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

#[derive(Debug)]
struct Subset {
    red: u32,
    blue: u32,
    green: u32,
}

fn one(input: &str) -> String {
    parse(input)
        .iter()
        .filter(|g| g.subsets
            .iter()
            .all(|s| s.se(&Subset { red: 12, green: 13, blue: 14 })))
        .map(|g| g.id)
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    parse(input)
        .iter()
        .map(|g| g.subsets
            .iter()
            .fold(Subset::zero(), |result, candidate| result.max(candidate)))
        .map(|s| s.product())
        .sum::<u32>()
        .to_string()
}

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| -> Game {
            Game {
                id: line
                    .split(": ")
                    .next()
                    .expect("no semicolon")
                    .split(' ')
                    .last()
                    .expect("no space")
                    .parse()
                    .expect("not a number"),
                subsets: line
                    .split(": ")
                    .last()
                    .expect("no semicolon")
                    .split("; ")
                    .map(|s| -> HashMap<&str, u32> {
                        s
                            .split(", ")
                            .map(|s| -> (&str, u32) {
                                (
                                    s
                                        .split(' ')
                                        .last()
                                        .expect("expect a space"),
                                    s
                                        .split(' ')
                                        .next()
                                        .expect("expect a space")
                                        .parse()
                                        .expect("expect a number")
                                )
                            })
                            .collect()
                    }
                    )
                    .map(|s|
                        Subset {
                            red: s.get("red").copied().unwrap_or(0),
                            blue: s.get("blue").copied().unwrap_or(0),
                            green: s.get("green").copied().unwrap_or(0),
                        })
                    .collect(),
            }
        })
        .collect()
}
