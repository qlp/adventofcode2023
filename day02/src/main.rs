use std::cmp::max;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "8");
    print_answer("one", &one(INPUT), "2061");
    print_answer("two (example)", &two(EXAMPLE), "2286");
    print_answer("two", &two(INPUT), "54265");
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

#[derive(Debug)]
struct Subset {
    red: u32,
    blue: u32,
    green: u32,
}

fn one(input: &str) -> String {
    input
        .lines()
        .map(|line| -> Game {
            Game {
                id: line.split(": ").next().expect("no semicolon").split(" ").last().expect("no space").parse().expect("not a number"),
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
                                        .split(" ")
                                        .last()
                                        .expect("expect a space"),
                                    s
                                        .split(" ")
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
        .filter(|g|
            g.subsets.iter().all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
        )
        .map(|g| g.id)
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    input
        .lines()
        .map(|line| -> Game {
            Game {
                id: line.split(": ").next().expect("no semicolon").split(" ").last().expect("no space").parse().expect("not a number"),
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
                                        .split(" ")
                                        .last()
                                        .expect("expect a space"),
                                    s
                                        .split(" ")
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
        .map(|g| g.subsets.iter().fold(Subset { red: 0, blue: 0, green: 0,}, |result, candidate|
            Subset {
                red: max(result.red, candidate.red),
                green: max(result.green, candidate.green),
                blue: max(result.blue, candidate.blue),
            }
        ))
        .map(|s| s.red * s.blue * s.green)
        .sum::<u32>()
        .to_string()
}
