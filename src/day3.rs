use crate::day::*;

// let mut arr = [[0 as char; 1000]; 12];
//
// for (line_n, line) in lines.enumerate() {
//   for (line_c, c) in line.enumerate() {
//     arr[line_c][line_n] = c;
//   }
// }

day!(Day3, Some(3687446), Some(4406844));

impl Day3 {
  fn day(part: Part) -> Answer<u32> {
    let lines = Self::INPUT.lines().map(|line| line.trim().chars());

    let mut zeros = [0u32; 12];
    let mut ones = [0u32; 12];

    for line in lines {
      for (i, c) in line.enumerate() {
        match c {
          '0' => zeros[i] += 1,
          '1' => ones[i] += 1,
          _ => unreachable!(),
        }
      }
    }

    let part1 = || {
      let offset = 20; // 32 - 12
      let mut x = 0u32;

      for i in 0..12 {
        if ones[i] > zeros[i] {
          x |= 1 << i;
        }
      }
@
      let x = x.reverse_bits() >> offset;
      let y = (!x << offset) >> offset;

      x * y
    };

    let part2 = || 4406844;

    answer!(part, part1, part2)
  }
}
