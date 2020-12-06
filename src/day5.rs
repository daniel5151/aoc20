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
    // since it's a packed flight, once the seat IDs are sorted, they are guaranteed
    // to be monotonically increasing by 1, _except_ for one index, which increases
    // by 2. That "hole" is our seat ID.
    //
    // another thing to note is that while seat IDs are not guaranteed to start at
    // zero, it's trivial to normalize them as though they start at zero by
    // subtracting the first seat ID from all subsequent IDs.
    //
    // finding the "hole" becomes a simple matter of finding the first index where
    // the normalized seat ID is not equal to the seat ID's position in the IDs
    // array, and then subtracting one.
    //
    // NOTE: `partition_point_enumerated` is a custom slice extension method defined
    // in the `crate::prelude`.
    let hole = ids.partition_point_enumerated(|i, v| v - ids[0] == i);
    let seat = ids[hole] - 1;
    Ok(seat)
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
}
