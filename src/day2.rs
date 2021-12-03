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
        "up" => [0, -amount],
        _ => unreachable!(),
      })
      .collect::<Vec<_>>();

    let part1 = || {
      let [hor, dep] = lines
        .iter()
        .fold([0, 0], |[x1, y1], [x2, y2]| [x1 + x2, y1 + y2]);
      hor * dep
    };

    let part2 = || {
      let [hor, dep, _] = lines.iter().fold([0, 0, 0], |[x1, y1, z1], [x2, y2]| {
        [x1 + x2, y1 + x2 * z1, z1 + y2]
      });
      hor * dep
    };

    answer!(part, part1, part2)
  }
}
