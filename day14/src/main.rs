use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "136");
    print_answer("one", &one(INPUT), "");
    // print_answer("two (example)", &two(EXAMPLE), "");
    // print_answer("two", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let platform = Platform::parse(input);

    println!("{platform}");

    let platform = platform.tilt_north();

    println!("{platform}");

    platform.load().to_string()
}

fn two(input: &str) -> String {
    String::new()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Platform {
    size: Size,
    rocks: HashMap<Coordinate, Rock>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            size: Size {
                width: lines.first().expect("at least one").len(),
                height: lines.len(),
            },
            rocks: lines
                .iter()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, char)| match char {
                            '#' => Some((Coordinate { x, y }, Rock::Cube)),
                            'O' => Some((Coordinate { x, y }, Rock::Round)),
                            '.' => None,
                            _ => panic!("unexpected char"),
                        })
                        .collect::<Vec<(Coordinate, Rock)>>()
                })
                .collect::<HashMap<Coordinate, Rock>>(),
        }
    }

    fn tilt_north(&self) -> Self {
        let mut tilted = self.rocks.clone();

        (1..self.size.height).for_each(|y| {
            (0..self.size.width).for_each(|x| {
                let rock = tilted.remove(&Coordinate { x, y });

                if let Some(rock) = rock {
                    let next_y = match rock {
                        Rock::Cube => y,
                        Rock::Round => {
                            let next_obstacle = (0..y).rev().find(|next_y| {
                                let candidate = Coordinate { x, y: *next_y };

                                let result = &tilted.contains_key(&candidate.clone());

                                *result
                            });

                            match next_obstacle {
                                None => 0,
                                Some(y) => y + 1,
                            }
                        }
                    };

                    tilted.insert(Coordinate { x, y: next_y }, rock.clone());
                }
            })
        });

        Self {
            size: self.size.clone(),
            rocks: tilted,
        }
    }

    fn load(&self) -> u32 {
        self.rocks
            .iter()
            .map(|(coordinate, rock)| match rock {
                Rock::Round => (self.size.height - coordinate.y) as u32,
                Rock::Cube => 0u32,
            })
            .sum()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.size.height).for_each(|y| {
            (0..self.size.width).for_each(|x| match self.rocks.get(&Coordinate { x, y }) {
                None => f.write_char('.').unwrap(),
                Some(r) => r.fmt(f).unwrap(),
            });
            f.write_char('\n').unwrap()
        });

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Size {
    width: usize,
    height: usize,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.width, self.height))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rock {
    Cube,
    Round,
}

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Rock::Cube => '#',
            Rock::Round => 'O',
        })
    }
}
