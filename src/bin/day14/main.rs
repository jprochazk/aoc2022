#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;

macro_rules! pos {
  ($x:expr, $y: expr) => {{
    Pos { x: $x, y: $y }
  }};
}

fn main() {
  let input = include_str!("input.txt").trim();

  aoc::time(
    || {
      let mut grid = parse(input);
      let n = simulate(&mut grid);
      (grid, n)
    },
    |(grid, n)| {
      // println!("{grid}");
      println!("Day 14 part 1 answer: {n}");
    },
  );

  aoc::time(
    || {
      let mut grid = DynamicGrid::from(parse(input));
      let n = simulate(&mut grid);
      (grid, n)
    },
    |(grid, n)| {
      // println!("{grid}");
      println!("Day 14 part 2 answer: {n}");
    },
  )
}

fn simulate<T: Grid>(grid: &mut T) -> u64 {
  let mut units = 0;

  // while we can place another piece of sand
  'simulation: loop {
    // simulate the unit of sand until it goes to rest or goes out of bounds
    let mut sand = pos!(500, 0);
    while let Some(move_) = maybe_move(grid, sand) {
      match move_ {
        Move::Coords(pos) => {
          sand = pos;
        }
        // stop at out of bounds, because any future unit of sand will also go out of bounds
        Move::OutOfBounds => {
          break 'simulation;
        }
      }

      // TEMP: debug printing
      // grid.set(sand, Cell::Sand);
      // println!("{grid}");
      // grid.set(sand, Cell::Air);
    }

    // no more moves and not out of bounds = at rest
    grid.set(sand, Cell::Sand);
    units += 1;

    if sand == pos!(500, 0) {
      break 'simulation;
    }
  }

  units
}

enum Move {
  Coords(Pos),
  OutOfBounds,
}

fn maybe_move<T: Grid>(grid: &T, sand: Pos) -> Option<Move> {
  let possible_moves = [
    // below
    pos!(sand.x, sand.y + 1),
    // below + left
    pos!(sand.x - 1, sand.y + 1),
    // below + right
    pos!(sand.x + 1, sand.y + 1),
  ];

  for move_ in possible_moves {
    match grid.get(move_) {
      Some(Cell::Air) => return Some(Move::Coords(move_)),
      Some(Cell::Rock | Cell::Sand) => continue,
      None => return Some(Move::OutOfBounds),
    }
  }

  None
}

fn parse(s: &str) -> StaticGrid {
  let mut paths = vec![];
  let mut bounds = Bounds::new();
  bounds.update((500, 0));

  // parse paths
  for path in s.split('\n') {
    let mut coords = vec![];
    for coord in path.split(" -> ") {
      let (x, y) = coord.split_once(',').unwrap();
      let pos: Pos = pos!(x.parse().unwrap(), y.parse().unwrap());

      coords.push(pos);
      bounds.update(pos);
    }
    paths.push(coords);
  }

  // init grid
  let mut grid = StaticGrid::new(bounds);
  for path in paths {
    let mut coords = path.iter();

    let mut prev = coords.next().unwrap();
    grid.set(*prev, Cell::Rock);

    for next in coords {
      // vertical line
      if prev.y != next.y {
        let col = if prev.y < next.y {
          prev.y..=next.y
        } else {
          next.y..=prev.y
        };
        for y in col {
          grid.set((prev.x, y), Cell::Rock);
        }
      }

      // horizontal line
      if prev.x != next.x {
        let row = if prev.x < next.x {
          prev.x..=next.x
        } else {
          next.x..=prev.x
        };
        for x in row {
          grid.set((x, prev.y), Cell::Rock);
        }
      }

      prev = next;
    }
  }

  grid
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
enum Cell {
  #[default]
  Air,
  Rock,
  Sand,
}

trait Grid: fmt::Display {
  fn get(&self, pos: impl Into<Pos>) -> Option<Cell>;
  fn set(&mut self, pos: impl Into<Pos>, cell: Cell);
}

impl Grid for StaticGrid {
  fn get(&self, pos: impl Into<Pos>) -> Option<Cell> {
    let pos = pos.into();
    if pos.x < self.bounds.min_x
      || pos.y < self.bounds.min_y
      || pos.x > self.bounds.max_x
      || pos.y > self.bounds.max_y
    {
      return None;
    }
    let (x, y) = (pos.x - self.bounds.min_x, pos.y - self.bounds.min_y);
    self.data.get(y * self.bounds.width() + x).cloned()
  }

  fn set(&mut self, pos: impl Into<Pos>, cell: Cell) {
    let pos = pos.into();
    let (x, y) = (pos.x - self.bounds.min_x, pos.y - self.bounds.min_y);
    self.data[y * self.bounds.width() + x] = cell;
  }
}

impl Grid for DynamicGrid {
  fn get(&self, pos: impl Into<Pos>) -> Option<Cell> {
    let pos = pos.into();
    if pos.y >= self.floor_y {
      return Some(Cell::Rock);
    }

    self.data.get(&pos).cloned().or(Some(Cell::Air))
  }

  fn set(&mut self, pos: impl Into<Pos>, cell: Cell) {
    let pos = pos.into();
    self.data.insert(pos, cell);
    self.bounds.update(pos);
  }
}

struct StaticGrid {
  data: Vec<Cell>,
  bounds: Bounds,
}

impl StaticGrid {
  fn new(bounds: Bounds) -> Self {
    Self {
      data: vec![Cell::default(); bounds.width() * bounds.height()],
      bounds,
    }
  }
}

struct DynamicGrid {
  data: HashMap<Pos, Cell>,
  bounds: Bounds,
  floor_y: usize,
}

impl From<StaticGrid> for DynamicGrid {
  fn from(grid: StaticGrid) -> Self {
    let mut data = HashMap::new();

    for y in 0..grid.bounds.height() {
      for x in 0..grid.bounds.width() {
        let pos = pos!(x + grid.bounds.min_x, y + grid.bounds.min_y);
        let cell = grid.data[y * grid.bounds.width() + x];

        data.insert(pos, cell);
      }
    }

    let floor_y = grid.bounds.max_y + 2;
    let mut bounds = grid.bounds;
    bounds.update((bounds.max_x, floor_y));

    Self {
      data,
      bounds,
      floor_y,
    }
  }
}

#[derive(Debug)]
struct Bounds {
  min_x: usize,
  max_x: usize,
  min_y: usize,
  max_y: usize,
}

impl Bounds {
  fn new() -> Self {
    Self {
      min_x: usize::MAX,
      max_x: 0,
      min_y: usize::MAX,
      max_y: 0,
    }
  }

  fn update(&mut self, pos: impl Into<Pos>) {
    let pos = pos.into();
    let (x, y) = (pos.x, pos.y);
    if x > self.max_x {
      self.max_x = x;
    }
    if x < self.min_x {
      self.min_x = x;
    }
    if y > self.max_y {
      self.max_y = y;
    }
    if y < self.min_y {
      self.min_y = y;
    }
  }

  fn width(&self) -> usize {
    self.max_x.saturating_sub(self.min_x) + 1
  }

  fn height(&self) -> usize {
    self.max_y.saturating_sub(self.min_y) + 1
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
  x: usize,
  y: usize,
}

impl From<(usize, usize)> for Pos {
  fn from(value: (usize, usize)) -> Self {
    Self {
      x: value.0,
      y: value.1,
    }
  }
}

impl fmt::Display for StaticGrid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in 0..self.bounds.height() {
      for x in 0..self.bounds.width() {
        match self.data[y * self.bounds.width() + x] {
          Cell::Air => write!(f, ".")?,
          Cell::Rock => write!(f, "#")?,
          Cell::Sand => write!(f, "o")?,
        }
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl fmt::Display for DynamicGrid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let padding = 2;
    for y in (0..self.floor_y).map(|y| y + self.bounds.min_y) {
      for x in (0..self.bounds.width() + (padding * 2)).map(|x| x + self.bounds.min_x - padding) {
        match self.data.get(&pos!(x, y)) {
          Some(cell) => match cell {
            Cell::Air => write!(f, ".")?,
            Cell::Rock => write!(f, "#")?,
            Cell::Sand => write!(f, "o")?,
          },
          None => write!(f, ".")?,
        }
      }
      writeln!(f)?;
    }
    // floor
    for _ in 0..self.bounds.width() + (padding * 2) {
      write!(f, "#")?;
    }
    writeln!(f)?;

    Ok(())
  }
}
