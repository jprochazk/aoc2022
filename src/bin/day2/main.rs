use std::str::FromStr;

fn main() {
  let input = include_str!("input.txt").trim();

  let strategy = input
    .split('\n')
    .map(|round| round.split_once(' ').unwrap())
    .collect::<Vec<_>>();

  let mut total = 0;
  for (opponent, me) in strategy.iter() {
    let opponent = opponent.parse::<Shape>().unwrap();
    let choice = me.parse::<Shape>().unwrap();

    let score = choice.value() + choice.play(opponent);

    total += score;
  }
  println!("Day 1 part 1 answer: {}", total);

  let mut total = 0;
  for (opponent, me) in strategy.iter() {
    let opponent = opponent.parse::<Shape>().unwrap();
    let choice = me.parse::<Outcome>().unwrap().choose(opponent);

    let score = choice.value() + choice.play(opponent);

    total += score;
  }
  println!("Day 1 part 2 answer: {}", total);
}

#[derive(Clone, Copy)]
enum Outcome {
  Win,
  Lose,
  Draw,
}

impl Outcome {
  fn choose(&self, them: Shape) -> Shape {
    use Outcome::*;
    use Shape::*;

    match (self, them) {
      (Draw, any) => any,
      (Lose, Rock) | (Win, Paper) => Scissors,
      (Lose, Scissors) | (Win, Rock) => Paper,
      (Lose, Paper) | (Win, Scissors) => Rock,
    }
  }
}

#[derive(Clone, Copy)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

impl Shape {
  fn value(&self) -> u64 {
    match self {
      Shape::Rock => 1,
      Shape::Paper => 2,
      Shape::Scissors => 3,
    }
  }

  fn play(&self, other: Shape) -> u64 {
    use Shape::*;

    match (self, other) {
      // win
      (Scissors, Paper) | (Paper, Rock) | (Rock, Scissors) => 6,
      // loss
      (Paper, Scissors) | (Rock, Paper) | (Scissors, Rock) => 0,
      // draw
      _ => 3,
    }
  }
}

impl FromStr for Outcome {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use Outcome::*;

    let outcome = match s {
      "X" => Lose,
      "Y" => Draw,
      "Z" => Win,
      _ => return Err(()),
    };

    Ok(outcome)
  }
}

impl FromStr for Shape {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use Shape::*;

    let shape = match s {
      "A" | "X" => Rock,
      "B" | "Y" => Paper,
      "C" | "Z" => Scissors,
      _ => return Err(()),
    };

    Ok(shape)
  }
}
