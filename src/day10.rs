use crate::day::*;

day!(Day10, Some(394647), Some(2380061249));

impl Day10 {
  fn day(part: Part) -> Answer<usize> {
    let mut incorrect = 0;
    let mut correct = Vec::new();

    'line: for line in Self::INPUT.lines().map(|it| it.chars()) {
      let mut open = Vec::new();

      for c in line {
        match c {
          '(' | '[' | '{' | '<' => open.push(c),
          ')' | ']' | '}' | '>' => match (open.pop().unwrap_or(' '), c) {
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
            _ => {
              incorrect += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
              };

              continue 'line;
            }
          },
          _ => unreachable!(),
        }
      }

      correct.push(
        open
          .iter()
          .rev()
          .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
          })
          .fold(0usize, |curr, value| curr * 5 + value),
      )
    }

    let p1 = || incorrect;

    let mut p2 = || {
      correct.sort_unstable();

      correct[correct.len() / 2]
    };

    answer!(part, p1, p2)
  }
}
