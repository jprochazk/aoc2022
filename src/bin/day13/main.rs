use core::fmt;
use std::mem::discriminant;
use std::ops::Range;

use itertools::Itertools;
use logos::Logos;
use thiserror::Error;

fn main() {
  let input = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    .trim();
  let input = include_str!("input.txt").trim();

  parse("[[[],[[],9]],[5,5,[4,[7],4]],[[]],[[6,[],[0,2,1],[0,0,7],[5,4]]]]").unwrap();

  aoc::time(
    || {
      input
        .split("\n\n")
        .map(|pair| {
          let (a, b) = pair.split_once('\n').unwrap();
          let (a, b) = (a.trim(), b.trim());
          let (a, b) = (parse(a).unwrap(), parse(b).unwrap());
          (a, b)
        })
        .positions(|(a, b)| a < b)
        .map(|i| i + 1)
        .sum::<usize>()
    },
    |sum_indices| {
      println!("Day 13 part 1 answer: {}", sum_indices);
    },
  );

  aoc::time(
    || {
      let dividers = [parse("[[2]]").unwrap(), parse("[[6]]").unwrap()];

      let mut pos = [1, 2];

      for packet in input.split("\n\n").flat_map(|pair| {
        let (a, b) = pair.split_once('\n').unwrap();
        let (a, b) = (a.trim(), b.trim());
        let (a, b) = (parse(a).unwrap(), parse(b).unwrap());
        [a, b]
      }) {
        if packet < dividers[0] {
          pos[0] += 1;
          pos[1] += 1;
        } else if packet < dividers[1] {
          pos[1] += 1;
        }
      }

      pos[0] * pos[1]
    },
    |key| {
      println!("Day 13 part 2 answer: {}", key);
    },
  )
}

#[derive(Clone, PartialEq, Eq)]
pub enum Packet {
  List(Vec<Packet>),
  Int(u64),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match (self, other) {
      (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
      (Packet::List(a), Packet::List(b)) => a.cmp(b),
      (Packet::Int(a), Packet::List(b)) => [Packet::Int(*a)][..].cmp(&b[..]),
      (Packet::List(a), Packet::Int(b)) => a[..].cmp(&[Packet::Int(*b)]),
    }
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

fn parse(s: &str) -> Result<Packet, Error> {
  fn parse_list(lex: &mut Lexer) -> Result<Packet, Error> {
    use TokenKind::*;

    let mut items = vec![];

    lex.expect(BracketL)?;
    if lex.bump_if(BracketR)? {
      return Ok(Packet::List(items));
    }
    items.push(parse_packet(lex)?);
    while lex.bump_if(Comma)? {
      items.push(parse_packet(lex)?)
    }
    lex.expect(BracketR)?;

    Ok(Packet::List(items))
  }
  fn parse_int(lex: &mut Lexer) -> Result<Packet, Error> {
    use TokenKind::*;

    let token = lex.expect(Int(0))?;
    let Int(n) = token.kind else { unreachable!() };
    Ok(Packet::Int(n))
  }
  fn parse_packet(lex: &mut Lexer) -> Result<Packet, Error> {
    use TokenKind::*;

    if lex.current().is(BracketL) {
      parse_list(lex)
    } else {
      parse_int(lex)
    }
  }

  let mut lexer = Lexer::new(s);
  lexer.bump()?;
  parse_list(&mut lexer)
}

struct Lexer<'a> {
  inner: logos::Lexer<'a, TokenKind>,
  previous: Token,
  current: Token,
  eof: Token,
}

impl<'a> Lexer<'a> {
  fn new(source: &'a str) -> Self {
    let end = source.len();
    let eof = Token {
      span: (end..end).into(),
      kind: TokenKind::Eof,
    };

    Self {
      inner: TokenKind::lexer(source),
      previous: eof,
      current: eof,
      eof,
    }
  }

  #[inline]
  fn expect(&mut self, which: TokenKind) -> Result<&Token, Error> {
    if self.current().is(which) {
      Ok(self.bump()?)
    } else {
      Err(Error::Expected(
        which,
        self.current().kind,
        self.current().span,
      ))
    }
  }

  #[inline]
  fn bump_if(&mut self, which: TokenKind) -> Result<bool, Error> {
    if self.current().is(which) {
      self.bump()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  #[inline]
  fn current(&self) -> &Token {
    &self.current
  }

  #[inline]
  fn bump(&mut self) -> Result<&Token, Error> {
    std::mem::swap(&mut self.previous, &mut self.current);
    self.current = match self.inner.next() {
      Some(kind) if kind == TokenKind::Bad => {
        return Err(Error::Invalid(
          self.inner.slice().to_owned(),
          self.inner.span().into(),
        ))
      }
      Some(kind) => Token {
        span: self.inner.span().into(),
        kind,
      },
      None => self.eof,
    };
    Ok(&self.previous)
  }
}

#[derive(Clone, Copy, Debug)]
struct Token {
  span: Span,
  kind: TokenKind,
}

impl Token {
  #[inline]
  fn is(&self, other: TokenKind) -> bool {
    discriminant(&self.kind) == discriminant(&other)
  }
}

#[derive(Clone, Copy, Debug, Logos, PartialEq, Eq)]
enum TokenKind {
  #[token("[")]
  BracketL,
  #[token("]")]
  BracketR,
  #[token(",")]
  Comma,
  #[regex(r"\d+", |l| l.slice().parse())]
  Int(u64),

  #[regex(r"[\s\n\t ]+", logos::skip)]
  Whitespace,
  Eof,
  #[error]
  Bad,
}

#[derive(Clone, Debug, Error)]
enum Error {
  #[error(r"expected token `{0}` found `{1}` at {2}")]
  Expected(TokenKind, TokenKind, Span),
  #[error(r"invalid token `{0}`")]
  Invalid(String, Span),
}

#[derive(Clone, Copy, Debug)]
struct Span {
  start: usize,
  end: usize,
}

impl From<Range<usize>> for Span {
  fn from(value: Range<usize>) -> Self {
    Span {
      start: value.start,
      end: value.end,
    }
  }
}

impl fmt::Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TokenKind::BracketL => "[",
        TokenKind::BracketR => "]",
        TokenKind::Comma => ",",
        TokenKind::Int(_) => "{integer}",
        TokenKind::Whitespace => "{whitespace}",
        TokenKind::Eof => "{eof}",
        TokenKind::Bad => "{invalid}",
      }
    )
  }
}

impl fmt::Display for Packet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Packet::List(list) => {
        write!(f, "[")?;
        let mut items = list.iter().peekable();
        while let Some(item) = items.next() {
          write!(f, "{item}")?;
          if items.peek().is_some() {
            write!(f, ",")?;
          }
        }
        write!(f, "]")
      }
      Packet::Int(int) => write!(f, "{int}"),
    }
  }
}
