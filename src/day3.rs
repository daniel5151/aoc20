use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|ln| ln.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut trees = 0;
    let mut x = 0;
    for (_y, ln) in input.into_iter().enumerate() {
        if ln[x % ln.len()] == '#' {
            trees += 1;
        }
        x += 3;
    }

    Ok(trees)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    const SLOPES: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total = 1;
    for &(dx, dy) in SLOPES {
        let mut trees = 0;
        let mut x = 0;
        for ln in input.iter().step_by(dy) {
            if ln[x % ln.len()] == '#' {
                trees += 1;
            }
            x += dx;
        }

        total *= trees;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let output = q1(input, &[]);
        assert_eq!(output.unwrap(), 7);
    }

    #[test]
    fn q2_e1() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let output = q2(input, &[]);
        assert_eq!(output.unwrap(), 336);
    }
}
