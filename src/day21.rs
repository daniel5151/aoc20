use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|ln| -> DynResult<_> {
                let mut sections = ln.split(" (contains ");
                let ings = sections.next().unwrap().split(' ').collect::<HashSet<_>>();
                let allergens = sections
                    .next()
                    .ok_or("missing allergens")?
                    .strip_suffix(')')
                    .ok_or("malformed allergens")?
                    .split(", ")
                    .collect::<HashSet<_>>();
                Ok((ings, allergens))
            })
            .collect::<DynResult<Vec<_>>>()?
    }};
}

fn get_allergen_guesses<'a>(
    ings_allergens: &[(HashSet<&'a str>, HashSet<&'a str>)],
) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut guesses = HashMap::new();
    for (ings, allergens) in ings_allergens {
        for &allergen in allergens {
            match guesses.get_mut(allergen) {
                None => {
                    guesses.insert(allergen, ings.clone());
                }
                Some(ings2) => *ings2 = ings2.intersection(ings).copied().collect(),
            }
        }
    }

    guesses
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let guesses = get_allergen_guesses(&input);

    dbg_map!(guesses);

    // find all ingredients _not_ in guesses
    let guesses = guesses
        .iter()
        .flat_map(|(_, ings)| ings.iter().copied())
        .collect::<HashSet<_>>();

    let all_ings = input
        .iter()
        .flat_map(|(ings, _)| ings.iter().copied())
        .collect::<HashSet<&str>>();

    let no_allergen_ings = all_ings.difference(&guesses).collect::<HashSet<_>>();

    let ing_counts = input
        .into_iter()
        .flat_map(|(ings, _)| ings.into_iter())
        .fold(HashMap::<&str, usize>::new(), |mut m, ing| {
            *m.entry(ing).or_default() += 1;
            m
        });

    let ans = no_allergen_ings
        .into_iter()
        .map(|ing| ing_counts[ing])
        .sum::<usize>();

    Ok(ans)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<String> {
    let input = munge_input!(input);

    let mut guesses = get_allergen_guesses(&input);

    dbg_map!(guesses);

    // lesson learned from day16 - lets keep things simple and assume there is
    // always an ingredient with a perfect guess.

    let mut bad_ings = Vec::new();
    while !guesses.is_empty() {
        let allergen = guesses
            .iter()
            .find(|(_, ings)| ings.len() == 1)
            .map(|(allergen, _)| *allergen)
            .ok_or("more complex then you thought")?;
        let ing = guesses
            .remove(allergen)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        bad_ings.push((ing, allergen));

        // update guesses
        for (_, ings) in guesses.iter_mut() {
            ings.remove(ing);
        }
    }

    bad_ings.sort_by(|(_, a), (_, b)| a.cmp(b));

    Ok(bad_ings
        .into_iter()
        .map(|(ing, _)| ing)
        .collect::<Vec<_>>()
        .join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"#;

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 5 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { "mxmxvkd,sqjhc,fvjkl" };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
