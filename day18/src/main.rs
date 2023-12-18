use std::i64;
use std::ops::RangeInclusive;

use crate::Direction::{Down, Left, Right, Up};
use crate::Part::{One, Two};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "62");
    print_answer("one", &one(INPUT), "45159");
    print_answer("two (example)", &two(EXAMPLE), "952408144115");
    print_answer("two", &two(INPUT), "134549294799713");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

enum Part {
    One,
    Two,
}

fn one(input: &str) -> String {
    DigPlan::parse(input, &One).filled().to_string()
}

fn two(input: &str) -> String {
    DigPlan::parse(input, &Two).filled().to_string()
}

struct DigPlan {
    points: Vec<Point>,
}

impl DigPlan {
    fn parse(input: &str, part: &Part) -> Self {
        let mut points = vec![Point::origin()];

        input
            .lines()
            .map(|line| Step::parse(line, part))
            .for_each(|step| {
                points.push(points.last().expect("point").dig(&step));
            });

        Self { points }
    }

    fn filled(&self) -> u64 {
        let mut ys: Vec<i64> = self.points.iter().map(|p| p.y).collect();
        ys.sort();
        ys.dedup();

        let mut ranges = self.ranges_on_y(ys[0]);
        let mut result = ranges
            .iter()
            .map(|r| (r.end() - r.start() + 1) as u64)
            .sum();

        ys.windows(2).for_each(|window| {
            let previous_y = window[0];
            let current_y = window[1];

            let xs_on_previous_y: Vec<i64> = ranges
                .iter()
                .flat_map(|range| vec![*range.start(), *range.end()])
                .collect();
            let xs_on_current_y = self.xs_on_y(current_y);

            let mut all_xs: Vec<i64> = Vec::new();
            all_xs.extend(&xs_on_previous_y);
            all_xs.extend(&xs_on_current_y);
            all_xs.sort();
            all_xs.dedup();

            let split_previous: Vec<RangeInclusive<i64>> = ranges
                .iter()
                .flat_map(|range| {
                    all_xs
                        .iter()
                        .filter(|x| range.contains(x))
                        .collect::<Vec<&i64>>()
                        .windows(2)
                        .map(|x| *x[0]..=*x[1])
                        .collect::<Vec<RangeInclusive<i64>>>()
                })
                .collect();

            let split_current: Vec<RangeInclusive<i64>> = self
                .ranges_on_y(current_y)
                .iter()
                .flat_map(|range| {
                    all_xs
                        .iter()
                        .filter(|x| range.contains(x))
                        .collect::<Vec<&i64>>()
                        .windows(2)
                        .map(|x| *x[0]..=*x[1])
                        .collect::<Vec<RangeInclusive<i64>>>()
                })
                .collect();

            let mut new_ranges: Vec<RangeInclusive<i64>> = Vec::new();
            new_ranges.extend(split_previous.clone());
            new_ranges.extend(split_current.clone());
            new_ranges.sort_by_key(|r| r.start().clone());
            new_ranges.dedup();

            let diff_y = (current_y - previous_y) as u64;
            let surface_above = (diff_y - 1)
                * ranges
                    .iter()
                    .map(|r| (r.end() - r.start() + 1) as u64)
                    .sum::<u64>();

            let surface_at_row = Self::merge(&new_ranges)
                .iter()
                .map(|r| (r.end() - r.start() + 1) as u64)
                .sum::<u64>();

            let surface = surface_above + surface_at_row;
            result += surface;

            let mut split_ranges = new_ranges
                .iter()
                .filter(|candidate| {
                    match (
                        split_previous.contains(candidate),
                        split_current.contains(candidate),
                    ) {
                        (true, true) => false,
                        (false, true) => true,
                        (true, false) => true,
                        (false, false) => panic!("unexpected"),
                    }
                })
                .cloned()
                .collect::<Vec<RangeInclusive<i64>>>();
            split_ranges.sort_by_key(|r| r.start().clone());

            ranges = Self::merge(&split_ranges);
        });

        result
    }

    fn merge(split_ranges: &Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
        let mut merged_ranges: Vec<RangeInclusive<i64>> = Vec::new();
        split_ranges
            .iter()
            .for_each(|range| match merged_ranges.is_empty() {
                true => {
                    merged_ranges.push(range.clone());
                }
                false => {
                    let len = merged_ranges.len();
                    let last = &merged_ranges[len - 1];
                    match last.end() == range.start() {
                        true => {
                            merged_ranges[len - 1] = *last.start()..=*range.end();
                        }
                        false => merged_ranges.push(range.clone()),
                    }
                }
            });
        merged_ranges
    }

    fn xs_on_y(&self, on_y: i64) -> Vec<i64> {
        let mut xs_on_y: Vec<i64> = self
            .points
            .iter()
            .filter(|p| p.y == on_y)
            .map(|p| p.x)
            .collect();
        xs_on_y.sort();
        xs_on_y.dedup();

        xs_on_y
    }

    fn ranges_on_y(&self, on_y: i64) -> Vec<RangeInclusive<i64>> {
        self.xs_on_y(on_y)
            .chunks(2)
            .map(|xs| (xs[0]..=xs[1]))
            .collect::<Vec<RangeInclusive<i64>>>()
    }
}

struct Step {
    direction: Direction,
    count: i64,
}

impl Step {
    fn parse(input: &str, part: &Part) -> Self {
        let segments: Vec<&str> = input.split(' ').collect();

        match part {
            One => Self {
                direction: Direction::parse(segments[0]),
                count: segments[1].parse().expect("number"),
            },
            Two => Self {
                direction: match segments[2].chars().nth(7) {
                    Some('0') => Right,
                    Some('1') => Down,
                    Some('2') => Left,
                    Some('3') => Up,
                    _ => panic!("unexpected"),
                },
                count: hex::decode(format!("0{}", &segments[2][2..=6]))
                    .expect("string")
                    .iter()
                    .fold(0i64, |acc, b| acc << 8 | *b as i64),
            },
        }
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

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn dig(&self, step: &Step) -> Point {
        match step.direction {
            Up => Point {
                x: self.x,
                y: self.y - step.count,
            },
            Down => Point {
                x: self.x,
                y: self.y + step.count,
            },
            Left => Point {
                x: self.x - step.count,
                y: self.y,
            },
            Right => Point {
                x: self.x + step.count,
                y: self.y,
            },
        }
    }
}
