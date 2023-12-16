use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use enum_iterator::{next_cycle, previous_cycle, Sequence};

use Direction::{Left, Up};
use Rotation::{Back, Forward};

use crate::Direction::{Down, Right};
use crate::Orientation::{Horizontal, Vertical};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "46");
    print_answer("one", &one(INPUT), "6816");
    print_answer("two (example)", &two(EXAMPLE), "51");
    print_answer("two", &two(INPUT), "8163");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    Contraption::parse(input)
        .energy(&Beam {
            point: Point { x: 0, y: 0 },
            direction: Right,
        })
        .to_string()
}

fn two(input: &str) -> String {
    let contraption = Contraption::parse(input);

    let max = (0..contraption.size)
        .flat_map(|position| {
            vec![
                Beam {
                    point: Point { x: position, y: 0 },
                    direction: Down,
                },
                Beam {
                    point: Point {
                        x: position,
                        y: contraption.size - 1,
                    },
                    direction: Up,
                },
                Beam {
                    point: Point {
                        x: contraption.size - 1,
                        y: position,
                    },
                    direction: Left,
                },
                Beam {
                    point: Point { x: 0, y: position },
                    direction: Right,
                },
            ]
        })
        .map(|beam| contraption.energy(&beam))
        .max()
        .expect("energy");

    max.to_string()
}

struct Contraption {
    size: usize,
    objects: HashMap<Point, Object>,
}

impl Contraption {
    fn parse(input: &str) -> Self {
        let size = input.lines().next().expect("line").len();
        let objects: HashMap<Point, Object> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        match c {
                            '.' => None,
                            '|' => Some(Item::Splitter(Vertical)),
                            '-' => Some(Item::Splitter(Horizontal)),
                            '\\' => Some(Item::Mirror(Back)),
                            '/' => Some(Item::Mirror(Forward)),
                            _ => panic!("unexpected char"),
                        }
                        .map(|item| {
                            (
                                Point { x, y },
                                Object {
                                    point: Point { x, y },
                                    item,
                                },
                            )
                        })
                    })
                    .collect::<Vec<(Point, Object)>>()
            })
            .collect();

        Self { size, objects }
    }

    fn trace(&self, beam: &Beam) -> (Vec<Beam>, Vec<Point>) {
        let mut point: Option<Point> = self.next(&beam.point, &beam.direction);
        let mut visited: Vec<Point> = vec![beam.point];

        loop {
            if let Some(current) = &point {
                visited.push(*current);

                match self.objects.get(current) {
                    None => {
                        point = self.next(current, &beam.direction);
                    }
                    Some(object) => {
                        return (
                            Self::beams_for_object(&beam.direction, current, &object),
                            visited,
                        )
                    }
                }
            } else {
                return (vec![], visited);
            }
        }
    }

    fn energy(&self, start: &Beam) -> usize {
        let mut active_beams: Vec<Beam> = match self.objects.get(&start.point) {
            None => vec![start.clone()],
            Some(object) => Self::beams_for_object(&start.direction, &start.point, object),
        };

        let mut processed_beams = HashSet::new();
        processed_beams.extend(&active_beams);

        let mut visited: HashSet<Point> = HashSet::new();

        while !active_beams.is_empty() {
            let traces = &active_beams
                .iter()
                .map(|beam| self.trace(&beam))
                .collect::<Vec<(Vec<Beam>, Vec<Point>)>>();

            let mut new_beams: HashSet<Beam> = HashSet::new();
            traces.iter().for_each(|(beams, points)| {
                visited.extend(points);
                new_beams.extend(beams);
            });

            active_beams = new_beams.difference(&processed_beams).copied().collect();

            processed_beams.extend(&active_beams);
        }

        visited.len()
    }

    fn beams_for_object(direction: &Direction, current: &Point, object: &Object) -> Vec<Beam> {
        match &object.item {
            Item::Mirror(rotation) => vec![Beam {
                point: *current,
                direction: direction.rotate(rotation),
            }],
            Item::Splitter(orientation) => orientation
                .split(&direction)
                .iter()
                .map(|next_direction| Beam {
                    point: *current,
                    direction: *next_direction,
                })
                .collect(),
        }
    }

    fn next(&self, point: &Point, direction: &Direction) -> Option<Point> {
        match direction {
            Up => match point.y == 0 {
                true => None,
                false => Some(Point {
                    x: point.x,
                    y: point.y - 1,
                }),
            },
            Down => match point.y + 1 == self.size {
                true => None,
                false => Some(Point {
                    x: point.x,
                    y: point.y + 1,
                }),
            },
            Left => match point.x == 0 {
                true => None,
                false => Some(Point {
                    x: point.x - 1,
                    y: point.y,
                }),
            },
            Right => match point.x + 1 == self.size {
                true => None,
                false => Some(Point {
                    x: point.x + 1,
                    y: point.y,
                }),
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Object {
    point: Point,
    item: Item,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Item {
    Mirror(Rotation),
    Splitter(Orientation),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Rotation {
    Back,
    Forward,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn split(&self, direction: &Direction) -> Vec<Direction> {
        match self {
            Horizontal => match direction {
                Left | Right => vec![direction.clone()],
                Up | Down => vec![Left, Right],
            },
            Vertical => match direction {
                Left | Right => vec![Up, Down],
                Up | Down => vec![direction.clone()],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Beam {
    point: Point,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Sequence)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(&self, rotation: &Rotation) -> Direction {
        let clockwise = match (self, rotation) {
            (Right | Left, Back) => true,
            (Up | Down, Back) => false,
            (Right | Left, Forward) => false,
            (Up | Down, Forward) => true,
        };

        match clockwise {
            true => next_cycle(self),
            false => previous_cycle(self),
        }
        .expect("to cycle")
    }
}
