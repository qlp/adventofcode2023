use crate::Orientation::{Down, Right};
use crate::Part::{One, Two};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use Orientation::{Left, Up};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE_1), "102");
    print_answer("one", &one(INPUT), "724");
    print_answer("two (example 1)", &two(EXAMPLE_1), "94");
    print_answer("two (example 2)", &two(EXAMPLE_2), "71");
    print_answer("two", &two(INPUT), "877");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    HeatMap::parse(input, One).minimum_temperature().to_string()
}

fn two(input: &str) -> String {
    HeatMap::parse(input, Two).minimum_temperature().to_string()
}

struct HeatMap {
    size: Size,
    values: Vec<u32>,
    part: Part,
}

enum Part {
    One,
    Two,
}

struct Size {
    width: usize,
    height: usize,
}

impl HeatMap {
    fn parse(input: &str, part: Part) -> Self {
        Self {
            size: Size {
                width: input.lines().next().expect("line").len(),
                height: input.lines().collect::<Vec<&str>>().len(),
            },
            values: input
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).expect("digit")))
                .collect(),
            part: part,
        }
    }

    fn get_by_coordinates(&self, x: usize, y: usize) -> u32 {
        self.values[y * self.size.width + x]
    }

    fn get_by_position(&self, position: &Position) -> u32 {
        self.get_by_coordinates(position.x, position.y)
    }

    fn next_crucible_drivers(&self, crucible_driver: &CrucibleDriver) -> Vec<CrucibleDriver> {
        [Turn::Left, Turn::Straight, Turn::Right]
            .iter()
            .filter_map(|turn| self.next_crucible(&crucible_driver.crucible, turn))
            .map(|next_crucible| CrucibleDriver {
                crucible: next_crucible.clone(),
                temperature: crucible_driver.temperature
                    + self.get_by_position(&next_crucible.position),
            })
            .collect()
    }

    fn next_crucible(&self, crucible: &Crucible, turn: &Turn) -> Option<Crucible> {
        let orientation = crucible.orientation.turn(turn);
        match self.part {
            One => match self.next_position(&crucible.position, orientation) {
                None => None,
                Some(position) => match (turn, crucible.straight_count) {
                    (Turn::Straight, 2) => None,
                    (Turn::Straight, _) => Some(Crucible {
                        position: position,
                        orientation: orientation.clone(),
                        straight_count: crucible.straight_count + 1,
                    }),
                    (_, _) => Some(Crucible {
                        position: position,
                        orientation: orientation.clone(),
                        straight_count: 0,
                    }),
                },
            },
            Two => match self.next_position(&crucible.position, orientation) {
                None => None,
                Some(position) => match (turn, crucible.straight_count) {
                    (Turn::Straight, 10) => None,
                    (Turn::Straight, _) => Some(Crucible {
                        position: position,
                        orientation: orientation.clone(),
                        straight_count: crucible.straight_count + 1,
                    }),
                    (_, 4..) => Some(Crucible {
                        position: position,
                        orientation: orientation.clone(),
                        straight_count: 1,
                    }),
                    _ => None,
                },
            },
        }
    }

    fn next_position(&self, position: &Position, orientation: &Orientation) -> Option<Position> {
        let left_border = position.x == 0;
        let top_border = position.y == 0;
        let right_border = position.x + 1 == self.size.width;
        let bottom_border = position.y + 1 == self.size.height;

        match (
            orientation,
            left_border,
            top_border,
            right_border,
            bottom_border,
        ) {
            (Up, _, true, _, _) => None,
            (Up, _, _, _, _) => Some(Position {
                x: position.x,
                y: position.y - 1,
            }),
            (Down, _, _, _, true) => None,
            (Down, _, _, _, _) => Some(Position {
                x: position.x,
                y: position.y + 1,
            }),
            (Left, true, _, _, _) => None,
            (Left, _, _, _, _) => Some(Position {
                x: position.x - 1,
                y: position.y,
            }),
            (Right, _, _, true, _) => None,
            (Right, _, _, _, _) => Some(Position {
                x: position.x + 1,
                y: position.y,
            }),
        }
    }

    fn minimum_temperature(&self) -> u32 {
        let mut history: HashMap<Crucible, u32> =
            HashMap::with_capacity(self.size.width * self.size.height * 10);

        let right = CrucibleDriver {
            crucible: Crucible {
                position: Position { x: 0, y: 0 },
                orientation: Right,
                straight_count: 0,
            },
            temperature: 0,
        };

        let down = CrucibleDriver {
            crucible: Crucible {
                position: Position { x: 0, y: 0 },
                orientation: Down,
                straight_count: 0,
            },
            temperature: 0,
        };

        history.insert(right.crucible.clone(), 0);
        history.insert(down.crucible.clone(), 0);

        let mut next_crucible_drivers: Vec<CrucibleDriver> = Vec::new();
        let left_drivers = self.next_crucible_drivers(&right);
        next_crucible_drivers.extend(left_drivers.clone());
        let down_drivers = self.next_crucible_drivers(&down);
        next_crucible_drivers.extend(down_drivers.clone());

        while !next_crucible_drivers.is_empty() {
            let mut best_drivers: HashSet<CrucibleDriver> = HashSet::new();

            let next_drivers: Vec<CrucibleDriver> = next_crucible_drivers
                .iter()
                .flat_map(|crucible| self.next_crucible_drivers(crucible))
                .collect();

            next_drivers
                .iter()
                .for_each(|driver| match history.get(&driver.crucible) {
                    None => {
                        best_drivers.insert(driver.clone());
                        history.insert(driver.crucible.clone(), driver.temperature);
                    }
                    Some(opponent_temperature) => {
                        if *opponent_temperature > driver.temperature {
                            best_drivers.insert(driver.clone());
                            history.insert(driver.crucible.clone(), driver.temperature);
                        }
                    }
                });

            next_crucible_drivers = Vec::from_iter(best_drivers);
        }

        *history
            .iter()
            .filter(|(crucible, _)| {
                match (crucible.position.x == self.size.width - 1)
                    && (crucible.position.y == self.size.height - 1)
                {
                    true => match self.part {
                        One => true,
                        Two => crucible.straight_count > 3,
                    },
                    false => false,
                }
            })
            .map(|(_, value)| value)
            .min()
            .expect("solution")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Crucible {
    position: Position,
    orientation: Orientation,
    straight_count: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct CrucibleDriver {
    crucible: Crucible,
    temperature: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn(&self, turn: &Turn) -> &Orientation {
        match turn {
            Turn::Straight => self,
            Turn::Left => match self {
                Up => &Left,
                Down => &Right,
                Left => &Down,
                Right => &Up,
            },
            Turn::Right => match self {
                Up => &Right,
                Down => &Left,
                Left => &Up,
                Right => &Down,
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Turn {
    Straight,
    Left,
    Right,
}
