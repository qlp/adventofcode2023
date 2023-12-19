use crate::Comparison::{GreaterThan, SmallerThan};
use crate::Parameter::{A_PARAM, M_PARAM, S_PARAM, X_PARAM};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE: &str = include_str!("example.txt");

fn main() {
    // print_answer("one (example)", &one(EXAMPLE), "19114");
    // print_answer("one", &one(INPUT), "362930");
    print_answer("two (example)", &two(EXAMPLE), "167409079868000");
    // print_answer("two", &two(INPUT), "");
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
    let world = World::parse(input);

    world
        .accepted_part_domains(PartDomain::new())
        .iter()
        .map(|part_domain| {
            part_domain
                .domains
                .values()
                .map(|domain| domain.end() - domain.start() + 1)
                .product::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

type RuleSetName = String;

struct World {
    rule_sets: IndexMap<RuleSetName, RuleSet>,
    parts: Vec<Part>,
}

impl World {
    fn parse(input: &str) -> Self {
        let (rule_sets, parts) = input.split_once("\n\n").expect("two sections");

        let rule_sets: IndexMap<RuleSetName, RuleSet> = rule_sets
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

    fn accepted_part_domains(&self, part_domain: PartDomain) -> Vec<PartDomain> {
        Vec::new()
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule_sets.iter().for_each(|(_, rule_set)| {
            rule_set.fmt(f).unwrap();
            f.write_char('\n').unwrap();
        });

        f.write_char('\n').unwrap();
        self.parts.iter().for_each(|part| {
            part.fmt(f).unwrap();
            f.write_char('\n').unwrap();
        });

        Ok(())
    }
}

struct RuleSet {
    name: RuleSetName,
    rules: Vec<Rule>,
    fallback: Action,
}

impl RuleSet {
    fn parse(input: &str) -> Self {
        let condition_index = input.find('{').expect("start of conditions");
        let name: RuleSetName = input[0..condition_index].to_string();

        let fallback_index = input.rfind(',').expect("at least one rule");
        let fallback = Action::parse(&input[fallback_index + 1..input.len() - 1]);

        let rules = input[condition_index + 1..fallback_index]
            .split(',')
            .map(|rule| {
                let (condition, action) = rule.split_once(':').expect("condition and action");
                let condition = Condition::parse(condition);
                let action = Action::parse(action);

                Rule { condition, action }
            })
            .collect();

        RuleSet {
            name,
            rules,
            fallback,
        }
    }

    fn eval(&self, part: &Part) -> Action {
        self.rules
            .iter()
            .find(|rule| rule.condition.matches(part))
            .map_or(self.fallback.clone(), |rule| rule.action.clone())
    }
}

impl Display for RuleSet {
    // px{a<2006:qkq,m>2090:A,rfg}

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{{", self.name))?;

        self.rules.iter().for_each(|rule| {
            rule.fmt(f).unwrap();
            f.write_char(',').unwrap();
        });

        f.write_fmt(format_args!("{}}}", self.fallback))
    }
}

#[derive(Clone)]
struct Rule {
    condition: Condition,
    action: Action,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.condition.fmt(f)?;
        f.write_char(':')?;
        self.action.fmt(f)
    }
}

#[derive(Clone)]
enum Action {
    Done(Decision),
    Move(RuleSetName),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Done(decision) => decision.fmt(f),
            Action::Move(rule_set_name) => f.write_fmt(format_args!("{}", rule_set_name)),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Decision {
    Accept,
    Reject,
}

impl Display for Decision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Decision::Accept => 'A',
            Decision::Reject => 'R',
        })
    }
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
struct Condition {
    parameter: Parameter,
    comparison: Comparison,
    value: u64,
}

impl Condition {
    fn matches(&self, part: &Part) -> bool {
        match self.comparison {
            GreaterThan => part.values[&self.parameter] > self.value,
            SmallerThan => part.values[&self.parameter] < self.value,
        }
    }

    fn parse(input: &str) -> Self {
        let comparison_index = input.find('<').or(input.find('>')).expect("< or >");
        let parameter = Parameter::parse(&input[0..comparison_index]);
        let comparison = Comparison::parse(&input[comparison_index..=comparison_index]);
        let value: u64 = input[comparison_index + 1..].parse().expect("number");

        Condition {
            parameter,
            comparison,
            value,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.parameter.fmt(f)?;
        self.comparison.fmt(f)?;

        f.write_fmt(format_args!("{}", self.value))
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Parameter {
    X_PARAM,
    M_PARAM,
    A_PARAM,
    S_PARAM,
}

impl Parameter {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Parameter::X_PARAM,
            "m" => Parameter::M_PARAM,
            "a" => Parameter::A_PARAM,
            "s" => Parameter::S_PARAM,
            _ => panic!("unexpected parameter"),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            X_PARAM => 'x',
            M_PARAM => 'm',
            A_PARAM => 'a',
            S_PARAM => 's',
        })
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

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            GreaterThan => '>',
            SmallerThan => '<',
        })
    }
}

struct PartDomain {
    domains: HashMap<Parameter, RangeInclusive<u64>>,
}

impl PartDomain {
    fn new() -> Self {
        Self {
            domains: [X_PARAM, M_PARAM, A_PARAM, S_PARAM]
                .iter()
                .map(|param| (param.clone(), 1..=4000))
                .collect(),
        }
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

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{x={},m={},a={},s={}}}",
            self.values[&X_PARAM],
            self.values[&M_PARAM],
            self.values[&A_PARAM],
            self.values[&S_PARAM]
        ))
    }
}
