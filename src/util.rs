pub use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

pub const fn not_empty(str: &&str) -> bool {
  !str.is_empty()
}

pub fn parse<T: FromStr<Err = ParseIntError>>(str: &str) -> T {
  T::from_str(str).unwrap()
}

pub trait UnwrapNext: Iterator {
  fn unwrap_next(&mut self) -> Self::Item {
    self.next().unwrap()
  }
}

impl<I: Iterator> UnwrapNext for I {}
