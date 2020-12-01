use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .split('\n')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    for v in input.into_iter().combinations(2) {
        if v[0] + v[1] == 2020 {
            return Ok(v[0] * v[1]);
        }
    }

    Err("invalid input".into())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    for v in input.into_iter().combinations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            return Ok(v[0] * v[1] * v[2]);
        }
    }

    Err("invalid input".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = r#"
1721
979
366
299
675
1456
"#
        .trim();
        let output = q1(input, &[]);
        assert_eq!(output.unwrap(), 514579);
    }

    #[test]
    fn q2_e1() {
        let input = r#"
1721
979
366
299
675
1456
"#
        .trim();
        let output = q2(input, &[]);
        assert_eq!(output.unwrap(), 241861950);
    }
}
