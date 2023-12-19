use std::collections::HashMap;
use std::ops::RangeInclusive;

use Condition::Fallback;

use crate::Comparison::{GreaterThan, SmallerThan};
use crate::Condition::Compare;
use crate::Parameter::{A, M, S, X};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    print_answer("one (example)", &one(EXAMPLE), "19114");
    print_answer("one", &one(INPUT), "362930");
    print_answer("two (example)", &two(EXAMPLE), "167409079868000");
    print_answer("two", &two(INPUT), "116365820987729");
}

fn print_answer(name: &str, actual: &str, expected: &str) {
    match actual == expected {
        true => println!("{name}: {actual} (OK)"),
        false => println!("{name}: {actual} (ERROR: expected {expected})"),
    }
}

fn one(input: &str) -> String {
    let world = World::parse(input);

    world
        .parts
        .iter()
        .filter(|part| world.eval(part) == Decision::Accept)
        .map(|part| part.rating())
        .sum::<u64>()
        .to_string()
}

fn two(input: &str) -> String {
    World::parse(input)
        .accepted_part_domains(&PartDomain::new())
        .iter()
        .map(|part_domain| part_domain.combinations())
        .sum::<u64>()
        .to_string()
}

type RuleSetName = String;

struct World {
    rule_sets: HashMap<RuleSetName, RuleSet>,
    parts: Vec<Part>,
}

impl World {
    fn parse(input: &str) -> Self {
        let (rule_sets, parts) = input.split_once("\n\n").expect("two sections");

        let rule_sets: HashMap<RuleSetName, RuleSet> = rule_sets
            .split('\n')
            .map(RuleSet::parse)
            .map(|rule_set| (rule_set.name.clone(), rule_set))
            .collect();
        let parts: Vec<Part> = parts.split('\n').map(Part::parse).collect();

        Self { rule_sets, parts }
    }

    fn eval(&self, part: &Part) -> Decision {
        self.eval_rule(part, &self.rule_sets["in"])
    }

    fn eval_rule(&self, part: &Part, rule_set: &RuleSet) -> Decision {
        match rule_set.eval(part) {
            Action::Done(decision) => decision,
            Action::Move(rule_set_name) => self.eval_rule(part, &self.rule_sets[&rule_set_name]),
        }
    }

    fn accepted_part_domains(&self, part_domain: &PartDomain) -> Vec<PartDomain> {
        self.accepted_part_domains_rule(part_domain, &"in".to_string())
    }

    fn accepted_part_domains_rule(
        &self,
        part_domain: &PartDomain,
        rule_set_name: &RuleSetName,
    ) -> Vec<PartDomain> {
        let rule_set = &self.rule_sets[rule_set_name];

        let mut remaining_domain = Some(part_domain.clone());
        let mut results: Vec<PartDomain> = rule_set
            .rules
            .iter()
            .flat_map(|rule| match remaining_domain.clone() {
                None => vec![],
                Some(remaining) => {
                    let (kept_domain, rule_domain) = rule.condition.split(&remaining);
                    remaining_domain = kept_domain;

                    match rule_domain {
                        None => vec![],
                        Some(some_rule_domain) => match &rule.action {
                            Action::Done(decision) => match decision {
                                Decision::Accept => vec![some_rule_domain],
                                Decision::Reject => vec![],
                            },
                            Action::Move(next_rule_set_name) => self
                                .accepted_part_domains_rule(&some_rule_domain, next_rule_set_name),
                        },
                    }
                }
            })
            .collect();

        if let Some(remaining) = remaining_domain {
            results.push(remaining)
        }

        results
    }
}

struct RuleSet {
    name: RuleSetName,
    rules: Vec<Rule>,
}

impl RuleSet {
    fn parse(input: &str) -> Self {
        let condition_index = input.find('{').expect("start of conditions");
        let name: RuleSetName = input[0..condition_index].to_string();

        let rules = input[condition_index + 1..input.len() - 1]
            .split(',')
            .map(Rule::parse)
            .collect();

        RuleSet { name, rules }
    }

    fn eval(&self, part: &Part) -> Action {
        self.rules
            .iter()
            .find(|rule| rule.condition.matches(part))
            .expect("a rule to match")
            .action
            .clone()
    }
}

#[derive(Clone)]
struct Rule {
    condition: Condition,
    action: Action,
}

impl Rule {
    fn parse(input: &str) -> Self {
        match input.split_once(':') {
            None => Rule {
                condition: Fallback,
                action: Action::parse(input),
            },
            Some((condition, action)) => Rule {
                condition: Compare(CompareCondition::parse(condition)),
                action: Action::parse(action),
            },
        }
    }
}

#[derive(Clone)]
enum Action {
    Done(Decision),
    Move(RuleSetName),
}

#[derive(Clone, Eq, PartialEq)]
enum Decision {
    Accept,
    Reject,
}

impl Action {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Action::Done(Decision::Accept),
            "R" => Action::Done(Decision::Reject),
            _ => Action::Move(input.to_string()),
        }
    }
}

#[derive(Clone)]
enum Condition {
    Compare(CompareCondition),
    Fallback,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Compare(compare_condition) => compare_condition.matches(part),
            Fallback => true,
        }
    }

    fn split(&self, part_domain: &PartDomain) -> (Option<PartDomain>, Option<PartDomain>) {
        match self {
            Compare(compare_condition) => compare_condition.split(part_domain),
            Fallback => (None, Some(part_domain.clone())),
        }
    }
}

#[derive(Clone)]
struct CompareCondition {
    parameter: Parameter,
    comparison: Comparison,
    value: u64,
}

impl CompareCondition {
    fn matches(&self, part: &Part) -> bool {
        match self.comparison {
            GreaterThan => part.values[&self.parameter] > self.value,
            SmallerThan => part.values[&self.parameter] < self.value,
        }
    }

    fn split(&self, part_domain: &PartDomain) -> (Option<PartDomain>, Option<PartDomain>) {
        let range = &part_domain.domains[&self.parameter];
        let start = *range.start();
        let end = *range.end();

        match self.comparison {
            GreaterThan => {
                let unchanged = start > self.value;
                let removed = end < self.value;
                let reduced = range.contains(&self.value);

                match (unchanged, removed, reduced) {
                    (true, false, false) => (None, Some(part_domain.clone())),
                    (false, true, false) => (Some(part_domain.clone()), None),
                    (false, false, true) => (
                        Some(part_domain.with_parameter(&self.parameter, start..=self.value)),
                        Some(part_domain.with_parameter(&self.parameter, self.value + 1..=end)),
                    ),
                    _ => panic!("unexpected"),
                }
            }
            SmallerThan => {
                let unchanged = end < self.value;
                let removed = start > self.value;
                let reduced = range.contains(&self.value);

                match (unchanged, removed, reduced) {
                    (true, false, false) => (None, Some(part_domain.clone())),
                    (false, true, false) => (Some(part_domain.clone()), None),
                    (false, false, true) => (
                        Some(part_domain.with_parameter(&self.parameter, self.value..=end)),
                        Some(part_domain.with_parameter(&self.parameter, start..=self.value - 1)),
                    ),
                    _ => panic!("unexpected"),
                }
            }
        }
    }

    fn parse(input: &str) -> Self {
        let comparison_index = input.find('<').or(input.find('>')).expect("< or >");
        let parameter = Parameter::parse(&input[0..comparison_index]);
        let comparison = Comparison::parse(&input[comparison_index..=comparison_index]);
        let value: u64 = input[comparison_index + 1..].parse().expect("number");

        CompareCondition {
            parameter,
            comparison,
            value,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Parameter {
    X,
    M,
    A,
    S,
}

impl Parameter {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Parameter::X,
            "m" => Parameter::M,
            "a" => Parameter::A,
            "s" => Parameter::S,
            _ => panic!("unexpected parameter"),
        }
    }
}

#[derive(Clone)]
enum Comparison {
    GreaterThan,
    SmallerThan,
}

impl Comparison {
    fn parse(input: &str) -> Self {
        match input {
            "<" => SmallerThan,
            ">" => GreaterThan,
            _ => panic!("unexpected comparison"),
        }
    }
}

#[derive(Clone)]
struct PartDomain {
    domains: HashMap<Parameter, RangeInclusive<u64>>,
}

impl PartDomain {
    fn new() -> Self {
        Self {
            domains: [X, M, A, S]
                .iter()
                .map(|param| (param.clone(), 1..=4000))
                .collect(),
        }
    }

    fn with_parameter(&self, parameter: &Parameter, new_value: RangeInclusive<u64>) -> Self {
        Self {
            domains: self
                .domains
                .iter()
                .map(|(key, current_value)| {
                    (
                        key.clone(),
                        match key == parameter {
                            true => new_value.clone(),
                            false => current_value.clone(),
                        },
                    )
                })
                .collect(),
        }
    }

    fn combinations(&self) -> u64 {
        self.domains
            .values()
            .map(|domain| domain.end() - domain.start() + 1)
            .product::<u64>()
    }
}

struct Part {
    values: HashMap<Parameter, u64>,
}

impl Part {
    fn parse(input: &str) -> Self {
        Self {
            values: input[1..input.len() - 1]
                .split(',')
                .map(|segment| segment.split_once('=').expect("assignment"))
                .map(|(parameter, value)| {
                    (
                        Parameter::parse(parameter),
                        value.parse::<u64>().expect("number"),
                    )
                })
                .collect(),
        }
    }

    fn rating(&self) -> u64 {
        self.values.values().sum()
    }
}
