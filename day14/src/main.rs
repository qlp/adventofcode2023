use bit_set::BitSet;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

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
    cube_rocks: BitSet,
    round_rocks: BitSet,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let size = lines.len();
        let mut cube_rocks = BitSet::with_capacity(size * size);
        let mut round_rocks = BitSet::with_capacity(size * size);

        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                match char {
                    '#' => {
                        cube_rocks.set(size, x, y);
                    }
                    'O' => {
                        round_rocks.set(size, x, y);
                    }
                    '.' => {}
                    _ => panic!("unexpected char"),
                };
            });
        });

        Self {
            size,
            cube_rocks,
            round_rocks,
        }
    }

    fn cycle(&self, times: u32) -> Self {
        self.tilt(vec![North, West, South, East], times)
    }

    fn tilt(&self, directions: Vec<Direction>, times: u32) -> Self {
        let mut new_round_rocks = self.round_rocks.clone();

        let mut cache: HashMap<BitSet, u32> = HashMap::new();

        let mut time = 0;

        while time < times {
            match cache.insert(new_round_rocks.clone(), time) {
                None => {}
                Some(old_time) => {
                    let times_to_go = times - time;
                    let repeat_every = time - old_time;
                    let skip = (times_to_go / repeat_every) * repeat_every;
                    time += skip;
                }
            }

            directions.iter().for_each(|direction| {
                (1..self.size).for_each(|line_index| {
                    (0..self.size).for_each(|column_index| {
                        let original_coordinate =
                            self.coordinate_for_line_column(direction, line_index, column_index);

                        let has_round_rock = new_round_rocks.get(
                            self.size,
                            original_coordinate.x,
                            original_coordinate.y,
                        );

                        if has_round_rock {
                            let next_obstacle =
                                (0..line_index).rev().find(|candidate_obstacle_line_index| {
                                    let candidate_obstacle_coordinates = self
                                        .coordinate_for_line_column(
                                            direction,
                                            *candidate_obstacle_line_index,
                                            column_index,
                                        );

                                    new_round_rocks.get(
                                        self.size,
                                        candidate_obstacle_coordinates.x,
                                        candidate_obstacle_coordinates.y,
                                    ) || self.cube_rocks.get(
                                        self.size,
                                        candidate_obstacle_coordinates.x,
                                        candidate_obstacle_coordinates.y,
                                    )
                                });

                            let next_line_index = match next_obstacle {
                                None => 0,
                                Some(line_index) => line_index + 1,
                            };

                            if next_line_index != line_index {
                                new_round_rocks.unset(
                                    self.size,
                                    original_coordinate.x,
                                    original_coordinate.y,
                                );

                                let new_coordinate = self.coordinate_for_line_column(
                                    &direction,
                                    next_line_index,
                                    column_index,
                                );

                                new_round_rocks.set(self.size, new_coordinate.x, new_coordinate.y);
                            }
                        }
                    })
                });
            });

            time += 1;
        }

        Self {
            size: self.size,
            round_rocks: new_round_rocks,
            cube_rocks: self.cube_rocks.clone(),
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

    fn load(&self) -> usize {
        self.round_rocks
            .iter()
            .map(|n| self.size - (n / self.size))
            .sum()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.size).for_each(|y| {
            (0..self.size).for_each(|x| {
                let char: char = match (
                    self.round_rocks.get(self.size, x, y),
                    self.cube_rocks.get(self.size, x, y),
                ) {
                    (true, false) => 'O',
                    (false, true) => '#',
                    (false, false) => '.',
                    (true, true) => panic!("can't be round and cube"),
                };

                f.write_char(char);
            });
            f.write_char('\n');
        });

        Ok(())
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

trait Coordinatable {
    fn set(&mut self, size: usize, x: usize, y: usize) -> bool;
    fn unset(&mut self, size: usize, x: usize, y: usize) -> bool;
    fn get(&self, size: usize, x: usize, y: usize) -> bool;
}

impl Coordinatable for BitSet {
    fn set(&mut self, size: usize, x: usize, y: usize) -> bool {
        self.insert(y * size + x)
    }

    fn unset(&mut self, size: usize, x: usize, y: usize) -> bool {
        self.remove(y * size + x)
    }

    fn get(&self, size: usize, x: usize, y: usize) -> bool {
        self.contains(y * size + x)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate {
    x: usize,
    y: usize,
}
