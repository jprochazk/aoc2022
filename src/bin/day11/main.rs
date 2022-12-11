#![allow(dead_code)]

use std::fmt::Debug;

use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
  let input = include_str!("input.txt").trim();

  aoc::time(
    || {
      let mut monkeys = vec![];

      for monkey in input.split("\n\n").map(str::trim) {
        monkeys.push(parse_monkey(monkey));
      }

      monkey_business(monkeys, 20, 3)
    },
    |monkey_business| {
      println!("Day 11 part 1 answer: {}", monkey_business);
    },
  );

  aoc::time(
    || {
      let mut monkeys = vec![];

      for monkey in input.split("\n\n").map(str::trim) {
        monkeys.push(parse_monkey(monkey));
      }

      monkey_business(monkeys, 10000, 1)
    },
    |monkey_business| {
      println!("Day 11 part 2 answer: {}", monkey_business);
    },
  );
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, worry_div: u64) -> u64 {
  let mut temp = Vec::with_capacity(64);

  // all divisors are prime, so avoid overflow by using modular arithmetic
  let modulo = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();

  for _ in 0..rounds {
    for i in 0..monkeys.len() {
      temp.append(&mut monkeys[i].items);
      for item in temp.iter().cloned() {
        monkeys[i].inspected += 1;
        let item = monkeys[i].op.apply(item);
        let item = item % modulo;
        let item = item / worry_div;
        let target = monkeys[i].test.target(item);
        monkeys[target].items.push(item);
      }
      temp.clear();
    }
  }

  monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
  monkeys[0].inspected * monkeys[1].inspected
}

#[derive(Debug)]
struct Monkey {
  id: usize,
  items: Vec<u64>,
  op: Op,
  test: Test,
  inspected: u64,
}

#[derive(Debug)]
enum Op {
  AddN(u64),
  MulN(u64),
  Square,
}

impl Op {
  fn apply(&self, level: u64) -> u64 {
    // match self {
    //   Op::AddN(n) => println!("    Worry level increases by {n} to {}.", level + n),
    //   Op::AddSelf => println!("    Worry level increases by {level} to {}.", level + level),
    //   Op::MulN(n) => println!("    Worry level is multiplied by {n} to {}.", level * n),
    //   Op::MulSelf => println!(
    //     "    Worry level is multiplied by itself to {}.",
    //     level * level
    //   ),
    // }
    match self {
      Op::AddN(n) => level + n,
      Op::MulN(n) => level * n,
      Op::Square => level * level,
    }
  }
}

#[derive(Debug)]
struct Test {
  divisible_by: u64,
  if_true: usize,
  if_false: usize,
}

impl Test {
  fn target(&self, item: u64) -> usize {
    // match item % self.divisible_by == 0 {
    //   true => println!(
    //     "    Current worry level is divisible by {}",
    //     self.divisible_by
    //   ),
    //   false => println!(
    //     "    Current worry level is not divisible by {}",
    //     self.divisible_by
    //   ),
    // }
    match item % self.divisible_by == 0 {
      true => self.if_true,
      false => self.if_false,
    }
  }
}

fn parse_monkey(s: &str) -> Monkey {
  // Monkey <id>:
  //   Starting items: <item>,*
  //   Operation: new = old <op> <op_arg>
  //   Test: divisible by <divisible_by>
  //     If true: throw to monkey <if_true>
  //     If false: throw to monkey <if_false>
  static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
      r"(?x)
        Monkey\s(?P<id>\d+):\n\s+
        Starting\sitems:\s(?P<items>(?:\d+)(?:,\s\d+)*)\n\s+
        Operation:\snew\s=\sold\s(?P<op>[*+])\s(?P<op_arg>\d+|\w+)\n\s+
        Test:\sdivisible\sby\s(?P<div>\d+)\n\s+
        If\strue:\sthrow\sto\smonkey\s(?P<if_true>\d+)\n\s+
        If\sfalse:\sthrow\sto\smonkey\s(?P<if_false>\d+)
    ",
    )
    .unwrap()
  });

  let cap = REGEX.captures(s).unwrap();

  let id = cap["id"].parse().unwrap();
  let items = cap["items"]
    .split(", ")
    .map(|v| v.parse().unwrap())
    .collect();
  let op = match &cap["op"] {
    "*" => match &cap["op_arg"] {
      "old" => Op::Square,
      v => Op::MulN(v.parse().unwrap()),
    },
    "+" => match &cap["op_arg"] {
      "old" => panic!("invalid add arg `old`"),
      v => Op::AddN(v.parse().unwrap()),
    },
    _ => panic!("invalid operator `{}`", &cap["op"]),
  };
  let test = Test {
    divisible_by: cap["div"].parse().unwrap(),
    if_true: cap["if_true"].parse().unwrap(),
    if_false: cap["if_false"].parse().unwrap(),
  };

  Monkey {
    id,
    items,
    op,
    test,
    inspected: 0,
  }
}
