fn main() {
  let input = include_str!("input.txt").trim();

  let n = 4;
  for (i, s) in Window::new(input, n).enumerate() {
    if has_only_unique_chars(s) {
      println!("Day 6 part 1 answer: ({}) {}", s, i + n);
      break;
    }
  }

  let n = 14;
  for (i, s) in Window::new(input, n).enumerate() {
    if has_only_unique_chars(s) {
      println!("Day 6 part 2 answer: ({}) {}", s, i + n);
      break;
    }
  }
}

struct Window<'a> {
  s: &'a str,
  n: usize,
  i: usize,
}

impl<'a> Window<'a> {
  fn new(s: &'a str, n: usize) -> Window<'a> {
    Window { s, n, i: 0 }
  }
}

impl<'a> Iterator for Window<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    let (s, len, i) = (self.s, self.n, self.i);

    if s.len() - i < len {
      return None;
    }

    let r = Some(&s[i..i + len]);
    self.i += 1;
    r
  }
}

fn has_only_unique_chars(s: &str) -> bool {
  if s.len() < 2 {
    return true;
  }

  let (mut needle, mut remainder) = s.split_at(1);
  while !remainder.is_empty() {
    if remainder.contains(needle) {
      return false;
    }
    (needle, remainder) = remainder.split_at(1);
  }

  true
}
