use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input.split('\n').map(|ln| ln.as_bytes())
    }};
}

fn count_trees<'a>(lns: impl Iterator<Item = &'a [u8]>, (dx, dy): (usize, usize)) -> usize {
    lns.step_by(dy)
        .zip((0..).step_by(dx))
        .filter(|(ln, x)| ln[x % ln.len()] == b'#')
        .count()
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let total = count_trees(input, (3, 1));
    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let total = slopes
        .iter()
        .map(|slope| count_trees(input.clone(), *slope))
        .product();

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
