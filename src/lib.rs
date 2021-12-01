#![feature(array_windows)]

#[cfg(test)]
mod tests {
  const INPUT: &str = include_str!("../input");

  #[test]
  fn d1p1() {
    let nums = INPUT
      .lines()
      .map(|it| u32::from_str_radix(it, 10).unwrap())
      .collect::<Vec<_>>()
      .array_windows::<2>()
      .filter(|nums| nums[0] < nums[1])
      .count();

      eprintln!("{nums}")
  }

  #[test]
  fn d1p2() {
    let nums = INPUT
      .lines()
      .map(|it| u32::from_str_radix(it, 10).unwrap())
      .collect::<Vec<_>>()
      .array_windows::<3>()
      .map(|it| it.iter().sum())
      .collect::<Vec<u32>>()
      .array_windows::<2>()
      .filter(|nums| nums[0] < nums[1])
      .count();

      eprintln!("{nums}")
  }
}
