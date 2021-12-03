use crate::day::*;

day!(Day2);

impl Day for Day2 {
  fn day(part: Part) -> Answer {
    let lines = Self::INPUT
      .lines()
      .map(|line| line.split_once(" ").unwrap())
      .map(|(pos, amount)| (pos, amount.parse::<i32>().unwrap()))
      .map(|(pos, amount)| match pos {
        "forward" => [amount, 0],
        "down" => [0, amount],
        "up"=> [0, -amount],
        _ => [0, 0],
      })
      .collect::<Vec<_>>();

    let part1 = || {
      let [hor, dep] = lines
        .iter()
        .fold([0, 0], |[hor, dep], [x, y]| [hor + x, dep + y]);

      hor * dep
    };

    let part2 = || {
      let [hor, dep, _] = lines
        .iter() // rustfmt likes putting this on one long line for some reason, this is here to stop it
        .fold([0, 0, 0], |[hor, dep, aim], [x, y]| {
          [hor + x, dep + x * aim, aim + y]
        });

      hor * dep
    };

    answer!(part, part1, part2)
  }
}
