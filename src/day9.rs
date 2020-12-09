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
        for combo in g.combinations_const::<2>() {
            if combo.iter().copied().sum::<usize>() == n {
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

    let target_sum = first_invalid(&input, preamble)?;

    // restrict the input to all elements before the target.
    //
    // for maximum efficiency, the index could be calculated as part of
    // `first_invalid`, but another pass isn't the end of the world...
    let max_i = input.iter().position(|e| *e == target_sum).unwrap();
    let input = &input[0..max_i];

    // grows the sum by extending from the right-hand-side of the window, then
    // shrinks the sum by shrinking from the left-hand-side of the window.
    let mut sum = input[0] + input[1];
    let (mut l, mut r) = (0, 2);
    loop {
        while sum < target_sum {
            sum += match input.get(r) {
                None => return Err("could not find valid span".into()),
                Some(v) => {
                    r += 1;
                    v
                }
            };
        }

        if sum == target_sum {
            // IMPROVEMENT: keep track of min and max while iterating
            // in the worst-case, this final iteration results in a full traversal of the
            // input array
            let (min, max) = input[l..r].iter().minmax().into_option().unwrap();
            return Ok(min + max);
        }

        while sum > target_sum {
            sum -= input[l];
            l += 1;

            if l == r {
                return Err("could not find valid span".into());
            }
        }
    }
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
