use crate::day::*;
use itertools::Itertools;

day!(Day4);

impl Day4 {
  fn day(part: Part) -> Answer<u32> {
    let input = Self::INPUT
      .lines()
      .filter(|it| !it.is_empty())
      .collect_vec();

    let numbers = input[0]
      .split(',')
      .map(|it| it.parse::<u32>().unwrap())
      .collect_vec();

    let mut boards = input[1..]
      .chunks(5)
      .map(|it| {
        it.iter()
          .map(|it| {
            it.split(' ')
              .filter(|it| !it.is_empty())
              .map(|it| (it.parse::<u32>().unwrap(), false))
              .collect_vec()
          })
          .collect_vec()
      })
      .collect_vec();

    let check = |board: &Vec<Vec<(u32, bool)>>, sum: &mut u32| {
      let mut ret = false;

      for row in board {
        if row.iter().all(|it| it.1) {
          ret = true;
          continue;
        }

        for (n, marked) in row {
          if !marked {
            *sum += n;
          }
        }
      }

      if ret {
        return ret;
      }

      *sum = 0;

      let mut flip = board.clone();

      for (line_n, row) in board.iter().enumerate() {
        for (line_c, c) in row.iter().enumerate() {
          flip[line_c][line_n] = *c;
        }
      }

      for col in flip {
        if col.iter().all(|it| it.1) {
          ret = true;
          continue;
        }

        for (n, marked) in col {
          if !marked {
            *sum += n;
          }
        }
      }

      ret
    };

    'root: for n in numbers {
      for (ib, board) in boards.clone().iter().enumerate() {
        for (ir, row) in board.iter().enumerate() {
          let marked = row.iter().positions(|it| it.0 == n).collect_vec();

          for m in marked {
            boards[ib][ir][m].1 = true;

            let mut sum = 0;

            if check(&boards[ib], &mut sum) {
              println!("{n} {sum} {}", n * sum);
              println!("{:?}", board);
              break 'root;
            }
          }

        }

      }
    }

    let part1 = || 0;

    let part2 = || 0;

    answer!(part, part1, part2)
  }
}
