use std::collections::BinaryHeap;

fn main() {
  let input = include_str!("day1.txt").trim();

  let mut max_calories = BinaryHeap::new();
  for elf in input.split("\n\n") {
    let mut sum = 0;
    for food_item in elf.split('\n') {
      sum += food_item.parse::<u64>().unwrap();
    }
    max_calories.push(sum);
  }
  println!("Day 1 part 1 answer: {}", max_calories.peek().unwrap());

  let mut top_three_total = 0;
  for elf in max_calories.into_sorted_vec().into_iter().rev().take(3) {
    top_three_total += elf;
  }
  println!("Day 1 part 2 answer: {}", top_three_total);
}
