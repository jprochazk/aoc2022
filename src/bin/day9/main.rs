use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

macro_rules! pos {
  ($x:expr, $y:expr) => {{
    Position { x: $x, y: $y }
  }};
}

fn main() {
  let input = include_str!("input.txt").trim();

  // UP    +Y
  // DOWN  -Y
  // LEFT  -X
  // RIGHT +X

  {
    let mut world = World::new(1);

    for move_ in input.split('\n').map(|m| Move::from_str(m).unwrap()) {
      world.simulate(move_);
      //println!("STEP: {move_:?}");
      //println!("{world}");
    }

    println!("{world}");
    println!("Day 9 part 1 answer: {}", world.visited.len());
  }

  {
    let mut world = World::new(9);

    for move_ in input.split('\n').map(|m| Move::from_str(m).unwrap()) {
      world.simulate(move_);
      //println!("STEP: {move_:?}");
      //println!("{world}");
    }

    println!("{world}");
    println!("Day 9 part 2 answer: {}", world.visited.len());
  }
}

#[derive(Default)]
struct World {
  head: Position,
  knots: Vec<Position>,
  bounds: Bounds,
  visited: HashSet<Position>,
}

impl World {
  fn new(length: usize) -> Self {
    Self {
      head: Position::default(),
      knots: vec![Position::default(); length],
      bounds: Bounds::default(),
      visited: [pos!(0, 0)].into_iter().collect(),
    }
  }

  fn simulate(&mut self, move_: Move) {
    for _ in 0..move_.count() {
      self.step(move_.direction());
    }
  }

  fn step(&mut self, dir: Direction) {
    // move head
    match dir {
      Direction::Up => self.head.y += 1,
      Direction::Down => self.head.y -= 1,
      Direction::Left => self.head.x -= 1,
      Direction::Right => self.head.x += 1,
    }

    // move tail to catch up
    let mut prev = &self.head;
    for current in self.knots.iter_mut() {
      apply_knot_constraint(prev, current);
      prev = &*current;
    }

    // expand bounds
    self.bounds.expand(&self.head);
    for knot in self.knots.iter() {
      self.bounds.expand(knot);
    }

    // record tail (final knot) position
    if let Some(last) = self.knots.last() {
      self.visited.insert(*last);
    }
  }
}

fn apply_knot_constraint(head: &Position, tail: &mut Position) {
  let (dx, dy) = (head.x - tail.x, head.y - tail.y);
  if dx.abs() > 1 || dy.abs() > 1 {
    tail.x += dx.signum();
    tail.y += dy.signum();
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
  x: i64,
  y: i64,
}

impl Position {
  fn zero() -> Self {
    Self { x: 0, y: 0 }
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct Bounds {
  min_x: i64,
  max_x: i64,
  min_y: i64,
  max_y: i64,
}

impl Bounds {
  fn expand(&mut self, pos: &Position) {
    if pos.x > self.max_x {
      self.max_x = pos.x;
    }

    if pos.x < self.min_x {
      self.min_x = pos.x;
    }

    if pos.y > self.max_y {
      self.max_y = pos.y;
    }

    if pos.y < self.min_y {
      self.min_y = pos.y;
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Move {
  Up(u64),
  Down(u64),
  Left(u64),
  Right(u64),
}

impl Move {
  fn direction(&self) -> Direction {
    match self {
      Move::Up(_) => Direction::Up,
      Move::Down(_) => Direction::Down,
      Move::Left(_) => Direction::Left,
      Move::Right(_) => Direction::Right,
    }
  }

  fn count(&self) -> u64 {
    match self {
      Move::Up(n) => *n,
      Move::Down(n) => *n,
      Move::Left(n) => *n,
      Move::Right(n) => *n,
    }
  }
}

impl FromStr for Move {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use Move::*;
    let (dir, n) = s.split_once(' ').ok_or(())?;
    let n = n.parse::<u64>().map_err(|_| ())?;
    match dir {
      "U" => Ok(Up(n)),
      "D" => Ok(Down(n)),
      "L" => Ok(Left(n)),
      "R" => Ok(Right(n)),
      _ => Err(()),
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl fmt::Display for World {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in (self.bounds.min_y..=self.bounds.max_y).rev() {
      for x in self.bounds.min_x..=self.bounds.max_x {
        let pos = pos!(x, y);

        // write head
        if self.head == pos {
          write!(f, "H")?;
          continue;
        }

        // write knot
        if let Some(idx) = self.knots.iter().position(|&p| p == pos) {
          // knots are 1-indexed
          write!(f, "{}", idx + 1)?;
          continue;
        }

        if Position::zero() == pos {
          write!(f, "s")?;
          continue;
        }

        if self.visited.contains(&pos) {
          write!(f, "#")?;
          continue;
        }

        write!(f, ".")?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}
