use crate::Orientation::{Down, Right};
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Write};
use Orientation::{Left, Up};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "102");
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
    let heat_map = HeatMap::parse(input);

    let mut history: HashMap<Crucible, u32> = HashMap::new();

    let right = CrucibleDriver {
        crucible: Crucible {
            position: Position { x: 0, y: 0 },
            orientation: Right,
            straight_count: 0,
        },
        path: Path::start(&Right),
        temperature: 0,
    };

    let down = CrucibleDriver {
        crucible: Crucible {
            position: Position { x: 0, y: 0 },
            orientation: Down,
            straight_count: 0,
        },
        path: Path::start(&Down),
        temperature: 0,
    };

    history.insert(right.crucible.clone(), 0);
    history.insert(down.crucible.clone(), 0);

    let mut next_crucible_drivers: Vec<CrucibleDriver> = Vec::new();
    let left_drivers = heat_map.next_crucible_drivers(&right);
    next_crucible_drivers.extend(left_drivers.clone());
    let down_drivers = heat_map.next_crucible_drivers(&down);
    next_crucible_drivers.extend(down_drivers.clone());

    while !next_crucible_drivers.is_empty() {
        // next_crucible_drivers.iter().for_each(|driver| {
        //     heat_map.print_driver(driver);
        //     println!("-----------------");
        // });
        // println!("=====================");

        let mut best_drivers: HashSet<CrucibleDriver> = HashSet::new();

        let next_drivers: Vec<CrucibleDriver> = next_crucible_drivers
            .iter()
            .flat_map(|crucible| heat_map.next_crucible_drivers(crucible))
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

    history
        .iter()
        .filter_map(|(crucible, temperature)| {
            match crucible.position.eq(&Position {
                x: heat_map.size - 1,
                y: heat_map.size - 1,
            }) {
                true => Some(temperature),
                false => None,
            }
        })
        .min()
        .expect("one")
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct HeatMap {
    size: usize,
    values: Vec<u32>,
}

impl HeatMap {
    fn parse(input: &str) -> Self {
        Self {
            size: input.lines().next().expect("line").len(),
            values: input
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_digit(10).expect("digit")))
                .collect(),
        }
    }

    fn get_by_coordinates(&self, x: usize, y: usize) -> u32 {
        self.values[y * self.size + x]
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
                path: crucible_driver
                    .path
                    .with(&next_crucible.position, &next_crucible.orientation),
                temperature: crucible_driver.temperature
                    + self.get_by_position(&next_crucible.position),
            })
            .collect()
    }

    fn next_crucible(&self, crucible: &Crucible, turn: &Turn) -> Option<Crucible> {
        let orientation = crucible.orientation.turn(turn);
        match self.next_position(&crucible.position, orientation) {
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
        }
    }

    fn print_driver(&self, driver: &CrucibleDriver) {
        (0..self.size).for_each(|y| {
            (0..self.size).for_each(|x| {
                let position = Position { x, y };
                match driver
                    .path
                    .positions
                    .iter()
                    .find(|(past_position, _)| position.eq(past_position))
                {
                    None => {
                        print!("{}", self.get_by_coordinates(x, y));
                    }
                    Some((_, orientation)) => match orientation {
                        Up => print!("^"),
                        Down => print!("v"),
                        Left => print!("<"),
                        Right => print!(">"),
                    },
                }
            });
            println!();
        });
        println!();

        let mut temperature = 0;

        driver
            .path
            .positions
            .iter()
            .for_each(|(position, orientation)| {
                temperature += self.get_by_position(position);
                print!(
                    "{}, {} {} {}° => ",
                    position.x,
                    position.y,
                    match orientation {
                        Up => "^",
                        Down => "v",
                        Left => "<",
                        Right => ">",
                    },
                    temperature,
                )
            });
        print!(" (driver temperature: {}°)", driver.temperature);
        println!();
    }

    fn next_position(&self, position: &Position, orientation: &Orientation) -> Option<Position> {
        let left_border = position.x == 0;
        let top_border = position.y == 0;
        let right_border = position.x + 1 == self.size;
        let bottom_border = position.y + 1 == self.size;

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
}

impl Display for HeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.size).for_each(|y| {
            (0..self.size).for_each(|x| {
                f.write_fmt(format_args!("{}", self.get_by_coordinates(x, y)))
                    .unwrap();
            });
            f.write_char('\n').unwrap();
        });

        Ok(())
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
    path: Path,
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
struct Path {
    positions: Vec<(Position, Orientation)>,
}

impl Path {
    fn with(&self, position: &Position, orientation: &Orientation) -> Self {
        let mut result = self.positions.clone();
        result.push((position.clone(), orientation.clone()));

        Self { positions: result }
    }

    fn start(orientation: &Orientation) -> Self {
        Self {
            // positions: vec![(Position { x: 0, y: 0 }, orientation.clone())],
            positions: vec![],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Turn {
    Straight,
    Left,
    Right,
}
