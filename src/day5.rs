use crate::day::*;

day!(Day5, Some(6225), Some(22116));

impl Day5 {
  fn day(part: Part) -> Answer<usize> {
    let points = Self::INPUT.lines().map(|line| {
      line
        .split(" -> ")
        .map(|it| it.split(',').map(parse_int).collect_arr::<2>())
        .collect_arr::<2>()
    });

    let mut counter_p1 = HashMap::<[usize; 2], usize>::new();
    let mut counter_p2 = HashMap::<[usize; 2], usize>::new();

    for [[x1, y1], [x2, y2]] in points {
      let mut add = |p1: usize, p2: usize, p3: usize, y: bool| {
        for p in if p1 > p2 { p2..=p1 } else { p1..=p2 } {
          let k = if y { [p, p3] } else { [p3, p] };
          *counter_p1.get_or_def_mut(k) += 1;
          *counter_p2.get_or_def_mut(k) += 1;
        }
      };

      if y1 == y2 {
        add(x1, x2, y1, true);
        continue;
      }

      if x1 == x2 {
        add(y1, y2, x1, false);
        continue;
      }

      let len = y2.max(y1) - y1.min(y2);

      for i in 0..=len {
        let x = if x2 > x1 { x2 - i } else { x2 + i };
        let y = if y2 > y1 { y2 - i } else { y2 + i };

        *counter_p2.get_or_def_mut([x, y]) += 1;
      }
    }

    let part1 = || counter_p1.values().filter(|it| **it > 1).count();
    let part2 = || counter_p2.values().filter(|it| **it > 1).count();

    answer!(part, part1, part2)
  }
}
