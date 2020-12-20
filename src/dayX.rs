use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
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

    const EXAMPLE_1: &str = r#"
.....dummy.....
.....input.....
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 0 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    // #[test]
    // fn q2_e1() {
    //     let input = EXAMPLE_1;
    //     let expected = { 0 };
    //     let q = q2;

    //     assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    // }
}
