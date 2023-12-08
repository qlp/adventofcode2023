use std::collections::HashMap;

use crate::Move::{LEFT, RIGHT};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");
const EXAMPLE_3: &str = include_str!("example-3.txt");

fn main() {
    print_answer("one (example 1)", &one(EXAMPLE_1), "2");
    print_answer("one (example 2)", &one(EXAMPLE_2), "6");
    print_answer("one", &one(INPUT), "21251");
    print_answer("two (example)", &two(EXAMPLE_3), "6");
    print_answer("two", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = parse(input);
    let moves = world.moves.clone();

    let mut steps = 0usize;
    let mut current = String::from("AAA");

    while current != "ZZZ" {
        let next_move_index = steps % moves.len();
        let next_move = moves.get(next_move_index).expect("expect node to exist");
        current = world.go(current, next_move);
        steps += 1;
    }

    steps.to_string()
}

fn two(input: &str) -> String {
    let world = parse(input);
    let moves = world.moves.clone();

    let mut steps = 0usize;
    let mut current: Vec<String> = world
        .nodes
        .values()
        .map(|v| v.name.clone())
        .filter(|n| n.ends_with('A'))
        .collect();

    while !current.iter().all(|c| c.ends_with('Z')) {
        let next_move_index = steps % moves.len();
        let next_move = moves.get(next_move_index).expect("expect node to exist");
        current = current
            .iter()
            .map(|c| world.go(c.clone(), next_move))
            .collect();
        steps += 1;
    }

    steps.to_string()
}

fn parse(input: &str) -> World {
    let (moves, nodes) = input.split_once("\n\n").expect("double newline");

    World {
        moves: moves
            .chars()
            .map(|c| match c {
                'L' => LEFT,
                'R' => RIGHT,
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
    }
}

#[derive(Debug, Clone)]
struct World {
    moves: Vec<Move>,
    nodes: HashMap<String, Node>,
}

impl World {
    fn go(&self, from: String, to: &Move) -> String {
        let destination = self.nodes.get(&*from).expect("expect from");

        match to {
            LEFT => destination.left.clone(),
            RIGHT => destination.right.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}
