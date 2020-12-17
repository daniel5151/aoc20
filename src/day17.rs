use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut map = HashSet::new();
        for (y, ln) in input.split('\n').enumerate() {
            for (x, c) in ln.chars().enumerate() {
                if c == '#' {
                    map.insert((x as isize, y as isize));
                }
            }
        }
        map
    }};
}

fn solve<const DIMS: usize>(input: HashSet<(isize, isize)>, iters: usize) -> usize {
    let mut curr = input
        .into_iter()
        .map(|(x, y)| {
            let mut a = [0; DIMS];
            a[0] = x;
            a[1] = y;
            a
        })
        .collect::<HashSet<_>>();
    let mut next = HashSet::new();

    let deltas = CartesianProduct::new([-1..=1; DIMS]).filter(|c| *c != [0; DIMS]);

    for _ in 0..iters {
        // calculate bounds
        let bounds = (0..DIMS)
            .map(|d| {
                let (min, max) = curr
                    .iter()
                    .map(|cord| cord[d])
                    .minmax()
                    .into_option()
                    .unwrap();
                (min - 1)..=(max + 1)
            })
            .take_array() // uses the `iter_to_array` crate
            .unwrap();

        for cord in CartesianProduct::new(bounds) {
            let neighbors = deltas
                .clone()
                .filter(|deltas| {
                    let mut p = [0; DIMS];
                    for ((val, base), delta) in p.iter_mut().zip(cord.iter()).zip(deltas.iter()) {
                        *val = base + delta;
                    }
                    curr.contains(&p)
                })
                .count();

            if matches!(
                (curr.contains(&cord), neighbors),
                (true, 2) | (true, 3) | (false, 3)
            ) {
                next.insert(cord);
            }
        }

        curr = next.clone();
        next.clear();
    }

    curr.len()
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    Ok(solve::<3>(input, 6))
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    Ok(solve::<4>(input, 6))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
.#.
..#
###
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 112 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 848 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
