use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut players = input.split("\n\n");
        let p1 = players
            .next()
            .unwrap()
            .split('\n')
            .skip(1)
            .map(|ln| ln.parse::<usize>())
            .collect::<Result<VecDeque<_>, _>>()?;
        let p2 = players
            .next()
            .ok_or("missing player 2")?
            .split('\n')
            .skip(1)
            .map(|ln| ln.parse::<usize>())
            .collect::<Result<VecDeque<_>, _>>()?;
        (p1, p2)
    }};
}

enum Winner {
    Human,
    Crab,
}

fn combat<const RECURSE: bool>(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> usize {
    fn combat_helper<const RECURSE: bool>(
        p1: &mut VecDeque<usize>,
        p2: &mut VecDeque<usize>,
    ) -> Winner {
        let mut cache = HashSet::new();

        loop {
            if !cache.insert((aoc::hash(p1), aoc::hash(p2))) {
                return Winner::Human;
            };

            let (p1_card, p2_card) = match (p1.front(), p2.front()) {
                (None, None) => unreachable!(),
                (None, Some(_)) => return Winner::Crab,
                (Some(_), None) => return Winner::Human,
                (Some(&a), Some(&b)) => (a, b),
            };

            p1.pop_front().unwrap();
            p2.pop_front().unwrap();

            let winner = if RECURSE && p1_card <= p1.len() && p2_card <= p2.len() {
                let rec = combat_helper::<true>(
                    &mut p1.iter().take(p1_card).copied().collect(),
                    &mut p2.iter().take(p2_card).copied().collect(),
                );

                rec
            } else {
                // classic rules
                if p1_card > p2_card {
                    Winner::Human
                } else {
                    Winner::Crab
                }
            };

            match winner {
                Winner::Human => {
                    p1.push_back(p1_card);
                    p1.push_back(p2_card);
                }
                Winner::Crab => {
                    p2.push_back(p2_card);
                    p2.push_back(p1_card);
                }
            }
        }
    }

    let winner = combat_helper::<RECURSE>(p1, p2);

    match winner {
        Winner::Human => &p1,
        Winner::Crab => &p2,
    }
    .iter()
    .rev()
    .enumerate()
    .map(|(i, x)| (i + 1) * x)
    .sum::<usize>()
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (mut p1, mut p2) = munge_input!(input);
    Ok(combat::<false>(&mut p1, &mut p2))
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (mut p1, mut p2) = munge_input!(input);
    Ok(combat::<true>(&mut p1, &mut p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 306 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 291 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = r#"
Player 1:
43
19

Player 2:
2
29
14
"#;

    #[test]
    fn q2_e2_terminates() {
        let input = EXAMPLE_2;
        let q = q2;
        q(input.trim(), &[]).unwrap();
    }
}
