use crate::day::*;

day!(Day01, Some(1655), Some(1683));

impl Day01 {
  fn day(part: Part) -> Answer<usize> {
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
