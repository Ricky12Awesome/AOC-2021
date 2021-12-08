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
  fn segments(chars: Chars) -> u8 {
    let mut num = 0;

    for c in chars {
      num |= 1 << (c as u8 - 97);
    }

    num
  }

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
            .map(Self::segments)
            .collect_vec()
        })
      })
      .map(|it| it.collect_arr::<2>())
      .collect_vec();

    let mut p1 = 0;

    for [unique, output] in segments {
      let mut v = Vec::with_capacity(unique.len());

      for bits in unique {
        match bits.count_ones() {
          1 | 3 | 4 | 8 => v.push(bits),
          _ => (),
        }
      }

      'out: for output_bits in output {
        for unique_bits in v.iter().copied() {
          if (output_bits & unique_bits) ^ unique_bits == 0 {
            p1 += 1;
            continue 'out;
          }
        }
      }
    }

    let p1 = || p1;
    let p2 = || 0;

    answer!(part, p1, p2)
  }
}
