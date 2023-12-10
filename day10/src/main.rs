use crate::Connection::*;
use crate::Direction::*;
use crate::Square::{Ground, Pipe, Start};
use std::fmt::{Debug, Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example_1.txt");
const EXAMPLE_2: &str = include_str!("example_2.txt");

fn main() {
    print_answer("one (example 1)", &one(EXAMPLE_1), "4");
    print_answer("one (example 2)", &one(EXAMPLE_2), "8");
    print_answer("one", &one(INPUT), "");
    // print_answer("two (example)", &two(EXAMPLE), "2");
    // print_answer("two", &two(INPUT), "1053");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = World::parse(input);

    println!("{world}");
    println!("{:?}", &world.start());

    let (start, direction) = &world.start();

    let mut current_coordinate: Coordinate = start.clone();
    let mut current_direction: Direction = direction.clone();
    let mut steps = 0;

    while true {
        steps += 1;

        current_coordinate = current_direction.next(&current_coordinate);
        let next_square = world.square_at(&current_coordinate);

        current_direction = match next_square {
            Pipe(c) => c.next_direction(&current_direction).clone(),
            Ground => panic!("expecting pipe or start"),
            Start => break,
        }
    }

    (steps / 2).to_string()
}

fn two(input: &str) -> String {
    String::new()
}

#[derive(Clone)]
struct World {
    squares: Vec<Vec<Square>>,
}

impl World {
    fn start(&self) -> (Coordinate, Direction) {
        let coordinate = self
            .squares
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find_map(|(x, square)| match square {
                        Start => Some(x),
                        _ => None,
                    })
                    .map(|x| Coordinate {
                        x: x as i32,
                        y: y as i32,
                    })
            })
            .expect("a starting point");

        let direction = [North, South, East, West]
            .iter()
            .find(|d| {
                let square = self.square_at(&d.next(&coordinate));
                match square {
                    Pipe(p) => p.directions().contains(&d.opposite()),
                    Ground => false,
                    Start => false,
                }
            })
            .expect("at least one");

        (coordinate, direction.clone())
    }

    fn square_at(&self, coordinate: &Coordinate) -> &Square {
        match (coordinate.x, coordinate.y) {
            (..=-1, _) => &Ground,
            (_, ..=-1) => &Ground,
            (x, y) => &self.squares[y as usize][x as usize],
        }
    }

    fn parse(input: &str) -> Self {
        Self {
            squares: input
                .lines()
                .map(|l| l.chars().map(|c| Square::parse(c)).collect())
                .collect(),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.squares.iter().for_each(|line| {
            line.iter().for_each(|s| s.fmt(f).expect("ok"));
            f.write_char('\n').expect("ok")
        });

        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Square {
    Pipe(Connection),
    Ground,
    Start,
}

impl Square {
    fn parse(input: char) -> Self {
        match input {
            '|' => Pipe(NS),
            '-' => Pipe(EW),
            'L' => Pipe(NE),
            'J' => Pipe(NW),
            '7' => Pipe(SW),
            'F' => Pipe(SE),
            '.' => Ground,
            'S' => Start,
            _ => panic!("unexpected character"),
        }
    }

    fn directions(&self) -> Vec<Direction> {
        match self {
            Pipe(c) => c.directions().to_vec(),
            Start => vec![North, South, East, West],
            Ground => vec![],
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe(p) => p.fmt(f),
            Ground => f.write_char(' '),
            Start => f.write_char('S'),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn next(&self, from: &Coordinate) -> Coordinate {
        match self {
            South => Coordinate {
                x: from.x,
                y: from.y + 1,
            },
            North => Coordinate {
                x: from.x,
                y: from.y - 1,
            },
            East => Coordinate {
                x: from.x + 1,
                y: from.y,
            },
            West => Coordinate {
                x: from.x - 1,
                y: from.y,
            },
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            South => North,
            North => South,
            East => West,
            West => East,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Connection {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
}

impl Connection {
    fn directions(&self) -> [Direction; 2] {
        match self {
            NS => [North, South],
            EW => [East, West],
            NE => [North, East],
            NW => [North, West],
            SW => [South, West],
            SE => [South, East],
        }
    }
}

impl Connection {
    fn next_direction(&self, from: &Direction) -> Direction {
        let directions = self.directions();

        let one = directions[0].clone();
        let two = directions[1].clone();

        match one == from.opposite() {
            true => two,
            false => one,
        }
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            NS => '│',
            EW => '─',
            NE => '└',
            NW => '┘',
            SW => '┐',
            SE => '┌',
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}
