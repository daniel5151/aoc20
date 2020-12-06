use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split("\n\n")
            .map(|g| g.split('\n').map(|p| p.chars()))
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let any = input
        .map(|g| g.flatten().collect::<HashSet<_>>().len())
        .sum();
    Ok(any)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    // this could be made marginally more efficient by lifting the fold into `g`
    // itself + counting the group size while calculating the char occurrence
    // hashmap, but I'm lazy, and this is fine.
    let all = input
        .map(|g| {
            let g_size = g.clone().count();
            g.flatten()
                .fold(HashMap::new(), |mut m, c| {
                    *m.entry(c).or_insert(0) += 1;
                    m
                })
                .iter()
                .filter(|(_, &v)| v == g_size)
                .count()
        })
        .sum();

    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 11;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = 6;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
