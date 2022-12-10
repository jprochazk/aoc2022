use std::str::FromStr;

use itertools::Itertools;

macro_rules! noop {
  () => {
    |_| {}
  };
}

fn main() {
  let input = include_str!("input.txt").trim();

  let mut sum = 0;
  let mut cpu = Cpu::new(|&State { clock, x }| {
    if clock >= 20 && (clock - 20) % 40 == 0 {
      sum += clock as i64 * x;
    }
  });

  for line in input.split('\n').map(str::trim) {
    let op: Op = line.parse().unwrap();
    cpu.exec(op);
  }

  println!("Day 10 part 1 answer: {}", sum);

  let width = 40;
  let height = 6;
  let mut crt = String::with_capacity(width * height);

  let mut cpu = Cpu::new(|&State { x, .. }| {
    use std::fmt::Write;
    let pixel = (crt.len() % width) as i64;
    if x - 1 <= pixel && pixel <= x + 1 {
      write!(&mut crt, "#").unwrap();
    } else {
      write!(&mut crt, ".").unwrap();
    }
  });

  for line in input.split('\n').map(str::trim) {
    let op: Op = line.parse().unwrap();
    cpu.exec(op);
  }

  println!(
    "Day 10 part 2 answer: \n{}",
    crt
      .chars()
      .chunks(width)
      .into_iter()
      .map(|mut c| c.join(""))
      .join("\n")
  );
}

#[derive(Clone, Copy)]
struct State {
  clock: u64,
  x: i64,
}

struct Cpu<Hook>
where
  Hook: FnMut(&State),
{
  state: State,
  hook: Hook,
}

impl<Hook> Cpu<Hook>
where
  Hook: FnMut(&State),
{
  fn new(hook: Hook) -> Self {
    Self {
      state: State { clock: 1, x: 1 },
      hook,
    }
  }

  fn exec(&mut self, op: Op) {
    match op {
      Op::Addx(n) => self.addx(n),
      Op::Noop => self.noop(),
    }
  }

  fn tick<H: FnOnce(&mut Self)>(&mut self, mutate: H) {
    // start cycle
    (self.hook)(&self.state);

    mutate(self);
    self.state.clock += 1;
  }

  fn addx(&mut self, n: i64) {
    self.tick(noop!());
    self.tick(|c| c.state.x += n);
  }

  fn noop(&mut self) {
    self.tick(noop!());
  }
}

enum Op {
  Addx(i64),
  Noop,
}

impl FromStr for Op {
  type Err = ();

  fn from_str(inst: &str) -> Result<Self, Self::Err> {
    use Op::*;
    match inst.split_once(' ') {
      Some((inst, arg)) => match inst {
        "addx" => Ok(Addx(arg.parse().unwrap())),
        _ => Err(()),
      },
      None => match inst {
        "noop" => Ok(Noop),
        _ => Err(()),
      },
    }
  }
}
