use std::collections::VecDeque;

fn main() {
  let input = include_str!("input.txt").trim();

  aoc::time(
    || {
      let grid = Grid::parse(input);
      bfs(&grid, grid.start, Cell::End, step_up).unwrap()
    },
    |shortest_path| {
      println!("Day 12 part 1 answer: {}", shortest_path);
    },
  );

  aoc::time(
    || {
      let grid = Grid::parse(input);
      bfs(&grid, grid.end, Cell::N(0), step_down).unwrap()
    },
    |shortest_path| {
      println!("Day 12 part 2 answer: {}", shortest_path);
    },
  );
}

fn step_up(grid: &Grid, from: Node, to: Node) -> bool {
  grid.height(to) - grid.height(from) <= 1
}

fn step_down(grid: &Grid, from: Node, to: Node) -> bool {
  grid.height(from) - grid.height(to) <= 1
}

fn bfs(
  grid: &Grid,
  start: Node,
  end: Cell,
  can_access: impl Fn(&Grid, Node, Node) -> bool,
) -> Option<usize> {
  let mut q: VecDeque<Node> = [start].into_iter().collect();
  let mut v = Visits::new(start, grid.rows(), grid.cols());

  while let Some(node) = q.pop_front() {
    if grid.at(node) == Some(end) {
      return Some(node.dist);
    }

    for neighbor in node.neighbors(grid) {
      if grid.at(neighbor).is_some() && !v.visited(neighbor) && can_access(grid, node, neighbor) {
        v.visit(neighbor);
        q.push_back(neighbor);
      }
    }
  }

  None
}

struct Grid {
  cols: usize,
  data: Vec<Cell>,
  start: Node,
  end: Node,
}

impl Grid {
  fn parse(s: &str) -> Self {
    let mut cols = 0;
    let mut data = vec![];
    let mut start = Node {
      y: 0,
      x: 0,
      dist: 0,
    };
    let mut end = Node {
      y: 0,
      x: 0,
      dist: 0,
    };
    for (y, line) in s.split('\n').map(str::trim).enumerate() {
      for (x, cell) in line.chars().enumerate() {
        match cell {
          'S' => {
            start.y = y;
            start.x = x;
            data.push(Cell::N(0));
          }
          'E' => {
            end.y = y;
            end.x = x;
            data.push(Cell::End);
          }
          n => data.push(Cell::N((n as u8 - b'a') as i64)),
        }
        cols = x + 1;
      }
    }

    Self {
      cols,
      data,
      start,
      end,
    }
  }

  fn at(&self, node: Node) -> Option<Cell> {
    self.data.get(node._1d(self.cols)).cloned()
  }

  fn height(&self, node: Node) -> i64 {
    match self.at(node) {
      Some(Cell::End) => (b'z' - b'a') as i64,
      Some(Cell::N(n)) => n,
      None => i64::MAX,
    }
  }

  fn rows(&self) -> usize {
    self.data.len() / self.cols
  }

  fn cols(&self) -> usize {
    self.cols
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
  End,
  N(i64),
}

#[derive(Clone, Copy, Debug)]
struct Node {
  x: usize,
  y: usize,
  dist: usize,
}

impl Node {
  fn _1d(&self, cols: usize) -> usize {
    self.y * cols + self.x
  }

  fn neighbors(&self, grid: &Grid) -> impl Iterator<Item = Node> {
    [
      self.up(),
      self.down(grid.rows()),
      self.left(),
      self.right(grid.cols()),
    ]
    .into_iter()
    .flatten()
  }

  fn up(&self) -> Option<Node> {
    if self.y > 0 {
      Some(Node {
        x: self.x,
        y: self.y - 1,
        dist: self.dist + 1,
      })
    } else {
      None
    }
  }

  fn down(&self, rows: usize) -> Option<Node> {
    if self.y + 1 < rows {
      Some(Node {
        x: self.x,
        y: self.y + 1,
        dist: self.dist + 1,
      })
    } else {
      None
    }
  }

  fn left(&self) -> Option<Node> {
    if self.x > 0 {
      Some(Node {
        x: self.x - 1,
        y: self.y,
        dist: self.dist + 1,
      })
    } else {
      None
    }
  }

  fn right(&self, cols: usize) -> Option<Node> {
    if self.x + 1 < cols {
      Some(Node {
        x: self.x + 1,
        y: self.y,
        dist: self.dist + 1,
      })
    } else {
      None
    }
  }
}

struct Visits {
  cols: usize,
  v: Vec<bool>,
}

impl Visits {
  fn new(start: Node, rows: usize, cols: usize) -> Self {
    let v = vec![false; rows * cols];
    let mut v = Self { cols, v };
    v.visit(start);
    v
  }

  fn visited(&self, node: Node) -> bool {
    self.v[node._1d(self.cols)]
  }

  fn visit(&mut self, node: Node) {
    self.v[node._1d(self.cols)] = true;
  }
}
