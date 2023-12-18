use crate::Direction::{Down, Left, Right, Up};
use bit_set::BitSet;
use hex;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Write};
use std::i32;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "62");
    print_answer("one", &one(INPUT), "37044 is too low");
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
    let plan = DigPlan::parse(input);

    println!("{}", plan);

    plan.as_bit_set().len().to_string()
}

fn two(input: &str) -> String {
    String::new()
}

struct DigPlan {
    steps: Vec<Step>,
}

impl DigPlan {
    fn parse(input: &str) -> Self {
        Self {
            steps: input.lines().map(Step::parse).collect(),
        }
    }

    fn points(&self) -> Vec<Point> {
        let mut result = vec![Point { x: 0, y: 0 }];

        self.steps.iter().for_each(|step| {
            result.push(result.last().expect("point").dig(step));
        });

        result
    }

    fn size(&self) -> Size {
        self.points().iter().fold(
            (Size {
                min: Point::origin(),
                max: Point::origin(),
            }),
            |acc, p| Size {
                min: Point {
                    x: acc.min.x.min(p.x),
                    y: acc.min.y.min(p.y),
                },
                max: Point {
                    x: acc.max.x.max(p.x),
                    y: acc.max.y.max(p.y),
                },
            },
        )
    }

    fn as_bit_set(&self) -> BitSet {
        let size = self.size();

        let mut display = BitSet::with_capacity(size.width() * size.height());

        self.points().windows(2).for_each(|points| {
            let from = &points[0];
            let to = &points[1];

            (from.y.min(to.y)..=from.y.max(to.y)).for_each(|y| {
                (from.x.min(to.x)..=from.x.max(to.x)).for_each(|x| {
                    let index =
                        ((y - size.min.y) as usize) * size.width() + (x - size.min.x) as usize;

                    display.insert(index);
                })
            })
        });

        let mut fill = vec![Point::origin().dig(&Step {
            direction: self.steps[0].direction.clone(),
            count: self.steps[0].count + 1,
            color: String::new(),
        })];

        while !fill.is_empty() {
            let mut next_fill: HashSet<Point> = HashSet::new();

            fill.iter().for_each(|point| {
                let index = ((point.y - size.min.y) as usize) * size.width()
                    + (point.x - size.min.x) as usize;

                display.insert(index);

                let empty_points_around: HashSet<Point> = point
                    .around()
                    .into_iter()
                    .filter(|point| size.contains(point))
                    .filter(|candidate| !display.contains(candidate.to_index(&size)))
                    .collect();

                next_fill.extend(empty_points_around);
            });

            fill = Vec::from_iter(next_fill);
        }

        display
    }
}

impl Display for DigPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size = self.size();

        let mut display = self.as_bit_set();

        (0..size.height()).for_each(|y| {
            (0..size.width()).for_each(|x| {
                let index = y * size.width() + x;

                f.write_char(match display.contains(index) {
                    true => match x as i32 == -size.min.x && y as i32 == -size.min.y {
                        true => 'S',
                        false => '#',
                    },
                    false => '.',
                })
                .unwrap();
            });
            f.write_char('\n').unwrap();
        });

        Ok(())
    }
}

struct Step {
    direction: Direction,
    count: i32,
    color: String,
}

impl Step {
    fn parse(input: &str) -> Self {
        let segments: Vec<&str> = input.split(' ').collect();

        let x = &segments[2].to_string()[1..segments[2].len() - 2];
        Self {
            direction: Direction::parse(segments[0]),
            count: segments[1].parse().expect("number"),
            color: segments[2][2..segments[2].len() - 1].to_string(),
        }
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} (#{})",
            self.direction, self.count, self.color,
        ))
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(input: &str) -> Self {
        match input {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("unexpected direction"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Up => 'U',
            Down => 'D',
            Left => 'L',
            Right => 'R',
        })
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn dig(&self, step: &Step) -> Point {
        match step.direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y - step.count,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + step.count,
            },
            Direction::Left => Point {
                x: self.x - step.count,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + step.count,
                y: self.y,
            },
        }
    }

    fn around(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    fn to_index(&self, size: &Size) -> usize {
        ((self.y - size.min.y) as usize) * size.width() + (self.x - size.min.x) as usize
    }
}

struct Size {
    min: Point,
    max: Point,
}

impl Size {
    fn width(&self) -> usize {
        (self.max.x - self.min.x + 1) as usize
    }

    fn height(&self) -> usize {
        (self.max.y - self.min.y + 1) as usize
    }

    fn contains(&self, point: &Point) -> bool {
        (self.min.x..=self.max.x).contains(&point.x) && (self.min.y..=self.max.y).contains(&point.y)
    }
}
