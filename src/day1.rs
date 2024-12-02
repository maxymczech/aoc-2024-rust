use aoc_runner_derive::{aoc};

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
  let s = input.split_whitespace();
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();

  for (i, str) in s.enumerate() {
    let num: i32 = str.parse().expect("Not a valid number");
    if i % 2 == 0 {
      left.push(num);
    } else {
      right.push(num);
    }
  }

  left.sort();
  right.sort();

  let mut result = 0;
  for (i, l) in left.iter().enumerate() {
    result += (right[i] - l).abs();
  }
  return result;
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
  let s = input.split_whitespace();
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();

  let mut max = 0;
  for (i, str) in s.enumerate() {
    let num: i32 = str.parse().expect("Not a valid number");
    if num > max {
      max = num;
    }
    if i % 2 == 0 {
      left.push(num);
    } else {
      right.push(num);
    }
  }

  let mut result = 0;
  let mut count = vec![0; (max.abs() + 1) as usize];
  for r in right {
    count[r.abs() as usize] += 1;
  }
  for l in left {
    result += l * count[l.abs() as usize];
  }

  return result;
}

mod tests {
}
