use crate::day::*;

day!(Day11, Some(1741), Some(440));

impl Day11 {
  fn day(part: Part) -> Answer<usize> {
    let grid = Self::INPUT
      .lines()
      .map(|it| it.chars().map(parse_char::<usize>).collect_arr::<10>())
      .collect_arr::<10>();

    let mut map = HashMap::new();

    for y in 0..10 {
      for x in 0..10 {
        map.insert([y, x], grid[y][x]);
      }
    }

    fn do_flash(grid: &mut HashMap<[usize; 2], usize>, y: usize, x: usize, flashed: &mut HashSet<[usize; 2]>) {
      if flashed.contains(&[y, x]) {
        return;
      }

      flashed.insert([y, x]);

      for ([y, x], _) in grid.get_adjacent_at(y, x).into_iter().flatten() {
        *grid.get_or_def_mut([y, x]) += 1;
      }

      for ([y, x], lvl) in grid.get_adjacent_at(y, x).into_iter().flatten() {
        if lvl > 9 {
          do_flash(grid, y, x, flashed);
        }
      }
    }

    fn do_step(grid: &mut HashMap<[usize; 2], usize>, flashed: &mut HashSet<[usize; 2]>) {
      for (_, val) in grid.iter_mut() {
        *val += 1;
      }

      for y in 0..10 {
        for x in 0..10 {
          if *grid.get(&[y, x]).unwrap() > 9 {
            do_flash(grid, y, x, flashed);
          }
        }
      }

      for [y, x] in flashed.iter().copied() {
        *grid.get_or_def_mut([y, x]) = 0;
      }
    }

    let mut sum = 0;
    let mut step = 0;

    loop {
      let mut set = HashSet::new();
      do_step(&mut map, &mut set);

      if step < 100 {
        sum += set.len();
      }

      step += 1;

      if map.values().sum::<usize>() == 0 {
        break
      }
    }

    let p1 = || sum;
    let p2 = || step;

    answer!(part, p1, p2)
  }
}
