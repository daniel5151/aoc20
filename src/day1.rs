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

fn find(nums: impl Iterator<Item = usize>, group_size: usize, sum: usize) -> Option<usize> {
    for v in nums.combinations(group_size) {
        if v.iter().sum::<usize>() == sum {
            return Some(v.iter().product());
        }
    }
    None
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    find(input.into_iter(), 2, 2020).ok_or_else(|| "invalid input".into())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    find(input.into_iter(), 3, 2020).ok_or_else(|| "invalid input".into())
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
