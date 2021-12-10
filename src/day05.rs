use crate::day::*;

day!(Day05, Some(6225), Some(22116));

impl Day05 {
  #[rustfmt::skip]
  #[allow(clippy::comparison_chain)]
  fn day(part: Part) -> Answer<usize> {
    let points = Self::INPUT.lines().map(|line| {
      line
        .split(" -> ")
        .map(|it| it.split(',').map(parse_int::<usize>).collect_arr::<2>())
        .collect_arr::<2>()
    });

    let mut counter_p1 = HashMap::<[usize; 2], usize>::new();
    let mut counter_p2 = HashMap::<[usize; 2], usize>::new();

    for [[x1, y1], [x2, y2]] in points {
      let len = if y1 == y2 { x2.max(x1) - x1.min(x2) } else { y2.max(y1) - y1.min(y2) };

      for i in 0..=len {
        let x = if x1 == x2 { x1 } else if x2 > x1 { x2 - i } else { x2 + i };
        let y = if y1 == y2 { y1 } else if y2 > y1 { y2 - i } else { y2 + i };

        if x1 == x2 || y1 == y2 {
          *counter_p1.get_or_def_mut([x, y]) += 1;
        }

        *counter_p2.get_or_def_mut([x, y]) += 1;
      }
    }

    let part1 = || counter_p1.values().filter(|it| **it > 1).count();
    let part2 = || counter_p2.values().filter(|it| **it > 1).count();

    answer!(part, part1, part2)
  }
}
