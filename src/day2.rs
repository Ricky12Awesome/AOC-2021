use crate::day::*;

day!(Day2);

impl Day for Day2 {
  fn day(part: Part) -> Answer {
    let lines = Self::INPUT
      .lines()
      .map(|it| {
        let (pos, amount) = it.split_once(" ").unwrap();
        let amount = amount.parse::<i32>().unwrap();

        match pos {
          "forward" => [amount, 0],
          "down" => [0, amount],
          "up" => [0, -amount],
          _ => [0, 0]
        }
      });

    let lines = lines.collect::<Vec<_>>();

    let part1 = || {
      let mut hor = 0;
      let mut dep = 0;

      for [x, y] in lines.clone() {
        hor += x;
        dep += y;
      }

      hor * dep
    };


    let lines = lines.clone();
    let part2 = || {
      let mut hor = 0;
      let mut dep = 0;
      let mut aim = 0;

      for [x, y] in lines {
        hor += x;
        aim += y;
        dep += x * aim;
      }

      hor * dep
    };

    answer!(part, part1, part2)
  }
}