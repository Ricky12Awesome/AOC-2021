#![allow(unused)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

pub use itertools::Itertools;

pub const fn not_empty(str: &&str) -> bool {
  !str.is_empty()
}

pub fn parse_char<T: FromStr<Err = ParseIntError>>(c: char) -> T {
  T::from_str(&c.to_string()).unwrap()
}
pub fn parse_int<T: FromStr<Err = ParseIntError>>(str: &str) -> T {
  T::from_str(str).unwrap()
}

pub fn parse_float<T: FromStr<Err = ParseFloatError>>(str: &str) -> T {
  T::from_str(str).unwrap()
}

pub trait CollectArr<T: Debug>: Iterator<Item = T> + Sized {
  fn collect_arr<const N: usize>(self) -> [T; N] {
    self.collect_vec().try_into().unwrap()
  }
}

pub trait UnwrapNext: Iterator {
  fn unwrap_next(&mut self) -> Self::Item {
    self.next().unwrap()
  }
}

pub trait HashmapGetOr<K, V> {
  fn get_or<'a>(&'a self, key: &'a K, val: &'a V) -> &'a V;
}

pub trait HashmapGetOrDefMut<K, V> {
  fn get_or_def_mut(&mut self, key: K) -> &mut V;
}

impl<I: Iterator> UnwrapNext for I {}

impl<T: Debug, I: Iterator<Item = T>> CollectArr<T> for I {}

impl <K: Hash + Eq, V> HashmapGetOr<K, V> for HashMap<K, V> {
  fn get_or<'a>(&'a self, key: &'a K, val: &'a V) -> &'a V {
    self.get(key).unwrap_or(val)
  }
}

impl <K: Hash + Eq + Copy, V: Default> HashmapGetOrDefMut<K,V> for HashMap<K, V> {
  fn get_or_def_mut(&mut self, key: K) -> &mut V {
    self.entry(key).or_insert_with(|| V::default())
  }
}