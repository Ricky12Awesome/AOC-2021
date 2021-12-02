use crate::day::*;

day!(Day1);

impl Day for Day1 {
  fn day(part: Part) -> Answer {
    let input = Self::INPUT
      .lines()
      .map(|it| it.parse::<u32>().unwrap())
      .collect::<Vec<_>>();

    let count = |input: &[u32]| {
      input
        .array_windows::<2>()
        .filter(|nums| nums[0] < nums[1])
        .count()
    };

    let part1 = || count(&input);
    let part2 = || {
      let input = input
        .array_windows::<3>()
        .map(|it| it.iter().sum())
        .collect::<Vec<u32>>();

      count(&input)
    };

    answer!(part, part1, part2)
  }
}
