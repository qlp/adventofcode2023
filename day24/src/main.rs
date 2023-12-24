use crate::Intersection::{Intersects, Overlaps, Parallel};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Pointer, Write};
use std::num::ParseIntError;
use std::ops::{Range, RangeInclusive, RangeToInclusive};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE, 7..=27), "2");
    print_answer(
        "one",
        &one(INPUT, 200000000000000..=400000000000000),
        "12015",
    );
    // print_answer("two (example)", &two(EXAMPLE), "");
    // print_answer("two", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str, range: RangeInclusive<i64>) -> String {
    Storm::from_str(input)
        .expect("storm")
        .collisions_in_x_y(range)
        .len()
        .to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct Storm {
    stones: Vec<Stone>,
}

impl Storm {
    fn collisions_in_x_y(&self, range: RangeInclusive<i64>) -> Vec<XyIntersection> {
        let result = (0..self.stones.len() - 1)
            .flat_map(|left_index| {
                (left_index + 1..self.stones.len())
                    .filter_map(|right_index| {
                        let left = &self.stones[left_index];
                        let right = &self.stones[right_index];

                        match left.collides_x_y(&right) {
                            Overlaps => Some(XyIntersection {
                                points: [left, right],
                            }), // TODO: check for lines outside the boix
                            Parallel => None,
                            Intersects(x) => match range.contains(&(x as i64)) {
                                true => match range
                                    .contains(&(LinearEquation::from(left).solve(x) as i64))
                                {
                                    true => Some(XyIntersection {
                                        points: [left, right],
                                    }),
                                    false => None,
                                },
                                false => None,
                            },
                        }
                    })
                    .collect::<Vec<XyIntersection>>()
            })
            .collect::<Vec<XyIntersection>>();

        // result.iter().for_each(|collision| {
        //     println!("{collision}");
        // });

        result
    }
}

impl FromStr for Storm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            stones: s
                .lines()
                .map(|line| Stone::from_str(line).expect("line"))
                .collect(),
        })
    }
}

impl Display for Storm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.stones.iter().try_for_each(|stone| {
            stone.fmt(f)?;
            f.write_char('\n')
        })
    }
}

#[derive(Debug)]
struct Stone {
    position: Point,
    velocity: Velocity,
}

impl Stone {
    fn collides_x_y(&self, other: &Stone) -> Intersection {
        let self_equation = LinearEquation::from(self);
        let other_equation = LinearEquation::from(other);

        let intersection = self_equation.intersection(&other_equation);

        match intersection {
            Intersects(x) => match self.x_is_valid(x as i64) && other.x_is_valid(x as i64) {
                true => intersection,
                false => Parallel,
            },
            _ => intersection,
        }
    }

    fn x_is_valid(&self, x: i64) -> bool {
        match self.velocity.x.cmp(&0) {
            Ordering::Less => x < self.position.x,
            Ordering::Equal => x == self.position.x,
            Ordering::Greater => x > self.position.x,
        }
    }
}

#[derive(Debug)]
struct XyIntersection<'a> {
    points: [&'a Stone; 2],
}

impl<'a> Display for XyIntersection<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.points[0], self.points[1]))
    }
}

impl FromStr for Stone {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").expect("separator");

        Ok(Self {
            position: Point::from_str(position).expect("point"),
            velocity: Velocity::from_str(velocity).expect("velocity"),
        })
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} @ {}", self.position, self.velocity))
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let velocities: Result<Vec<i64>, _> =
            s.split(", ").map(|speed| speed.trim().parse()).collect();
        let velocities: Vec<i64> = velocities?;

        Ok(Self {
            x: velocities[0],
            y: velocities[1],
            z: velocities[2],
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}, {}", self.x, self.y, self.z))
    }
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Velocity {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let velocities: Result<Vec<i64>, _> =
            s.split(", ").map(|speed| speed.trim().parse()).collect();
        let velocities: Vec<i64> = velocities?;

        Ok(Self {
            x: velocities[0],
            y: velocities[1],
            z: velocities[2],
        })
    }
}

impl Display for Velocity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}, {}, {}", self.x, self.y, self.z))
    }
}

enum Intersection {
    Overlaps,
    Parallel,
    Intersects(f64),
}

impl Display for Intersection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Overlaps => f.write_str("overlaps"),
            Parallel => f.write_str("parallel"),
            Intersects(x) => f.write_fmt(format_args!("intersects at {}", x)),
        }
    }
}

struct LinearEquation {
    a: f64,
    b: f64,
}

impl LinearEquation {
    fn solve(&self, x: f64) -> f64 {
        self.a * x + self.b
    }

    fn intersection(&self, other: &LinearEquation) -> Intersection {
        match (self.a == other.a, self.b == other.b) {
            (true, true) => Overlaps,
            (true, false) => Parallel,
            (false, _) => Intersects((other.b - self.b) / (self.a - other.a)),
        }
    }
}

impl From<&Stone> for LinearEquation {
    fn from(stone: &Stone) -> Self {
        let a = stone.velocity.y as f64 / stone.velocity.x as f64;
        let b = stone.position.y as f64 - a * stone.position.x as f64;

        Self { a, b }
    }
}

impl Display for LinearEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "y = {}x {}",
            self.a,
            match self.b < 0f64 {
                true => format!("- {}", self.b.abs()),
                false => format!("+ {}", self.b),
            }
        ))
    }
}
