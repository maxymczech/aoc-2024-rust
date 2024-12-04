use aoc_runner_derive::{aoc};

const LD: u8 = 0x64;
const LL: u8 = 0x6c;
const LM: u8 = 0x6d;
const LN: u8 = 0x6e;
const LO: u8 = 0x6f;
const LT: u8 = 0x74;
const LU: u8 = 0x75;
const PO: u8 = 0x28;
const PC: u8 = 0x29;
const N0: u8 = 0x30;
const N9: u8 = 0x39;
const APOS: u8 = 0x27;
const COMA: u8 = 0x2C;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
  let s = input.as_bytes();

  let mut result = 0;
  let mut state = 0;
  let mut n1: u32 = 0;
  let mut n2: u32 = 0;

  for c in s.iter() {
    if (state == 0) && c.eq(&LM) {
      state = 1;
    }
    else if state == 1 {
      if c.eq(&LU) { state = 2; }
      else { state = 0; }
    }
    else if state == 2 {
      if c.eq(&LL) { state = 3; }
      else { state = 0; }
    }
    else if state == 3 {
      if c.eq(&PO) {
        state = 4;
        n1 = 0;
        n2 = 0;
      }
      else { state = 0; }
    }
    else if state == 4 {
      if c.eq(&COMA) {
        if (n1 > 0) && (n1 < 1000) { state = 5; }
        else { state = 0; }
      } else if (c.ge(&N0)) && (c.le(&N9)) {
        n1 = n1 * 10 + (*c as u32) - 48;
      } else { state = 0; }
    }
    else if state == 5 {
      if c.eq(&PC) {
        if (n2 > 0) && (n2 < 1000) {
          result += n1 * n2;
        }
        state = 0;
      } else if (c.ge(&N0)) && (c.le(&N9)) {
        n2 = n2 * 10 + (*c as u32) - 48;
      } else { state = 0; }
    }
  }

  // println!("Part 1: {}", result);
  return result;
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
  let s = input.as_bytes();

  let mut result = 0;
  let mut state = 0;
  let mut n1: u32 = 0;
  let mut n2: u32 = 0;
  let mut do_it = true;

  for c in s.iter() {
    if state == 0 {
      if do_it && c.eq(&LM) { state = 1; }
      else if c.eq(&LD) { state = 10; }
    }
    else if state == 1 {
      if c.eq(&LU) { state = 2; }
      else { state = 0; }
    }
    else if state == 2 {
      if c.eq(&LL) { state = 3; }
      else { state = 0; }
    }
    else if state == 3 {
      if c.eq(&PO) {
        state = 4;
        n1 = 0;
        n2 = 0;
      }
      else { state = 0; }
    }
    else if state == 4 {
      if c.eq(&COMA) {
        if (n1 > 0) && (n1 < 1000) { state = 5; }
        else { state = 0; }
      } else if (c.ge(&N0)) && (c.le(&N9)) {
        n1 = n1 * 10 + (*c as u32) - 48;
      } else { state = 0; }
    }
    else if state == 5 {
      if c.eq(&PC) {
        if (n2 > 0) && (n2 < 1000) {
          result += n1 * n2;
        }
        state = 0;
      } else if (c.ge(&N0)) && (c.le(&N9)) {
        n2 = n2 * 10 + (*c as u32) - 48;
      } else { state = 0; }
    }

    else if state == 10 {
      if c.eq(&LO) { state = 11; }
      else { state = 0; }
    }
    else if state == 11 {
      if c.eq(&PO) { state = 12; }
      else if c.eq(&LN) { state = 13; }
      else { state = 0; }
    }
    else if state == 12 {
      if c.eq(&PC) {
        do_it = true;
      }
      state = 0;
    }
    else if state == 13 {
      if c.eq(&APOS) { state = 14; }
      else { state = 0; }
    }
    else if state == 14 {
      if c.eq(&LT) { state = 15; }
      else { state = 0; }
    }
    else if state == 15 {
      if c.eq(&PO) { state = 16; }
      else { state = 0; }
    }
    else if state == 16 {
      if c.eq(&PC) {
        do_it = false;
      }
      state = 0;
    }
  }

  // println!("Part 2: {}", result);
  return result;
}
