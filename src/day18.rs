use crate::prelude::*;

// CONTENT WARNING: ABSOLUTELY TRASH CODE
//
// Challenge: can I slap together a solution without writing a "proper" parser?
// Answer: yes, but fuck me is this code absolutely ass.
//
// I'll try to find some time this weekend to revisit it and write a proper
// parser that actually spits out a tree, but there's a non-zero chance I just
// won't.
//
// I won't lie, parsing was always my _least_ favorite part of my compilers /
// programming language courses and Uni, and because of that, I don't actually
// remember how to hand-roll a parser. Whenever I have to do this sort of thing
// "on the job", I'll just use something like `pest` or `nom`.
//
// anyways, yeah, fuck this problem, and fuck this code.

fn solve_expr(s: &[u8], with_precedence: bool) -> (u64, usize) {
    // eprintln!("solving {:?}", String::from_utf8_lossy(s));

    let mut vals = Vec::new();
    let mut ops = Vec::new();

    let mut chars = s.iter().copied().enumerate();
    let mut final_i = None;
    while let Some((i, c)) = chars.next() {
        match c {
            b' ' => continue,
            b'+' | b'*' => ops.push(c as char),
            b'0'..=b'9' => vals.push((c - b'0') as u64),
            b'(' => {
                let (val, end_i) = solve_expr(&s[(i + 1)..], with_precedence);
                vals.push(val);

                let end_i = end_i + i;
                // eprintln!("skipping until {}", end_i);
                while !matches!(chars.next(), Some((i, _)) if i == end_i) {}
                chars.next();
                // eprintln!(
                //     "remaining: {}",
                //     String::from_utf8(chars.clone().map(|(_, c)|
                // c).collect::<Vec<u8>>())
                //         .unwrap_or_default()
                // )
            }
            b')' => {
                final_i = Some(i);
                break;
            }
            _ => panic!("invalid input"),
        }
    }

    vals.reverse();
    ops.reverse();

    // eprintln!("vals: {:?}", vals);
    // eprintln!("ops: {:?}", ops);

    if with_precedence {
        // eval all '+' first
        while let Some(pos) = ops.iter().position(|op| *op == '+') {
            let op = ops.remove(pos);

            let a = vals.remove(pos);
            let b = vals.remove(pos);
            // eprintln!("calc: {} {} {}", a, op, b);

            vals.insert(
                pos,
                match op {
                    '+' => a + b,
                    '*' => a * b,
                    _ => unreachable!(),
                },
            );
        }
    }

    while let Some(op) = ops.pop() {
        let a = vals.pop().unwrap();
        let b = vals.pop().unwrap();

        // eprintln!("calc: {} {} {}", a, op, b);

        vals.push(match op {
            '+' => a + b,
            '*' => a * b,
            _ => unreachable!(),
        });
    }

    // eprintln!("returned {}, {}\n", vals[0], final_i.unwrap_or_default());

    assert!(vals.len() == 1);

    (vals[0], final_i.unwrap_or_default())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let sum = input
        .split('\n')
        .map(|ln| solve_expr(ln.as_bytes(), false).0)
        .sum::<u64>();
    Ok(sum as usize)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let sum = input
        .split('\n')
        .map(|ln| solve_expr(ln.as_bytes(), true).0)
        .sum::<u64>();
    Ok(sum as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 26 + 437 + 12240 + 13632 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 46 + 1445 + 669060 + 23340 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
