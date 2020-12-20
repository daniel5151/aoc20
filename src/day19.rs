use crate::prelude::*;

#[derive(Debug)]
enum Rule<'a> {
    Terminal(&'a str),
    Seq(Vec<usize>),
}

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut sections = input.split("\n\n");
        let rules = sections
            .next()
            .ok_or("missing rules")?
            .split('\n')
            .map(|ln| -> DynResult<_> {
                let mut ln = ln.split(": ");
                let key = ln.next().unwrap().parse::<usize>()?;
                let val = ln.next().ok_or("missing rule rhs")?;
                let rules = val
                    .split(" | ")
                    .map(|rule| -> DynResult<_> {
                        let rule = match rule.strip_prefix('"') {
                            Some(rest) => {
                                Rule::Terminal(rest.strip_suffix('"').ok_or("malformed terminal")?)
                            }
                            None => Rule::Seq(
                                rule.split(' ')
                                    .map(|rule| rule.parse::<usize>())
                                    .collect::<Result<Vec<_>, _>>()?,
                            ),
                        };
                        Ok(rule)
                    })
                    .collect::<DynResult<Vec<_>>>()?;
                Ok((key, rules))
            })
            .collect::<DynResult<HashMap<_, _>>>()?;
        let msgs = sections.next().ok_or("missing messages")?.split('\n');

        (rules, msgs)
    }};
}

fn validate_msg<'a>(
    msg: &'a str,
    rules: &HashMap<usize, Vec<Rule<'a>>>,
    rule: usize,
    indent: usize,
) -> Option<&'a str> {
    let indent_s = "    ".repeat(indent);

    macro_rules! log {
        ($fmt:literal, $($args:tt)*) => {
            eprintln!(concat!("{}", $fmt), indent_s, $($args)*);
        };
    }

    log!("validate_msg({} : {:?})", rule, msg);
    let options = rules.get(&rule).unwrap();

    for rule in options {
        let remaining = match rule {
            Rule::Terminal(val) => msg.strip_prefix(val),
            Rule::Seq(seq) => {
                let mut msg = Some(msg);
                for rule in seq {
                    if let Some(inner_msg) = msg {
                        msg = validate_msg(inner_msg, rules, *rule, indent + 1);
                    } else {
                        break;
                    }
                }
                msg
            }
        };

        match remaining {
            None => continue,              // this branch hard-failed,
            Some(msg) => return Some(msg), // cool, this came through fine
        }
    }

    None
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (rules, msgs) = munge_input!(input);

    Ok(msgs
        .filter(|msg| validate_msg(msg, &rules, 0, 0) == Some(""))
        .count())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (mut rules, msgs) = munge_input!(input);

    *rules.get_mut(&8).unwrap() = vec![Rule::Seq(vec![42]), Rule::Seq(vec![42, 8])];
    *rules.get_mut(&11).unwrap() = vec![Rule::Seq(vec![42, 31]), Rule::Seq(vec![42, 11, 31])];

    Ok(msgs
        .filter(|msg| validate_msg(msg, &rules, 0, 0) == Some(""))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 2 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

    #[test]
    fn q1_e2() {
        let input = EXAMPLE_2;
        let expected = { 3 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_2;
        let expected = { 12 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
