use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut map = HashSet::new();
        for (y, ln) in input.split('\n').enumerate() {
            for (x, c) in ln.chars().enumerate() {
                if c == '#' {
                    map.insert((x as isize, y as isize));
                }
            }
        }
        map
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut curr = input
        .into_iter()
        .map(|(x, y)| (x, y, 0))
        .collect::<HashSet<_>>();
    let mut next = HashSet::new();

    for _ in 0..6 {
        // calculate bounds
        let bounds = (|| {
            let x = curr.iter().map(|(x, _, _)| *x).minmax().into_option()?;
            let y = curr.iter().map(|(_, y, _)| *y).minmax().into_option()?;
            let z = curr.iter().map(|(_, _, z)| *z).minmax().into_option()?;
            Some((x, y, z))
        })()
        .ok_or("invalid input")?;

        let cords = ((bounds.0 .0 - 1)..=(bounds.0 .1 + 1))
            .cartesian_product((bounds.1 .0 - 1)..=(bounds.1 .1 + 1))
            .cartesian_product((bounds.2 .0 - 1)..=(bounds.2 .1 + 1))
            .map(|((x, y), z)| (x, y, z));

        let deltas = (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|((x, y), z)| (x, y, z))
            .filter(|c| *c != (0, 0, 0));

        for (x, y, z) in cords {
            let neighbors = deltas
                .clone()
                .filter(|(dx, dy, dz)| curr.contains(&(x + dx, y + dy, z + dz)))
                .count();

            if matches!(
                (curr.contains(&(x, y, z)), neighbors),
                (true, 2) | (true, 3) | (false, 3)
            ) {
                next.insert((x, y, z));
            }
        }

        curr = next.clone();
        next.clear();
    }

    Ok(curr.len())
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut curr = input
        .into_iter()
        .map(|(x, y)| (x, y, 0, 0))
        .collect::<HashSet<_>>();
    let mut next = HashSet::new();

    for _ in 0..6 {
        // calculate bounds
        let bounds = (|| {
            let x = curr.iter().map(|(x, _, _, _)| *x).minmax().into_option()?;
            let y = curr.iter().map(|(_, y, _, _)| *y).minmax().into_option()?;
            let z = curr.iter().map(|(_, _, z, _)| *z).minmax().into_option()?;
            let w = curr.iter().map(|(_, _, _, w)| *w).minmax().into_option()?;
            Some((x, y, z, w))
        })()
        .ok_or("invalid input")?;

        let cords = ((bounds.0 .0 - 1)..=(bounds.0 .1 + 1))
            .cartesian_product((bounds.1 .0 - 1)..=(bounds.1 .1 + 1))
            .cartesian_product((bounds.2 .0 - 1)..=(bounds.2 .1 + 1))
            .cartesian_product((bounds.3 .0 - 1)..=(bounds.3 .1 + 1))
            .map(|(((x, y), z), w)| (x, y, z, w));

        let deltas = (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|(((x, y), z), w)| (x, y, z, w))
            .filter(|c| *c != (0, 0, 0, 0));

        for (x, y, z, w) in cords {
            let neighbors = deltas
                .clone()
                .filter(|(dx, dy, dz, dw)| curr.contains(&(x + dx, y + dy, z + dz, w + dw)))
                .count();

            if matches!(
                (curr.contains(&(x, y, z, w)), neighbors),
                (true, 2) | (true, 3) | (false, 3)
            ) {
                next.insert((x, y, z, w));
            }
        }

        curr = next.clone();
        next.clear();
    }

    Ok(curr.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
.#.
..#
###
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 112 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 848 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
