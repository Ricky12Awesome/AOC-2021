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
          _ => [0, 0],
        }
      })
      .collect::<Vec<_>>();

    let part1 = || {
      let [hor, dep] = lines
        .clone()
        .into_iter()
        .reduce(|[hor, dep], [x, y]| [hor + x, dep + y])
        .unwrap();

      hor * dep
    };

    let part2 = || {
      let [hor, dep, _] = lines
        .clone()
        .into_iter()
        .fold([0, 0, 0], |[hor, dep, aim], [x, y]| {
          [hor + x, dep + x * aim, aim + y]
        });

      hor * dep
    };

    answer!(part, part1, part2)
  }
}
