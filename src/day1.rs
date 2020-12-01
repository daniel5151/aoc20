use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .split('\n')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut m = HashSet::new();
    for n in input {
        m.insert(n);
        if m.contains(&(2020 - n)) {
            return Ok(n * (2020 - n));
        }
    }

    Err("invalid input".into())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut m: HashMap<usize, (usize, usize)> = HashMap::new();
    for n1 in input.iter().copied() {
        for n2 in input.iter().copied() {
            m.insert(n1 + n2, (n1, n2));
            if m.contains_key(&(2020 - n2)) {
                let (m1, m2) = m.get(&(2020 - n2)).unwrap();
                return Ok(n2 * m1 * m2);
            }
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
