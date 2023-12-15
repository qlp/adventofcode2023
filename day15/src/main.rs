use crate::Operation::{ADD, SUBTRACT};
use array_init::array_init;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "1320");
    print_answer("one", &one(INPUT), "506269");
    print_answer("two (example)", &two(EXAMPLE), "145");
    print_answer("two", &two(INPUT), "264021");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    Operations::parse(input)
        .operations
        .iter()
        .map(|o| o.aoc_hash() as u32)
        .sum::<u32>()
        .to_string()
}

fn two(input: &str) -> String {
    let mut boxes = Boxes::new();

    let operations = Operations::parse(input);

    operations.operations.iter().for_each(|operation| {
        boxes.apply(&operation);
    });

    boxes.focussing_power().to_string()
}

struct Boxes<'a> {
    boxes: [Box<'a>; 256],
}

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self {
            boxes: array_init(|_| Box::new()),
        }
    }

    fn apply(&mut self, operation: &'a Operation) {
        match operation {
            SUBTRACT(label) => self.subtract(label),
            ADD(label, length) => self.add(label, *length),
        }
    }

    fn subtract(&mut self, label: &str) {
        self.boxes
            .iter_mut()
            .for_each(|b| b.lenses.retain(|l| l.label.ne(label)))
    }

    fn add(&mut self, label: &'a str, length: u8) {
        let b = self.boxes.get_mut(label.aoc_hash() as usize).unwrap();
        let lens = Lens { label, length };

        match b
            .lenses
            .iter()
            .enumerate()
            .position(|(_, l)| l.label.eq(label))
        {
            None => b.lenses.push(lens),
            Some(index) => b.lenses[index] = lens,
        }
    }

    fn focussing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(index, b)| b.focussing_power() * (index as u32 + 1))
            .sum()
    }
}

impl<'a> Display for Boxes<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.boxes.iter().enumerate().for_each(|(box_index, b)| {
            if !b.lenses.is_empty() {
                f.write_fmt(format_args!("Box {}: ", box_index)).unwrap();

                b.lenses.iter().enumerate().for_each(|(lens_index, l)| {
                    if lens_index != 0 {
                        f.write_char(' ').unwrap();
                    }
                    f.write_fmt(format_args!("{}", l)).unwrap();
                });
                f.write_char('\n').unwrap()
            }
        });

        Ok(())
    }
}

struct Operations<'a> {
    operations: Vec<Operation<'a>>,
}

impl<'a> Operations<'a> {
    fn parse(str: &'a str) -> Self {
        Operations {
            operations: str.split(',').map(Operation::parse).collect(),
        }
    }
}

impl<'a> Display for Operations<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.operations.iter().enumerate().for_each(|(index, o)| {
            if index != 0 {
                f.write_char(',').unwrap();
            }

            o.fmt(f).unwrap();
        });

        Ok(())
    }
}

enum Operation<'a> {
    SUBTRACT(&'a str),
    ADD(&'a str, u8),
}

impl<'a> Operation<'a> {
    fn parse(str: &'a str) -> Self {
        match str.ends_with('-') {
            true => SUBTRACT(&str[0..str.len() - 1]),
            false => str
                .find('=')
                .map(|index| {
                    ADD(
                        &str[0..index],
                        str[index + 1..].parse::<u8>().expect("number"),
                    )
                })
                .expect("equals"),
        }
    }
}

impl<'a> Display for Operation<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SUBTRACT(label) => f.write_fmt(format_args!("{label}-")),
            ADD(label, length) => f.write_fmt(format_args!("{label}={length}")),
        }
    }
}

impl<'a> AocHash for Operation<'a> {
    fn aoc_hash(&self) -> u8 {
        self.to_string().aoc_hash()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Box<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> Box<'a> {
    fn new() -> Self {
        Box { lenses: vec![] }
    }

    fn focussing_power(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(index, lens)| (index + 1) as u32 * lens.length as u32)
            .sum()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Lens<'a> {
    label: &'a str,
    length: u8,
}

impl<'a> Display for Lens<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.label, self.length))
    }
}

trait AocHash {
    fn aoc_hash(&self) -> u8;
}

impl AocHash for String {
    fn aoc_hash(&self) -> u8 {
        self.chars()
            .fold(0u32, |acc, c| (acc + c as u8 as u32) * 17 % 256) as u8
    }
}

impl AocHash for &str {
    fn aoc_hash(&self) -> u8 {
        self.chars()
            .fold(0u32, |acc, c| (acc + c as u8 as u32) * 17 % 256) as u8
    }
}
