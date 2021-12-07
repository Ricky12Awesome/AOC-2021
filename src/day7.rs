use crate::day::*;

day!(Day7, Some(323647), Some(87640209));

impl Day7 {
  fn day(part: Part) -> Answer<usize> {
    let positions = Self::INPUT.split(',').map(parse_int::<usize>).collect_arr::<1000>();
    let compare = 0..*positions.iter().max().unwrap();

    let mut fuels_p1 = HashMap::<usize, usize>::new();
    let mut fuels_p2 = HashMap::<usize, usize>::new();

    for n in compare {
      for pos in positions {
        let fuel = n.max(pos) - n.min(pos);
        let cost = fuel * (fuel + 1) / 2;

        *fuels_p1.get_or_def_mut(n) += fuel;
        *fuels_p2.get_or_def_mut(n) += cost;
      }
    }

    let p1 = || *fuels_p1.values().min().unwrap();
    let p2 = || *fuels_p2.values().min().unwrap();

    answer!(part, p1, p2)
  }
}
