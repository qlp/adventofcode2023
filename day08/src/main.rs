use std::collections::HashMap;

use crate::Move::{Left, Right};
use crate::Part::{One, Two};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");
const EXAMPLE_3: &str = include_str!("example-3.txt");

fn main() {
    print_answer("one (example 1)", &one(EXAMPLE_1), "2");
    print_answer("one (example 2)", &one(EXAMPLE_2), "6");
    print_answer("one", &one(INPUT), "21251");
    print_answer("two (example)", &two(EXAMPLE_3), "6");
    print_answer("two", &two(INPUT), "11678319315857");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    parse(input, One)
        .find(&Position {
            name: "AAA".to_string(),
            steps: 0,
        })
        .steps
        .to_string()
}

fn two(input: &str) -> String {
    let world = parse(input, Two);

    world
        .nodes
        .values()
        .map(|v| v.name.clone())
        .filter(|n| n.ends_with('A'))
        .map(|n| Position { name: n, steps: 0 })
        .map(|p| world.find(&p))
        .map(|p| p.steps)
        .reduce(lcm)
        .expect("at least on")
        .to_string()
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn parse(input: &str, part: Part) -> World {
    let (moves, nodes) = input.split_once("\n\n").expect("double newline");

    World {
        moves: moves
            .chars()
            .map(|c| match c {
                'L' => Left,
                'R' => Right,
                _ => panic!("expect L or R"),
            })
            .collect(),
        nodes: nodes
            .lines()
            .map(|l| {
                let name = String::from(&l[0..3]);
                let left = String::from(&l[7..10]);
                let right = String::from(&l[12..15]);

                (name.clone(), Node { name, left, right })
            })
            .collect(),
        part,
    }
}

#[derive(Debug, Clone)]
struct World {
    moves: Vec<Move>,
    nodes: HashMap<String, Node>,
    part: Part,
}

impl World {
    fn go(&self, from: &String, to: &Move) -> String {
        let destination = self.nodes.get(from).expect("expect from");

        match to {
            Left => destination.left.clone(),
            Right => destination.right.clone(),
        }
    }

    fn find(&self, from: &Position) -> Position {
        let mut steps = from.steps;
        let mut name = from.name.clone();
        let number_of_moves = self.moves.len() as u64;

        while steps == from.steps || !self.condition(&name) {
            let next_move_index = steps % number_of_moves;
            let next_move = self
                .moves
                .get(next_move_index as usize)
                .expect("expect node to exist");
            name = self.go(&name, next_move);
            steps += 1;
        }

        Position { name, steps }
    }

    fn condition(&self, name: &String) -> bool {
        match self.part {
            One => name == "ZZZ",
            Two => name.ends_with('Z'),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Part {
    One,
    Two,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    name: String,
    steps: u64,
}
