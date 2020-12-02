use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        let mut v = Vec::new();
        for ln in input.split('\n') {
            let mut l = ln.split(' ');
            let mut nums = l.next().unwrap().split('-');
            let min = nums.next().unwrap();
            let max = nums.next().unwrap();
            let c = l.next().unwrap().chars().next().unwrap();
            let pass = l.next().unwrap();
            v.push((
                (min.parse::<usize>()?, max.parse::<usize>()?),
                c,
                pass.to_string(),
            ))
        }
        v
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut total = 0;
    for ((min, max), c, pass) in input {
        let count = pass.chars().filter(|c2| *c2 == c).count();
        if count >= min && count <= max {
            total += 1;
        }
    }

    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut total = 0;
    'outer: for ((min, max), c, pass) in input {
        let pass = pass.as_bytes();
        let mut n = 0;
        for i in &[min, max] {
            if pass[i - 1] == c as u8 {
                n += 1;
            }
            if n > 1 {
                continue 'outer;
            }
        }
        if n == 1 {
            total += 1;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let output = q1(input, &[]);
        assert_eq!(output.unwrap(), 2);
    }

    #[test]
    fn q2_e1() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let output = q2(input, &[]);
        assert_eq!(output.unwrap(), 1);
    }
}
