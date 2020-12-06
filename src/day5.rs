use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|ln| ln.as_bytes().split_at(7))
            .map(|(row, col)| {
                (
                    row.iter().fold(0, |a, c| a << 1 | (*c == b'B') as usize),
                    col.iter().fold(0, |a, c| a << 1 | (*c == b'R') as usize),
                )
            })
            .map(|(row, col)| row * 8 + col)
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let max = input.max().ok_or("invalid input")?;
    Ok(max)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let ids = input.sorted().collect::<Vec<_>>();
    for ids in ids.windows(2) {
        if ids[0] + 1 != ids[1] {
            return Ok(ids[0] + 1);
        }
    }

    Err("could not find seat".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 820;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    // #[test]
    // fn q2_e1() {
    //     let input = EXAMPLE_1;
    //     let expected = 0;
    //     let q = q2;

    //     assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    // }
}
