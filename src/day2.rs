use aoc_runner_derive::{aoc};

const N: usize = 20;
const LF: u8 = 0x0A;
const CR: u8 = 0x0D;
const SP: u8 = 0x20;

fn is_report_ok(report: &[u8; N], count: usize) -> bool {
  let mut ok = false;
  if report[1] > report[0] {
    // Inc
    ok = true;
    for i in 1..count {
      if report[i] <= report[i - 1] {
        ok = false;
        break;
      }
      let diff = report[i] - report[i - 1];
      if diff < 1 || diff > 3 {
        ok = false;
        break;
      }
    }
  }
  if report[1] < report[0] {
    // Dec
    ok = true;
    for i in 1..count {
      if report[i] >= report[i - 1] {
        ok = false;
        break;
      }
      let diff = report[i - 1] - report[i];
      if diff < 1 || diff > 3 {
        ok = false;
        break;
      }
    }
  }

  return ok;
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
  let s = input.as_bytes();

  let mut result = 0;
  let mut report = [0; N];
  let mut count: usize = 0;
  let mut cur: u8 = 0;
  
  for c in s.iter() {
    if c.eq(&CR) {
    } else if c.eq(&LF) {
      // End of line - solve for report
      if count == 0 {
        break;
      }
      report[count] = cur;
      count += 1;

      if is_report_ok(&report, count) {
        result += 1;
      }

      count = 0;
      cur = 0;
    } else if c.eq(&SP) {
      report[count] = cur;
      count += 1;
      cur = 0;
    } else {
      // cur = cur * 10 + u32::from(*c) - 48;
      cur = cur * 10 + c - 48;
    }
  }

  return result;
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
  let s = input.as_bytes();

  let mut result = 0;
  let mut report = [0; N];
  let mut count: usize = 0;
  let mut cur: u8 = 0;
  
  for c in s.iter() {
    if c.eq(&CR) {
    } else if c.eq(&LF) {
      // End of line - solve for report
      if count == 0 {
        break;
      }
      report[count] = cur;
      count += 1;

      if is_report_ok(&report, count) {
        result += 1;
      } else {
        let mut report_omitted = [0; N];
        for omit in 0..count {
          let mut count2 = 0;
          for i in 0..count {
            if i != omit {
              report_omitted[count2] = report[i];
              count2 += 1
            }
          }
          if is_report_ok(&report_omitted, count2) {
            result += 1;
            break;
          }
        }
      }

      count = 0;
      cur = 0;
    } else if c.eq(&SP) {
      report[count] = cur;
      count += 1;
      cur = 0;
    } else {
      cur = cur * 10 + c - 48;
    }
  }

  return result;
}
