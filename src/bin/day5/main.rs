use std::str::FromStr;

use logos::Logos;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
  let input = include_str!("input.txt").trim();

  let (cargo, instructions) = input.split_once("\n\n").unwrap();

  let mut builder = CargoBuilder::default();
  for line in cargo.split('\n').take_while(|l| !l.starts_with(' ')) {
    for (index, token) in Lexer::new(line).enumerate() {
      if let Token::Crate(label) = token {
        builder.push_crate(index, label);
      }
    }
  }
  let cargo = builder.finish();

  let instructions = instructions
    .split('\n')
    .map(|inst| inst.parse::<Instruction>().unwrap())
    .collect::<Vec<_>>();

  {
    let mut cargo = cargo.clone();
    for inst in instructions.iter() {
      cargo.execute(*inst, Version::_9000);
    }

    let mut result = String::new();
    for stack in cargo.stacks.iter() {
      result += stack[stack.len() - 1];
    }
    println!("Day 1 part 1 answer: {}", result);
  }

  {
    let mut cargo = cargo.clone();
    for inst in instructions.iter() {
      cargo.execute(*inst, Version::_9001);
    }

    let mut result = String::new();
    for stack in cargo.stacks.iter() {
      result += stack[stack.len() - 1];
    }
    println!("Day 1 part 2 answer: {}", result);
  }
}

#[derive(Debug, Default)]
struct CargoBuilder<'a> {
  stacks: Vec<Vec<&'a str>>,
}

impl<'a> CargoBuilder<'a> {
  fn push_crate(&mut self, index: usize, label: &'a str) {
    if index >= self.stacks.len() {
      for _ in 0..index - self.stacks.len() + 1 {
        self.stacks.push(vec![]);
      }
    }

    self.stacks[index].push(label);
  }

  fn finish(mut self) -> Cargo<'a> {
    for stack in self.stacks.iter_mut() {
      stack.reverse();
    }
    Cargo {
      stacks: self.stacks,
    }
  }
}

#[derive(Clone)]
struct Cargo<'a> {
  stacks: Vec<Vec<&'a str>>,
}

enum Version {
  _9000,
  _9001,
}

impl<'a> Cargo<'a> {
  fn execute(&mut self, inst: Instruction, version: Version) {
    let Instruction { quantity, from, to } = inst;

    let mut iter = self.stacks.iter_mut();
    let (src, dest) = match from.cmp(&to) {
      std::cmp::Ordering::Less => {
        let src = iter.nth(from).unwrap();
        let dest = iter.nth(to - from - 1).unwrap();
        (src, dest)
      }
      std::cmp::Ordering::Greater => {
        let dest = iter.nth(to).unwrap();
        let src = iter.nth(from - to - 1).unwrap();
        (src, dest)
      }
      std::cmp::Ordering::Equal => panic!(
        "cannot move {} elements from stack {} to itself",
        quantity, from
      ),
    };

    match version {
      Version::_9000 => dest.extend(src.drain(src.len() - quantity..src.len()).rev()),
      Version::_9001 => dest.extend(src.drain(src.len() - quantity..src.len())),
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
  quantity: usize,
  from: usize,
  to: usize,
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    static REGEX: Lazy<Regex> =
      Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

    let cap = REGEX.captures(s).unwrap();

    Ok(Instruction {
      quantity: cap[1].parse().unwrap(),
      // crate indices use 1-indexing
      from: cap[2].parse::<usize>().unwrap() - 1usize,
      to: cap[3].parse::<usize>().unwrap() - 1usize,
    })
  }
}

#[derive(Clone, Copy, Debug, Logos)]
enum Token<'a> {
  #[regex(r"\[\w\]", lex_crate)]
  Crate(&'a str),
  #[token(r"   ", lex_empty)]
  Empty,
  #[error]
  Error,
}

type Lexer<'a> = logos::Lexer<'a, Token<'a>>;

fn lex_crate<'a>(lexer: &mut Lexer<'a>) -> &'a str {
  let lexeme = lexer.slice();
  let lexeme = &lexeme[1..lexeme.len() - 1];
  if !lexer.remainder().is_empty() {
    lexer.bump(1);
  }
  lexeme
}

fn lex_empty(lexer: &mut Lexer<'_>) {
  if !lexer.remainder().is_empty() {
    lexer.bump(1);
  }
}
