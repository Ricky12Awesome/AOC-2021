use crate::day::*;

day!(Day6, Some(380243), Some(1708791884591));

impl Day6 {
  fn day(part: Part) -> Answer<usize> {
    let states = Self::INPUT.split(',').map(parse_int::<usize>).collect_arr::<300>();

    let fish_count = |day: usize| {
      let mut counter = [0; 9];

      for state in &states {
        counter[*state] += 1;
      }

      for _ in 0..day {
        let n = counter[1];
        counter[1..].rotate_left(1);
        counter[8] = 0;
        counter[8] += counter[0];
        counter[6] += counter[0];
        counter[0] = n;
      }

      counter.iter().sum::<usize>()
    };

    let p1 = || fish_count(80);
    let p2 = || fish_count(256);

    answer!(part, p1, p2)
  }
}
