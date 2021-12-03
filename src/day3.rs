use crate::day::*;
use itertools::Itertools;

day!(Day3);

impl Day for Day3 {
  fn day(part: Part) -> Answer {
    let lines = Self::INPUT.lines();

    let mut one = [0; 16];
    let mut zero = [0; 16];

    let mut nums1 = lines
      .clone()
      .map(|it| it.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();

    let mut nums2 = nums1.clone();

    for line in lines {
      for (index, n) in line.split("").filter(|it| !it.is_empty()).enumerate() {
        let n = n.parse::<u32>().unwrap();

        if n == 1 {
          one[index] += 1;
        }

        if n == 0 {
          zero[index] += 1;
        }
      }
    }

    let mut str = String::new();
    let mut str2 = String::new();

    for i in 0..12 {
      if one[i] > zero[i] {
        str += "1";
        str2 += "0";
      } else {
        str += "0";
        str2 += "1";
      }
    }

    let part1 = || {
      let n1 = u128::from_str_radix(&str, 2).unwrap();
      let n2 = u128::from_str_radix(&str2, 2).unwrap();

      n1 * n2
    };

    let mut str1p = String::new();
    let mut str2p = String::new();

    for i in 0..12 {
      let zero1 = nums1.iter().filter(|it| it[i] == '0').count();
      let zero2 = nums2.iter().filter(|it| it[i] == '0').count();
      let one1 = nums1.iter().filter(|it| it[i] == '1').count();
      let one2 = nums2.iter().filter(|it| it[i] == '1').count();
      let c1 = if one1 >= zero1 { '1' } else { '0' };
      let c2 = if one2 < zero2 { '1' } else { '0' };

      if nums1.len() > 1 {
        nums1 = nums1
          .iter()
          .filter(|it| it[i] == c1)
          .cloned()
          .collect::<_>();
      }

      str1p = String::from_iter(nums1.first().unwrap());

      if nums2.len() > 1 {
        nums2 = nums2
          .iter()
          .filter(|it| it[i] == c2)
          .cloned()
          .collect::<_>();
      }

      str2p = String::from_iter(nums2.first().unwrap());
    }

    let part2 = || {
      let n1 = u32::from_str_radix(&str1p, 2).unwrap();
      let n2 = u32::from_str_radix(&str2p, 2).unwrap();

      n1 * n2
    };

    answer!(part, part1, part2)
  }
}
