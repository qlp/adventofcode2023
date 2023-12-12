use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::ops::BitXor;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "21");
    print_answer("one", &one(INPUT), "7344");
    print_answer("two (example)", &two(EXAMPLE), "525152");
    print_answer("two (example)", &two(INPUT), "");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = World::parse(input, 1);

    world.records.iter().for_each(|record| {
        // let record = world.records.iter().nth(0).expect("it");
        let mut cache: HashMap<CacheKey, u64> = HashMap::new();
        let candidates = record.candidates(0, 0, 0, &mut cache);

        println!("{record} {candidates}");
    });

    world
        .records
        .iter()
        .map(|record| {
            let mut cache: HashMap<CacheKey, u64> = HashMap::new();
            record.candidates(0, 0, 0, &mut cache)
        })
        .sum::<u64>()
        .to_string()
}

fn two(input: &str) -> String {
    let world = World::parse(input, 5);

    world.records.iter().for_each(|record| {
        let mut cache: HashMap<CacheKey, u64> = HashMap::new();
        // let record = world.records.iter().nth(0).expect("it");
        let candidates = record.candidates(0, 0, 0, &mut cache);

        println!("{record} {candidates}");
    });

    world
        .records
        .iter()
        .map(|record| {
            let mut cache: HashMap<CacheKey, u64> = HashMap::new();
            record.candidates(0, 0, 0, &mut cache)
        })
        .sum::<u64>()
        .to_string()
}

struct World {
    records: Vec<Record>,
}

impl World {
    fn parse(input: &str, copies: u32) -> Self {
        Self {
            records: input.lines().map(|l| Record::parse(l, copies)).collect(),
        }
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.records.iter().for_each(|r| {
            r.fmt(f);
            f.write_char('\n');
        });

        Ok(())
    }
}

struct Record {
    length: u32,
    has_spring: u128,
    has_no_spring: u128,
    groups: Vec<u32>,
}

impl Record {
    fn parse(input: &str, copies: u32) -> Self {
        let (springs, groups) = input.split_once(' ').expect("space");

        let springs = (0..copies)
            .map(|_| springs)
            .collect::<Vec<&str>>()
            .join("?");
        let groups = (0..copies).map(|_| groups).collect::<Vec<&str>>().join(",");

        let length = springs.len() as u32;
        let mut has_spring = 0u128;
        let mut has_no_spring = 0u128;
        (0..length).for_each(|i| {
            let bit = 2u128.pow(length - i - 1);
            match springs.chars().nth(i as usize).expect("char at index") {
                '#' => has_spring |= bit,
                '.' => has_no_spring |= bit,
                '?' => {}
                _ => panic!("unexpect char"),
            };
        });

        let groups: Vec<u32> = groups
            .split(',')
            .map(|group| group.parse().expect("number"))
            .collect();

        Self {
            length,
            has_spring,
            has_no_spring,
            groups,
        }
    }

    fn space(&self) -> u32 {
        self.length - self.groups.iter().sum::<u32>() + 1 - self.groups.len() as u32
    }

    fn candidates(
        &self,
        acc: u128,
        from: u32,
        group_index: usize,
        cache: &mut HashMap<CacheKey, u64>,
    ) -> u64 {
        let group_size = self.groups[group_index];

        let mut result = 0;

        let mut passed_spring = false;
        let mut from = from;

        while !passed_spring && self.length >= from && (self.length - from) >= group_size {
            let mask = (2u128.pow(group_size) - 1) << (self.length - group_size - from);
            let cancidate = acc | mask;

            let final_group = group_index == self.groups.len() - 1;
            let has_remaining_positions = self.length - from - group_size > 0;
            let no_negative_springs = self.has_no_spring & mask == 0;
            let possible_no_spring_after = !has_remaining_positions
                || 1u128 << (self.length - group_size - from - 1) & self.has_spring == 0;
            let possible_no_tail = !final_group
                || !has_remaining_positions
                || (2u128.pow(self.length - from - group_size) - 1) & self.has_spring == 0;

            if final_group {
                // println!("check: {:032b} {no_negative_springs} {possible_no_spring_after} {possible_no_tail}", cancidate);
            }
            // println!("{:07b}", mask);
            // println!("{:07b}", self.has_spring);
            // println!("{:07b}", self.has_no_spring);
            // println!(
            //     "{:07b} {no_negative_springs} {possible_no_spring_after} {possible_no_tail} {final_group} {group_index}",
            //     cancidate
            // );

            if no_negative_springs && possible_no_spring_after && possible_no_tail {
                result += match final_group {
                    true => {
                        let ok_springs = cancidate | self.has_spring == self.has_spring;
                        let ok_no_springs = !cancidate | self.has_no_spring == self.has_no_spring;

                        // println!("found: {:032b} {ok_springs} {ok_no_springs}", cancidate);

                        1u64
                    }
                    false => {
                        let new_acc = acc | mask;
                        let new_from = from + group_size + 1;
                        let new_group_index = group_index + 1;

                        let cache_key = CacheKey {
                            from: new_from,
                            group_index: new_group_index,
                        };

                        let result = cache.get(&cache_key);

                        match result {
                            None => {
                                let value =
                                    self.candidates(new_acc, new_from, new_group_index, cache);

                                cache.insert(cache_key, value);

                                value
                            }
                            Some(value) => *value,
                        }
                    }
                }
            }

            from += 1;

            passed_spring = 2u128.pow(self.length - from) & self.has_spring != 0
        }

        result
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        (0..self.length).rev().for_each(|i| {
            let bit = 2u128.pow(i);
            let has_spring = self.has_spring & bit > 0;
            let has_no_spring = self.has_no_spring & bit > 0;
            match (has_spring, has_no_spring) {
                (false, false) => f.write_char('?'),
                (false, true) => f.write_char('.'),
                (true, false) => f.write_char('#'),
                _ => panic!("unexpected"),
            };
        });

        f.write_char(' ');
        f.write_str(
            self.groups
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct CacheKey {
    from: u32,
    group_index: usize,
}
