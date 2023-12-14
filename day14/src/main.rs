use bit_set::BitSet;
use std::collections::HashMap;

use crate::Direction::{East, North, South, West};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "136");
    print_answer("one", &one(INPUT), "108792");
    print_answer("two (example)", &two(EXAMPLE), "64");
    print_answer("two", &two(INPUT), "99118");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    Platform::parse(input)
        .tilt(vec![North], 1)
        .load()
        .to_string()
}

fn two(input: &str) -> String {
    Platform::parse(input)
        .cycle(1_000_000_000)
        .load()
        .to_string()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Platform {
    size: usize,
    rocks: HashMap<Coordinate, Rock>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            size: lines.len(),
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

    fn cycle(&self, times: u32) -> Self {
        self.tilt(vec![North, West, South, East], times)
    }

    fn tilt(&self, directions: Vec<Direction>, times: u32) -> Self {
        let mut tilted = self.rocks.clone();

        let mut cache: HashMap<BitSet, u32> = HashMap::new();

        let mut time = 0;

        while time < times {
            match cache.insert(self.state(&tilted), time) {
                None => {}
                Some(old_time) => {
                    let times_to_go = times - time;
                    let repeat_every = time - old_time;
                    let skip = (times_to_go / repeat_every) * repeat_every;
                    time += skip;
                }
            }

            directions.iter().for_each(|direction| {
                (1..self.size).for_each(|lines_index| {
                    (0..self.size).for_each(|column_index| {
                        let rock = tilted.remove(&self.coordinate_for_line_column(
                            direction,
                            lines_index,
                            column_index,
                        ));

                        if let Some(rock) = rock {
                            let next_line_index = match rock {
                                Rock::Cube => lines_index,
                                Rock::Round => {
                                    let next_obstacle =
                                        (0..lines_index).rev().find(|candidate_line_index| {
                                            let candidate = self.coordinate_for_line_column(
                                                direction,
                                                *candidate_line_index,
                                                column_index,
                                            );

                                            let result = &tilted.contains_key(&candidate.clone());

                                            *result
                                        });

                                    match next_obstacle {
                                        None => 0,
                                        Some(line_index) => line_index + 1,
                                    }
                                }
                            };

                            tilted.insert(
                                self.coordinate_for_line_column(
                                    &direction,
                                    next_line_index,
                                    column_index,
                                ),
                                rock.clone(),
                            );
                        }
                    })
                });
            });

            time += 1;
        }

        Self {
            size: self.size,
            rocks: tilted,
        }
    }

    fn coordinate_for_line_column(
        &self,
        direction: &Direction,
        line_index: usize,
        column_index: usize,
    ) -> Coordinate {
        match direction {
            North => Coordinate {
                x: column_index,
                y: line_index,
            },
            East => Coordinate {
                x: self.size - line_index - 1,
                y: column_index,
            },
            South => Coordinate {
                x: self.size - column_index - 1,
                y: self.size - line_index - 1,
            },
            West => Coordinate {
                x: line_index,
                y: self.size - column_index - 1,
            },
        }
    }

    fn load(&self) -> u32 {
        self.rocks
            .iter()
            .map(|(coordinate, rock)| match rock {
                Rock::Round => (self.size - coordinate.y) as u32,
                Rock::Cube => 0u32,
            })
            .sum()
    }

    fn state(&self, map: &HashMap<Coordinate, Rock>) -> BitSet {
        let mut result = BitSet::with_capacity(self.size * self.size);

        (0..self.size).for_each(|y| {
            (0..self.size).for_each(|x| match map.get(&Coordinate { x, y }) {
                None => {}
                Some(_) => {
                    result.insert(y * self.size + x);
                }
            });
        });

        result
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rock {
    Cube,
    Round,
}
