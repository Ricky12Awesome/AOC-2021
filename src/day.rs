pub use std::fmt::Debug;

#[macro_export]
macro_rules! answer {
  ($answer:expr) => {
    Some(Box::new($answer))
  };
  ($part:expr, $p1:expr, $p2:expr) => {
    match $part {
      Part::One => [answer!($p1()), None],
      Part::Two => [None, answer!($p2())],
      Part::Both => [answer!($p1()), answer!($p2())],
    }
  };
}

#[macro_export]
macro_rules! day {
    ($name:ident) => {
      pub struct $name;

      impl Input for $name {
        const INPUT: &'static str = include_str!(concat!("../assets/", stringify!($name)));
      }

      generate_tests_for_day!($name);
    };
}

#[macro_export]
macro_rules! generate_tests_for_day {
  ($day:ident) => {
    #[test]
    pub fn both() {
      let answer = $day::day(Part::Both);
      let day = stringify!($day).trim_start_matches("Day");

      println!("Day {day} Part 1: {:?}", answer[0]);
      println!("Day {day} Part 2: {:?}", answer[1]);
    }

    #[test]
    pub fn part1() {
      let answer = $day::day(Part::One);
      let day = stringify!($day).trim_start_matches("Day");

      println!("Day {day} Part 1: {:?}", answer[0]);
    }

    #[test]
    pub fn part2() {
      let answer = $day::day(Part::Two);
      let day = stringify!($day).trim_start_matches("Day");

      println!("Day {day} Part 2: {:?}", answer[1]);
    }
  };
}

#[allow(dead_code)]
pub enum Part {
  One,
  Two,
  Both,
}

#[allow(dead_code)]
pub type Answer = [Option<Box<dyn Debug>>; 2];

pub trait Input {
  const INPUT: &'static str = "";
}

pub trait Day: Input {
  fn day(part: Part) -> Answer;
}
