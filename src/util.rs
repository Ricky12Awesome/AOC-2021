#![allow(unused)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::num::{ParseFloatError, ParseIntError};
use std::ops::Sub;
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

pub trait Array2DIndex<T> {
  fn get_at(&self, y: usize, x: usize) -> Option<T>;

  fn get_at_offset(&self, y: usize, x: usize, off_y: isize, off_x: isize) -> Option<([usize; 2], T)> {
    let y = ((y as isize) + off_y).try_into().ok()?;
    let x = ((x as isize) + off_x).try_into().ok()?;

    self.get_at(y, x).map(|it| ([y, x], it))
  }

  fn get_adjacent_at(&self, y: usize, x: usize) -> [Option<([usize; 2], T)>; 8] {
    [
      self.get_at_offset(y, x, 1, 1),
      self.get_at_offset(y, x, 1, 0),
      self.get_at_offset(y, x, 0, 1),
      self.get_at_offset(y, x, 1, -1),
      self.get_at_offset(y, x, -1, 1),
      self.get_at_offset(y, x, 0, -1),
      self.get_at_offset(y, x, -1, 0),
      self.get_at_offset(y, x, -1, -1),
    ]
  }
}

impl<I: Iterator> UnwrapNext for I {}

impl<T: Debug, I: Iterator<Item = T>> CollectArr<T> for I {}

impl<K: Hash + Eq, V> HashmapGetOr<K, V> for HashMap<K, V> {
  fn get_or<'a>(&'a self, key: &'a K, val: &'a V) -> &'a V {
    self.get(key).unwrap_or(val)
  }
}

impl<K: Hash + Eq + Copy, V: Default> HashmapGetOrDefMut<K, V> for HashMap<K, V> {
  fn get_or_def_mut(&mut self, key: K) -> &mut V {
    self.entry(key).or_insert_with(|| V::default())
  }
}

impl<T: Sized + Copy, const Y: usize, const X: usize> Array2DIndex<T> for [[T; Y]; X] {
  fn get_at(&self, y: usize, x: usize) -> Option<T> {
    self.get(y).and_then(|it| it.get(x).copied())
  }
}

impl <T: Sized + Copy> Array2DIndex<T> for HashMap<[usize; 2], T> {
  fn get_at(&self, y: usize, x: usize) -> Option<T> {
    self.get(&[y, x]).copied()
  }
}