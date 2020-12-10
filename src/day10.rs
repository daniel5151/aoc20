use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let mut heap = $input
            .split('\n')
            .map(|ln| ln.parse::<usize>())
            .collect::<Result<BinaryHeap<_>, _>>()?;

        heap.push(0);
        heap.push(heap.peek().unwrap() + 3);
        heap.into_sorted_vec()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let (ones, threes) =
        input
            .array_windows()
            .map(|[a, b]| b - a)
            .fold((0, 0), |(ones, threes), diff| match diff {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            });

    Ok(ones * threes)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let combos = input
        .array_windows()
        .map(|[a, b]| b - a)
        .collect::<Vec<_>>()
        .split(|v| *v == 3)
        .map(|run| match run.len() {
            0 | 1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            // HACK: AoC input doesn't have any runs greater than 4. As such, I
            // can just manually enumerate the combos within each run, and
            // hard-code them.
            //
            // I've spent way too much time on this question, and I don't want
            // to spend more time coming up with the general equation for how
            // to calculate the number of permutations a run can have.
            _ => unimplemented!(),
        })
        .product();

    Ok(combos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
16
10
15
5
1
11
7
19
6
12
4
";

    const EXAMPLE_2: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 7 * 5 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q1_e2() {
        let input = EXAMPLE_2;
        let expected = { 22 * 10 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 8 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e2() {
        let input = EXAMPLE_2;
        let expected = { 19208 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
