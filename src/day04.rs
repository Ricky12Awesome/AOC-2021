use crate::day::*;

day!(Day04, Some(49860), Some(24628));

impl Day04 {
  fn day(part: Part) -> Answer<u32> {
    let mut input = Self::INPUT.lines().filter(not_empty);
    let numbers = input.unwrap_next().split(',').map(parse_int::<u32>);
    let boards = input
      .map(|it| it.split(' ').filter(not_empty).map(parse_int::<u32>).collect_vec())
      .collect_vec();

    let mut won_history = vec![];
    let mut values = [[[0; 5]; 5]; 100];
    let [mut won, mut sum] = [[0; 100]; 2];
    let [mut row_marked, mut col_marked] = [[[[false; 5]; 5]; 100]; 2];

    for n in numbers {
      for (id, board) in boards.chunks(5).enumerate() {
        for (x, row) in board.iter().enumerate() {
          for (y, val) in row.iter().enumerate() {
            values[id][x][y] = *val;

            if n == *val {
              row_marked[id][x][y] = true;
              col_marked[id][y][x] = true;

              if won[id] == 0 && (row_marked[id][x] == [true; 5] || col_marked[id][y] == [true; 5]) {
                won_history.push(id);
                won[id] = n;

                for x in 0..5 {
                  for y in 0..5 {
                    if !row_marked[id][x][y] {
                      sum[id] += values[id][x][y]
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    let (first, last) = (won_history[0], won_history[won_history.len() - 2]);
    let part1 = || won[first] * sum[first];
    let part2 = || won[last] * sum[last];

    answer!(part, part1, part2)
  }
}
