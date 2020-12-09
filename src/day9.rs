use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?
    }};
}

pub fn first_invalid(nums: &[usize], preamble: usize) -> DynResult<usize> {
    'outer: for g in nums.windows(preamble + 1) {
        let n = *g.last().ok_or("nums cannot be empty")?;
        for combo in g.iter().copied().combinations(2) {
            if combo.iter().sum::<usize>() == n {
                continue 'outer;
            }
        }
        return Ok(n);
    }

    Err("all numbers are valid".into())
}

pub fn q1(input: &str, args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let preamble = match args.get(0) {
        Some(n) => n.parse::<usize>().map_err(|_e| "invalid preamble len")?,
        None => 25,
    };

    let first_invalid = first_invalid(&input, preamble)?;
    Ok(first_invalid)
}

pub fn q2(input: &str, args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let preamble = match args.get(0) {
        Some(n) => n.parse::<usize>().map_err(|_e| "invalid preamble len")?,
        None => 25,
    };

    let sum = first_invalid(&input, preamble)?;
    let max_i = input.iter().position(|e| *e == sum).unwrap();

    for i in 0..max_i {
        for j in i..max_i {
            let range = &input[i..=j];
            if range.iter().sum::<usize>() == sum {
                match range.iter().minmax() {
                    MinMaxResult::MinMax(min, max) => return Ok(min + max),
                    _ => return Err("invalid input".into()),
                }
            }
        }
    }

    Err("could not find valid span".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 127;
        let q = q1;

        assert_eq!(q(input.trim(), &["5"]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = 62;
        let q = q2;

        assert_eq!(q(input.trim(), &["5"]).unwrap(), expected);
    }
}
