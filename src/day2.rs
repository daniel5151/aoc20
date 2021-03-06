use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|ln| -> Option<_> {
                let mut ln = ln.split(' ');
                let mut range = ln.next()?.split('-');
                let min = range.next()?.parse::<usize>().ok()?;
                let max = range.next()?.parse::<usize>().ok()?;
                let c = ln.next()?.chars().next()?;
                let pass = ln.next()?;
                Some(((min, max), c, pass.to_owned()))
            })
            .collect::<Option<Vec<_>>>()
            .ok_or("invalid input")?
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let valid = input
        .into_iter()
        .filter(|((min, max), c, pass)| {
            (min..=max).contains(&&pass.chars().filter(|pc| pc == c).count())
        })
        .count();

    Ok(valid)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let valid = input
        .into_iter()
        .filter(|((a, b), c, pass)| {
            let (a, b) = (a - 1, b - 1);
            let c = *c as u8;
            let pass = pass.as_bytes();

            (pass[a] == c || pass[b] == c) && pass[a] != pass[b]
        })
        .count();

    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 2;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = 1;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
