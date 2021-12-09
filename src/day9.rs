use crate::day::*;

day!(Day9);

impl Day9 {
  fn day(part: Part) -> Answer<usize> {
    let input = Self::INPUT.lines().map(|it| it.chars().map(parse_char::<usize>));
    let mut map = HashMap::<[isize; 2], usize>::new();

    for (col, row) in input.enumerate() {
      for (row, val) in row.enumerate() {
        map.insert([col as isize, row as isize], val);
      }
    }

    #[rustfmt::skip]
    fn adjacent(map: &HashMap<[isize; 2], usize>, [col, row]: [isize; 2], border: usize) -> [([isize; 2], usize); 5] {
      [
        ([col + 1, row], *map.get_or(&[col + 1, row], &border)), // up
        ([col, row - 1], *map.get_or(&[col, row - 1], &border)), // left
        ([col, row    ], *map.get_or(&[col, row    ], &border)), // middle
        ([col, row + 1], *map.get_or(&[col, row + 1], &border)), // right
        ([col - 1, row], *map.get_or(&[col - 1, row], &border)), // down
      ]
    }

    fn fill(map: &HashMap<[isize; 2], usize>, cords: [isize; 2], checked: &mut HashSet<[isize; 2]>) {
      let [(up, _), (left, _), (_, mid), (right, _), (down, _)] = adjacent(map, cords, 9);

      if mid < 9 && !checked.contains(&cords) {
        checked.insert(cords);
        fill(map, up, checked);
        fill(map, left, checked);
        fill(map, right, checked);
        fill(map, down, checked);
      }
    }

    let p1 = || {
      let mut minimums = map.clone();

      loop {
        let tmp = minimums.clone();
        minimums.clear();

        for (cords, _) in tmp.iter() {
          let val = adjacent(&tmp, *cords, 69);
          let (cords, val) = val.into_iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

          minimums.insert(cords, val);
        }

        if tmp.len() == minimums.len() {
          break;
        }
      }

      minimums.values().map(|it| *it + 1).sum::<usize>()
    };

    let p2 = || {
      let mut filled = HashSet::new();
      let mut sums = Vec::with_capacity(map.len());

      for (cords, _) in map.iter() {
        if !filled.contains(cords) {
          let mut checked = HashSet::new();

          fill(&map, *cords, &mut checked);

          if !checked.is_empty() {
            sums.push(checked.len());
          }

          filled.extend(checked);
        }
      }

      sums.sort_unstable();

      sums[sums.len() - 3..].iter().copied().reduce(|x, y| x * y).unwrap()
    };

    answer!(part, p1, p2)
  }
}
