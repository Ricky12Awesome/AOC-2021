use crate::day::*;

//be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce

day!(Day8);

#[allow(unused)]
impl Day8 {
  fn day(part: Part) -> Answer<usize> {
    let segments = Self::INPUT
      .lines()
      .map(|it| it.split('|'))
      .map(|it| {
        it.map(|it| {
          it.split(' ')
            .map(str::trim)
            .filter(not_empty)
            .map(str::chars)
            .map(|it| it.fold(0u8, |n, c| n | 1 << (c as u8 - b'a')))
            // .collect_vec()
        })
      })
      .map(|it| it.collect_arr::<2>())
      .collect_vec();

    let mut p1 = 0;

    for [unique, output] in segments {
      p1 += output.filter(|x| [2, 3, 4, 7].contains(&x.count_ones())).count()
    }

    let p1 = || p1;
    let p2 = || 0;

    answer!(part, p1, p2)
  }
}
