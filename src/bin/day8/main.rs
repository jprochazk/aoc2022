use std::collections::HashSet;
use std::fmt;

fn main() {
  let input = include_str!("input.txt").trim();

  let rows = input.split('\n').count();
  let cols = input.split('\n').next().unwrap().chars().count();
  let mut grid = Grid::new(rows, cols);

  {
    for (y, row) in input.split('\n').enumerate() {
      for (x, height) in row.chars().enumerate() {
        grid.set(x, y, height.to_digit(10).unwrap() as u8);
      }
    }

    let mut scratch = vec![];
    let mut visible = HashSet::new();

    // for each row, find the highest tree, add it to the set of visible trees,
    // then find the next highest tree, add it, repeat until we hit the edge.
    // repeat the entire process in reverse.
    // also do all of the above for each column.
    //
    // the length of the `visible` set is equal to the number of trees
    // visible from any direction.

    // rows
    for y in 0..grid.rows() {
      // left <- right visible
      scratch.extend(grid.row(y));

      let mut skip_to = 0;
      loop {
        let mut highest = Tree::default();
        for (x, height) in scratch.iter().cloned().enumerate().skip(skip_to) {
          if height >= highest.height {
            highest = Tree { x, y, height };
          }
        }
        visible.insert((highest.x, highest.y));

        if highest.x == grid.cols() - 1 {
          break;
        }
        skip_to = highest.x + 1;
      }

      // left -> right visible
      let mut take_n = grid.cols();
      loop {
        let mut highest = Tree::default();
        for (x, height) in scratch.iter().cloned().enumerate().take(take_n).rev() {
          if height >= highest.height {
            highest = Tree { x, y, height };
          }
        }
        visible.insert((highest.x, highest.y));

        if highest.x == 0 {
          break;
        }
        take_n = highest.x;
      }

      scratch.clear();
    }

    // columns
    for x in (0..grid.cols()).rev() {
      // top <- bottom visible
      scratch.extend(grid.col(x));

      let mut skip_to = 0;
      loop {
        let mut highest = Tree::default();
        for (y, height) in scratch.iter().cloned().enumerate().skip(skip_to) {
          if height >= highest.height {
            highest = Tree { x, y, height };
          }
        }
        visible.insert((highest.x, highest.y));

        if highest.y == grid.cols() - 1 {
          break;
        }
        skip_to = highest.y + 1;
      }

      // top -> bottom visible
      let mut take_n = grid.cols();
      loop {
        let mut highest = Tree::default();
        for (y, height) in scratch.iter().cloned().enumerate().take(take_n).rev() {
          if height >= highest.height {
            highest = Tree { x, y, height };
          }
        }
        visible.insert((highest.x, highest.y));

        if highest.y == 0 {
          break;
        }
        take_n = highest.y;
      }

      scratch.clear();
    }

    println!("Day 8 part 1 answer: {}", visible.len());
  }

  {
    let mut scratch = vec![];
    // for each tree, measure its viewing distance along
    // the horizontal and vertical axes: +x, -x, +y, -y
    // multiply those viewing distances together to obtain
    // the scenic score. find the tree with the highest
    // scenic score.

    let mut highest_score = 0;
    for y in 0..grid.rows() {
      for x in 0..grid.cols() {
        let height = grid.get(x, y);

        scratch.extend(grid.row(y));
        // -x
        let px = scratch
          .iter()
          // from 0 to x, in reverse
          .take(x)
          .rev()
          // count the number of trees
          .enumerate()
          .map(|(n, height)| (n + 1, *height))
          // find the next tree with greater or equal height
          .find(|(_, h)| *h >= height)
          .map(|(n, _)| n)
          // if we find none, then the number of visible trees
          // is equal to the distance from the edge
          .unwrap_or(x);

        // +x
        let mx = scratch
          .iter()
          // from x+1 to end
          .skip(x + 1)
          // count the number of trees
          .enumerate()
          .map(|(n, height)| (n + 1, *height))
          // find the next tree with greater or equal height
          .find(|(_, h)| *h >= height)
          .map(|(n, _)| n)
          // if we find none, then the number of visible trees
          // is equal to the distance from the edge
          .unwrap_or(grid.cols() - (x + 1));
        scratch.clear();

        scratch.extend(grid.col(x));
        // -y
        let py = scratch
          .iter()
          // from 0 to x, in reverse
          .take(y)
          .rev()
          // count the number of trees
          .enumerate()
          .map(|(n, height)| (n + 1, *height))
          // find the next tree with greater or equal height
          .find(|(_, h)| *h >= height)
          .map(|(n, _)| n)
          // if we find none, then the number of visible trees
          // is equal to the distance from the edge
          .unwrap_or(y);
        // +y
        let my = scratch
          .iter()
          // from y+1 to end
          .skip(y + 1)
          // count the number of trees
          .enumerate()
          .map(|(n, height)| (n + 1, *height))
          // find the next tree with greater or equal height
          .find(|(_, h)| *h >= height)
          .map(|(n, _)| n)
          // if we find none, then the number of visible trees
          // is equal to the distance from the edge
          .unwrap_or(grid.rows() - (y + 1));
        scratch.clear();

        let score = px * mx * py * my;
        if score > highest_score {
          highest_score = score;
        }
      }
    }

    println!("Day 8 part 2 answer: {}", highest_score);
  }
}

#[derive(Default)]
struct Tree {
  x: usize,
  y: usize,
  height: u8,
}

struct Grid {
  data: Vec<u8>,
  cols: usize,
}

impl Grid {
  fn new(rows: usize, cols: usize) -> Self {
    Self {
      data: vec![0; rows * cols],
      cols,
    }
  }

  fn set(&mut self, x: usize, y: usize, v: u8) {
    self.data[y * self.cols + x] = v;
  }

  fn get(&self, x: usize, y: usize) -> u8 {
    self.data[y * self.cols + x]
  }

  fn rows(&self) -> usize {
    self.data.len() / self.cols
  }

  fn cols(&self) -> usize {
    self.cols
  }

  fn row(&self, y: usize) -> Row {
    Row {
      grid: self,
      y,
      cx: 0,
    }
  }

  fn col(&self, x: usize) -> Column {
    Column {
      grid: self,
      x,
      cy: 0,
    }
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in 0..self.data.len() / self.cols {
      for x in 0..self.cols {
        write!(f, "{}", self.get(x, y))?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

struct Row<'a> {
  grid: &'a Grid,
  y: usize,
  cx: usize,
}

impl<'a> Iterator for Row<'a> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cx >= self.grid.cols() {
      return None;
    }

    let x = {
      let temp = self.cx;
      self.cx += 1;
      temp
    };
    let y = self.y;

    Some(self.grid.data[y * self.grid.cols() + x])
  }
}

struct Column<'a> {
  grid: &'a Grid,
  x: usize,
  cy: usize,
}

impl<'a> Iterator for Column<'a> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cy >= self.grid.rows() {
      return None;
    }

    let y = {
      let temp = self.cy;
      self.cy += 1;
      temp
    };
    let x = self.x;

    Some(self.grid.data[y * self.grid.cols() + x])
  }
}
