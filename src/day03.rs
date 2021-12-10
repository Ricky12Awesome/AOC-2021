use crate::day::*;

day!(Day03, Some(3687446), Some(4406844));

impl Day03 {
  fn day(part: Part) -> Answer<u32> {
    let lines = Self::INPUT
      .lines()
      .map(|line| line.trim().chars().collect_vec())
      .collect_vec();

    let counts = |lines: &Vec<Vec<char>>| {
      let mut zeros = [0u32; 12];
      let mut ones = [0u32; 12];

      for line in lines {
        for (i, c) in line.iter().enumerate() {
          match c {
            '0' => zeros[i] += 1,
            '1' => ones[i] += 1,
            _ => unreachable!(),
          }
        }
      }

      [zeros, ones]
    };

    let offset = 20; // 32 - 12

    let part1 = || {
      let [zeros, ones] = counts(&lines);
      let mut x = 0u32;

      for i in 0..12 {
        if ones[i] > zeros[i] {
          x |= 1 << i;
        }
      }

      let x = x.reverse_bits() >> offset;
      let y = (!x << offset) >> offset;

      x * y
    };

    let part2 = || {
      let val = |mut lines: Vec<Vec<char>>, c: char| {
        for i in 0..12 {
          let [zeros, ones] = counts(&lines);
          let c = match c {
            '1' if ones[i] >= zeros[i] => '1',
            '0' if ones[i] < zeros[i] => '1',
            _ => '0',
          };

          lines = lines.into_iter().filter(|it| it[i] == c).collect_vec();

          if lines.len() == 1 {
            let str = String::from_iter(lines.first().unwrap());
            return u32::from_str_radix(&str, 2).unwrap();
          }
        }

        unreachable!()
      };

      let x = val(lines.clone(), '1');
      let y = val(lines.clone(), '0');

      x * y
    };

    answer!(part, part1, part2)
  }
}
