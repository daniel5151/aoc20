use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    }};
}

fn solve(init: &[usize], end_turn: usize) -> usize {
    let mut num_spoken_at: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
    let mut prev_n = 0;

    // init
    for (i, &n) in init.iter().enumerate() {
        num_spoken_at.insert(n, (i, None));
        prev_n = n;
    }

    // send it
    for i in init.len()..end_turn {
        let n = match num_spoken_at.get(&prev_n).unwrap() {
            (_, None) => 0,
            (turn, Some(last_turn)) => last_turn - turn,
        };

        match num_spoken_at.get_mut(&n) {
            None => {
                num_spoken_at.insert(n, (i, None));
            }
            Some((_, last_turn @ None)) => {
                *last_turn = Some(i);
            }
            Some((turn, Some(last_turn))) => {
                *turn = *last_turn;
                *last_turn = i;
            }
        }

        prev_n = n;
    }

    prev_n
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    Ok(solve(&input, 2020))
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    eprintln!("It'll run faster in --release!");
    Ok(solve(&input, 30000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
0,3,6
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 436 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
