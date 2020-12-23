use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let input = input
            .chars()
            .map(|c| ((c as u8) - b'0') as u32)
            .collect::<VecDeque<_>>();
        if input.len() != 9 || input.iter().any(|c| !(1..=9).contains(c)) {
            return Err("invalid input".into());
        }
        input
    }};
}

pub fn q1(input: &str, args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let iters = match args.get(0) {
        Some(iters) => iters.parse::<usize>().map_err(|_| "invalid num iters")?,
        None => 100,
    };

    let mut cups = input;

    for _ in 0..iters {
        let curr_cup = *cups.front().unwrap();
        cups.rotate_left(1);

        let picked_up = cups.drain(..3).take_array::<3>().unwrap();

        let dst_cup_i = {
            let mut cup = curr_cup - 1;
            loop {
                // handle underflow
                if cup == 0 {
                    cup = 9
                }
                if let Some(pos) = cups.iter().position(|v| *v == cup) {
                    break pos;
                }
                cup -= 1;
            }
        };

        for cup in picked_up.into_iter().rev() {
            cups.insert(dst_cup_i + 1, cup)
        }
    }

    let i = cups.iter().position(|v| *v == 1).unwrap();
    let (tail, head) = cups.make_contiguous().split_at(i);
    let ans = head
        .iter()
        .skip(1) // don't include 1
        .chain(tail.iter())
        .fold(0usize, |a, x| a * 10 + *x as usize);

    Ok(ans)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<u64> {
    let input = munge_input!(input);

    let _ = input;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
389125467
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 92658374 };
        let q = q1;

        assert_eq!(q(input.trim(), &["10"]).unwrap(), expected);
    }

    #[test]
    fn q1_e2() {
        let input = EXAMPLE_1;
        let expected = { 67384529 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 934001 * 159792 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
