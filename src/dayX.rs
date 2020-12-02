use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input.split('\n')
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let _ = input;

    Ok(0)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let _ = input;

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "...";
        let output = q1(input, &[]);
        assert_eq!(output.unwrap(), 0);
    }
}
