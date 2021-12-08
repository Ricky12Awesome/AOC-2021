use crate::day::*;

day!(Day8, Some(294), Some(973292));

#[allow(unused)]
impl Day8 {
  fn day(part: Part) -> Answer<usize> {
    let segments = Self::INPUT.lines().map(|it| {
      it.split(" | ")
        .map(str::split_whitespace)
        .map(|it| it.map(|str| str.chars().fold(0u8, |n, c| n | 1 << (c as u8 - b'a'))))
        .map(|it| it.collect_vec())
        .collect_arr::<2>()
    });

    let (mut p1, mut p2) = (0, 0);

    for [unique, output] in segments {
      let one = unique.iter().find(|it| it.count_ones() == 2).unwrap();
      let four = unique.iter().find(|it| it.count_ones() == 4).unwrap();

      p1 += output.iter().filter(|x| [2, 3, 4, 7].contains(&x.count_ones())).count();
      p2 += output
        .iter()
        .map(
          |it| match (it.count_ones(), (it & four).count_ones(), (it & one).count_ones()) {
            (2, _, _) => 1,
            (4, _, _) => 4,
            (3, _, _) => 7,
            (7, _, _) => 8,

            (5, 2, _) => 2,
            (5, 3, 1) => 5,
            (5, 3, 2) => 3,

            (6, 4, _) => 9,
            (6, 3, 1) => 6,
            (6, 3, 2) => 0,
            _ => unreachable!(),
          },
        )
        .rev()
        .zip(std::iter::successors(Some(1usize), |n| n.checked_mul(10)))
        .map(|(digit, n)| digit * n)
        .sum::<usize>();
    }

    answer!(part, || p1, || p2)
  }
}
