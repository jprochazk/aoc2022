use itertools::Itertools;
use std::mem::MaybeUninit;

fn main() {
  let input = include_str!("input.txt").trim();

  let mut total_priority = 0;
  for rucksack in input.split('\n') {
    let (first, second) = rucksack.split_at(rucksack.len() / 2);

    let mut common = MaybeUninit::<char>::uninit();
    for item in first.chars() {
      if second.contains(item) {
        common.write(item);
        break;
      }
    }
    // input always contains exactly one common item type in both compartments
    let common = unsafe { common.assume_init() };

    total_priority += priority(common);
  }
  println!("Day 3 part 1 answer: {}", total_priority);

  let mut total_priority = 0;
  for group in input.split('\n').chunks(3).into_iter() {
    let mut parts = take::<3, _>(group).unwrap();
    parts.sort_by_key(|v| usize::MAX - v.len());

    let mut common = MaybeUninit::<char>::uninit();
    let [a, b, c] = parts;
    for item in a.chars() {
      if b.contains(item) && c.contains(item) {
        common.write(item);
        break;
      }
    }
    // input always contains exactly one common item type in all three compartments
    let common = unsafe { common.assume_init() };

    total_priority += priority(common);
  }
  println!("Day 3 part 2 answer: {}", total_priority);
}

fn priority(item: char) -> u64 {
  match item {
    // a..z -> 1..26
    'a'..='z' => (item as u8 - b'a' + 1) as u64,
    // A..Z -> 27..52
    'A'..='Z' => (item as u8 - b'A' + 27) as u64,
    _ => panic!("invalid item type: {item}"),
  }
}

fn take<const N: usize, I: Iterator>(mut iter: I) -> Option<[I::Item; N]> {
  let mut uninit: [MaybeUninit<I::Item>; N] = unsafe { MaybeUninit::uninit().assume_init() };

  for elem in &mut uninit {
    unsafe { std::ptr::write(elem.as_mut_ptr(), iter.next()?) };
  }

  let array = unsafe { std::ptr::read(&uninit as *const _ as *const [I::Item; N]) };
  std::mem::forget(uninit);

  Some(array)
}
