use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};

use State::On;

use crate::Kind::{Broadcaster, Conjunction, FlipFlop};
use crate::Pulse::{High, Low};
use crate::State::Off;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_1: &str = include_str!("example-1.txt");
const EXAMPLE_2: &str = include_str!("example-2.txt");

fn main() {
    // print_answer("one (example 1)", &one(EXAMPLE_1), "32000000");
    // print_answer("one (example 2)", &one(EXAMPLE_2), "11687500");
    // print_answer("one", &one(INPUT), "681194780");
    print_answer("two", &two(INPUT), "238593356738827");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    World::parse(input).push(1000, |_| false).0.to_string()
}

fn two(input: &str) -> String {
    let other_world = World::parse(input);

    let rx_sender = other_world
        .machines
        .iter()
        .find(|machine| machine.outgoing.contains(&"rx".to_string()))
        .expect("rx to exist");

    let rx_inputs: Vec<&Machine> = other_world
        .machines
        .iter()
        .filter(|machine| machine.outgoing.contains(&rx_sender.name()))
        .collect();

    println!("{}", other_world);
    println!("rx: {rx_sender}");

    rx_inputs.iter().for_each(|input| {
        println!("rx sender input: {input}");
    });

    let (_, vg_presses) = World::parse(input).push(u64::MAX, |world| {
        world
            .machines
            .iter()
            .find(|machine| machine.name() == *"vg")
            .expect("vg to exist")
            .current
            .iter()
            .any(|pulse| *pulse == High)
    });
    println!("presses of 'vg': {vg_presses}");

    let (_, kp_presses) = World::parse(input).push(u64::MAX, |world| {
        world
            .machines
            .iter()
            .find(|machine| machine.name() == *"kp")
            .expect("kp to exist")
            .current
            .iter()
            .any(|pulse| *pulse == High)
    });
    println!("presses of 'kp': {kp_presses}");

    let (_, gc_presses) = World::parse(input).push(u64::MAX, |world| {
        world
            .machines
            .iter()
            .find(|machine| machine.name() == *"gc")
            .expect("gc to exist")
            .current
            .iter()
            .any(|pulse| *pulse == High)
    });
    println!("presses of 'gc': {gc_presses}");

    let (_, tx_presses) = World::parse(input).push(u64::MAX, |world| {
        world
            .machines
            .iter()
            .find(|machine| machine.name() == *"tx")
            .expect("tx to exist")
            .current
            .iter()
            .any(|pulse| *pulse == High)
    });

    println!("presses of 'tx': {tx_presses}");

    let result = vec![vg_presses, kp_presses, gc_presses, tx_presses]
        .into_iter()
        .reduce(lcm)
        .expect("value");

    result.to_string()
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

struct World {
    machines: Vec<Machine>,
}

impl World {
    fn parse(input: &str) -> Self {
        World {
            machines: input.lines().map(Machine::parse).collect(),
        }
    }

    fn push(&mut self, times: u64, or: fn(&World) -> bool) -> (u64, u64) {
        let (mut total_low, mut total_high) = (0u64, 0u64);
        let mut time = 0;

        while time < times && !or(self) {
            time += 1;
            // println!("time: {}", time);

            let broadcaster = self
                .machines
                .iter_mut()
                .find(|machine| machine.kind == Broadcaster)
                .expect("broadcaster");

            broadcaster.current = vec![Low];

            let (mut added_low, mut added_high) = (1u64, 0u64);

            while added_low != 0 || added_high != 0 {
                total_low += added_low;
                total_high += added_high;

                let pulses: Vec<(MachineName, MachineName, Vec<Pulse>)> = self
                    .machines
                    .iter()
                    .flat_map(|machine| {
                        machine
                            .outgoing
                            .iter()
                            .map(|name| (machine.name(), name.clone(), machine.current.clone()))
                    })
                    .collect();

                // pulses.iter().for_each(|(from, to, pulse)| match pulse {
                //     None => {}
                //     Some(pulse) => {
                //         println!("{} -{}-> {}", from, pulse, to);
                //     }
                // });

                self.machines.iter_mut().for_each(|machine| {
                    machine.process(
                        pulses
                            .iter()
                            .filter(|(_, to, _)| *to == machine.name())
                            .map(|(from, _, pulse)| (from.clone(), pulse.clone()))
                            .collect(),
                    )
                });

                self.machines
                    .iter_mut()
                    .for_each(|machine| machine.execute());

                // self.machines.iter().for_each(|machine| {
                //     println!("{}", machine);
                // });

                (added_low, added_high) = pulses.iter().fold(
                    (0, 0),
                    |(outer_added_low, outer_added_high), (_, _, inner_pulses)| {
                        let (inner_added_low, inner_added_high) = inner_pulses.iter().fold(
                            (0, 0),
                            |(inner_added_low, inner_added_high), inner_pulse| match inner_pulse {
                                Low => (inner_added_low + 1, inner_added_high),
                                High => (inner_added_low, inner_added_high + 1),
                            },
                        );

                        (
                            outer_added_low + inner_added_low,
                            outer_added_high + inner_added_high,
                        )
                    },
                );

                if or(self) {
                    break;
                }
            }
        }

        (total_low * total_high, time)
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.machines
            .iter()
            .for_each(|machine| f.write_fmt(format_args!("{}\n", machine)).unwrap());

        Ok(())
    }
}

type MachineName = String;

struct Machine {
    kind: Kind,
    next: Vec<Pulse>,
    current: Vec<Pulse>,
    outgoing: Vec<String>,
}

impl Machine {
    fn parse(input: &str) -> Self {
        let (kind, outgoing) = input.split_once(" -> ").expect("separator");

        Self {
            kind: Kind::parse(kind),
            outgoing: outgoing.split(", ").map(|s| s.to_string()).collect(),
            next: vec![],
            current: vec![],
        }
    }

    fn name(&self) -> String {
        match &self.kind {
            Broadcaster => "broadcaster".to_string(),
            FlipFlop(name, _) => name.to_string(),
            Conjunction(name, _) => name.to_string(),
        }
    }

    fn process(&mut self, pulses_by_machine: HashMap<MachineName, Vec<Pulse>>) {
        self.next = vec![];

        match self.kind.clone() {
            Broadcaster => {}
            FlipFlop(name, state) => {
                pulses_by_machine.iter().for_each(|(machine_name, pulses)| {
                    pulses.iter().for_each(|pulse| match pulse {
                        High => {}
                        Low => {
                            let new_state = state.flip();
                            self.kind = FlipFlop(name.clone(), new_state);

                            self.next.push(match new_state {
                                On => High,
                                Off => Low,
                            });
                        }
                    })
                });
            }
            Conjunction(name, state) => {
                let mut new_state = state.clone();
                if new_state.is_empty() {
                    pulses_by_machine.keys().for_each(|key| {
                        new_state.insert(key.clone(), Low);
                    });
                }

                pulses_by_machine.iter().for_each(|(machine_name, pulses)| {
                    pulses.iter().for_each(|pulse| {
                        new_state.insert(machine_name.clone(), *pulse);

                        self.kind = Conjunction(name.clone(), new_state.clone());

                        let all_high = new_state.iter().all(|(_, pulse)| pulse == &High);

                        self.next.push(match all_high {
                            true => Low,
                            false => High,
                        });
                    });
                });
            }
        };
    }

    fn execute(&mut self) {
        self.current = self.next.clone();
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} -> {}",
            self.kind,
            self.outgoing.join(", ")
        ))
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Kind {
    Broadcaster,
    FlipFlop(String, State),
    Conjunction(String, HashMap<MachineName, Pulse>),
}

impl Kind {
    fn parse(input: &str) -> Self {
        match input.chars().next() {
            Some('%') => FlipFlop(input[1..input.len()].to_string(), Off),
            Some('&') => Conjunction(input[1..input.len()].to_string(), HashMap::new()),
            Some('b') => Broadcaster,
            _ => panic!("unexpected kind"),
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Broadcaster => f.write_str("broadcaster"),
            FlipFlop(name, _) => f.write_fmt(format_args!("%{}", name)),
            Conjunction(name, _) => f.write_fmt(format_args!("&{}", name)),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    On,
    Off,
}

impl State {
    fn flip(&self) -> Self {
        match self {
            On => Off,
            Off => On,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            High => "high",
            Low => "low",
        })
    }
}
