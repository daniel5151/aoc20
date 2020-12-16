use crate::prelude::*;

type Range = (usize, usize);

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut sections = input.split("\n\n");

        let fields = sections
            .next()
            .ok_or("missing fields section")?
            .split('\n')
            .map(|ln| -> DynResult<_> {
                let mut chunks = ln.split(": ");
                let name = chunks.next().unwrap();
                let mut ranges = chunks
                    .next()
                    .ok_or("missing field ranges")?
                    .split(" or ")
                    .map(|range| -> DynResult<_> {
                        let mut nums = range.split('-').map(|n| n.parse::<usize>());
                        Ok((
                            nums.next().ok_or("missing num in range")??,
                            nums.next().ok_or("missing num in range")??,
                        ))
                    });
                Ok((
                    name,
                    ranges.next().ok_or("missing first range")??,
                    ranges.next().ok_or("missing second range")??,
                ))
            })
            .collect::<DynResult<Vec<(&str, Range, Range)>>>()?;

        let my_ticket: Vec<usize> = sections
            .next()
            .ok_or("missing my ticket section")?
            .split('\n')
            .skip(1) // "your ticket:" header
            .next()
            .ok_or("missing my ticket")?
            .split(',')
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        let tickets: Vec<Vec<usize>> = sections
            .next()
            .ok_or("missing nearby tickets section")?
            .split('\n')
            .skip(1) // "nearby tickets:" header
            .map(|ticket| {
                ticket
                    .split(',')
                    .map(|n| n.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        (fields, my_ticket, tickets)
    }};
}

fn filter_invalid(
    fields: &[(&str, Range, Range)],
    tickets: Vec<Vec<usize>>,
) -> (usize, Vec<Vec<usize>>) {
    // this is a valid approach since the AoC output limits ranges for numbers
    // between 0 and 1000.
    let any_valid_map = fields
        .iter()
        .flat_map(|&(_, r1, r2)| Some(r1).into_iter().chain(Some(r2)))
        .map(|(start, end)| start..=end)
        .flatten()
        .collect::<HashSet<usize>>();

    let mut valid = Vec::new();
    let mut error_rate = 0;

    for ticket in tickets {
        let mut is_valid = true;
        for val in &ticket {
            if !any_valid_map.contains(&val) {
                error_rate += val;
                is_valid = false;
            }
        }
        if is_valid {
            valid.push(ticket)
        }
    }

    (error_rate, valid)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (fields, _my_ticket, tickets) = munge_input!(input);

    let (error_rate, _) = filter_invalid(&fields, tickets);

    Ok(error_rate)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (fields, my_ticket, tickets) = munge_input!(input);

    let (_, tickets) = filter_invalid(&fields, tickets);

    // associate field (index) with valid columns in tickets
    let mut guess_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (field_idx, (_, r1, r2)) in fields.iter().enumerate() {
        let (r1, r2) = (r1.0..=r1.1, r2.0..=r2.1);

        'col: for col in 0..tickets[0].len() {
            for val in tickets.iter().map(|ticket| ticket[col]) {
                if !r1.contains(&val) && !r2.contains(&val) {
                    continue 'col;
                }
            }
            guess_map.entry(field_idx).or_default().push(col);
        }
    }

    // okay, so this is like a sudoku, where you sometimes have to "guess" what's
    // the right column, and check if your guess was correct...

    // lets start with a quick-and-dirty recursive solution that doesn't do any
    // fancy caching...

    //  we fixup the guess_map to remove the guess
    fn fixup_guess_map(
        mut guess_map: HashMap<usize, Vec<usize>>,
        (field, col): (usize, usize),
    ) -> HashMap<usize, Vec<usize>> {
        guess_map.remove(&field);

        let mut empty_vecs = HashSet::new();
        for (&field, cols) in guess_map.iter_mut() {
            if let Some(pos) = cols.iter().position(|c| *c == col) {
                cols.remove(pos);
            }

            if cols.is_empty() {
                empty_vecs.insert(field);
            }
        }

        // and remove the empty vecs
        for field in empty_vecs {
            guess_map.remove(&field);
        }

        guess_map
    }

    fn solve(
        guess_map: HashMap<usize, Vec<usize>>,
        mut fixed_map: HashMap<usize, usize>,
    ) -> Option<HashMap<usize, usize>> {
        // if the guess_map is empty, then we're done!
        if guess_map.is_empty() {
            return Some(fixed_map);
        }

        // check if there is a possible map with only a single possibility, otherwise,
        // just guess.
        let mut guess: Option<(usize, usize)> = None;
        for (field, cols) in guess_map.iter() {
            if cols.len() == 1 {
                guess = Some((*field, cols[0]));
            }
        }

        if let Some((field, col)) = guess {
            // check if the "guaranteed true" possibility conflicts with the fixed_map
            if fixed_map.contains_key(&field) {
                return None;
            }

            // if not, then recurse!
            if fixed_map.insert(field, col).is_some() {
                panic!("I messed up my algo");
            };

            let guess_map = fixup_guess_map(guess_map, (field, col));
            return solve(guess_map, fixed_map);
        }

        // otherwise, we have to make some guesses...

        for (field, col) in guess_map
            .iter()
            .flat_map(|(field, cols)| core::iter::repeat(*field).zip(cols.iter().copied()))
        {
            #[allow(clippy::map_entry)]
            if !fixed_map.contains_key(&field) {
                // recurse on the guess
                let mut fixed_map = fixed_map.clone();
                fixed_map.insert(field, col).unwrap();
                let guess_map = fixup_guess_map(guess_map.clone(), (field, col));

                if let Some(soln) = solve(guess_map, fixed_map) {
                    return Some(soln);
                }
            }
        }

        // no dice
        None
    }

    let solution = solve(guess_map, HashMap::new()).ok_or("could not find a solution")?;

    // we only care about the fields that start with "departure"
    let soln = fields
        .iter()
        .enumerate()
        .filter(|(_, (name, _, _))| name.starts_with("departure"))
        .map(|(i, _)| my_ticket[solution[&i]])
        .product();

    Ok(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 71 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = "
departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_2;
        let expected = { 11 * 12 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
