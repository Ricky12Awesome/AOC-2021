use crate::day::*;

day!(Day9);

impl Day9 {
  fn day(part: Part) -> Answer<usize> {
    let map = Self::INPUT.lines().map(|it| it.chars().map(parse_char::<usize>));
    let mut mins = HashMap::<[isize; 2], usize>::new();

    for (col, row) in map.enumerate() {
      for (row, val) in row.enumerate() {
        mins.insert([col as isize, row as isize], val);
      }
    }

    let adjacent = |map: &HashMap<[isize; 2], usize>, col: isize, row: isize| {
      [
        ([col + 1, row], *map.get_or(&[col + 1, row], &10)), // up
        ([col, row - 1], *map.get_or(&[col, row - 1], &10)), // left
        ([col, row], *map.get_or(&[col, row], &10)),         // middle
        ([col, row + 1], *map.get_or(&[col, row + 1], &10)), // right
        ([col - 1, row], *map.get_or(&[col - 1, row], &10)), // down
      ]
    };

    loop {
      let tmp = mins.clone();
      mins.clear();

      for ([col, row], _) in tmp.iter() {
        let val = adjacent(&tmp, *col, *row);
        let (cords, val) = val.into_iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap();

        mins.insert(cords, val);
      }

      if tmp.len() == mins.len() {
        break;
      }
    }

    let sum = mins.values().map(|it| *it + 1).sum::<usize>();

    let p1 = || sum;
    let p2 = || 0;

    answer!(part, p1, p2)
  }
}
