use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
  let input = include_str!("input.txt").trim();

  let mut matches = 0;
  for pair in input.split('\n') {
    let (a, b) = pair.split_once(',').unwrap();
    let (a, b): (Span, Span) = (a.parse().unwrap(), b.parse().unwrap());

    if a.contains(b) || b.contains(a) {
      matches += 1;
    }
  }
  println!("Day 4 part 1 answer: {}", matches);

  let mut matches = 0;
  for pair in input.split('\n') {
    let (a, b) = pair.split_once(',').unwrap();
    let (a, b): (Span, Span) = (a.parse().unwrap(), b.parse().unwrap());

    if a.overlap(b) {
      matches += 1;
    }
  }
  println!("Day 4 part 2 answer: {}", matches);
}

#[derive(Clone, Copy)]
struct Span {
  start: usize,
  end: usize,
}

impl Span {
  fn contains(&self, other: Span) -> bool {
    self.start <= other.start && other.end <= self.end
  }

  fn overlap(&self, other: Span) -> bool {
    self.start <= other.end && self.end >= other.start
  }
}

impl FromStr for Span {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (start, end) = s.split_once('-').unwrap();

    Ok(Span {
      start: start.parse().unwrap(),
      end: end.parse().unwrap(),
    })
  }
}
