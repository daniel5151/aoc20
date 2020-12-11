use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split('\n')
            .map(|ln| ln.as_bytes().to_vec())
            .collect::<Vec<_>>()
    }};
}

fn count_stable(
    input: Vec<Vec<u8>>,
    visible: impl Fn(&[Vec<u8>], (usize, usize)) -> usize,
    empty_cuttoff: usize,
) -> DynResult<usize> {
    let mut curr = input;
    let mut next = curr.clone(); // clone to maintain dimensions

    loop {
        // eprintln!();
        let mut stable = true;
        for (r, row) in curr.iter().enumerate() {
            // eprintln!();
            for (c, seat) in row.iter().enumerate() {
                // eprint!("{}", *seat as char);
                next[r][c] = *seat; // copy over state from curr
                match seat {
                    b'L' => {
                        if visible(&curr, (r, c)) == 0 {
                            next[r][c] = b'#';
                            stable = false;
                        }
                    }
                    b'#' => {
                        if visible(&curr, (r, c)) >= empty_cuttoff {
                            next[r][c] = b'L';
                            stable = false;
                        }
                    }
                    b'.' => continue,
                    _ => return Err("invalid input".into()),
                }
            }
        }

        std::mem::swap(&mut curr, &mut next);

        if stable {
            break;
        }
    }

    let occupied = curr
        .into_iter()
        .flat_map(|v| v.into_iter())
        .filter(|c| *c == b'#')
        .count();

    Ok(occupied)
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    fn adj_count(input: &[Vec<u8>], (r, c): (usize, usize)) -> usize {
        let rows = r.saturating_sub(1)..=(r.saturating_add(1).min(input.len() - 1));
        let cols = c.saturating_sub(1)..=c.saturating_add(1).min(input[0].len() - 1);
        rows.cartesian_product(cols)
            .filter(|rc| *rc != (r, c))
            .filter(|&(r, c)| input[r][c] == b'#')
            .count()
    }

    let input = munge_input!(input);
    count_stable(input, adj_count, 4)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    // calculate visibility map (i.e: list of positions to check for each seat)
    let mut visibility: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let vecs = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|rc| *rc != (0, 0))
        .collect::<Vec<_>>();
    assert_eq!(vecs.len(), 8);

    for (base_r, row) in input.iter().enumerate() {
        for (base_c, seat) in row.iter().enumerate() {
            match seat {
                b'.' => continue,
                b'L' | b'#' => {}
                _ => return Err("invalid input".into()),
            }

            for &(dr, dc) in vecs.iter() {
                let (mut r, mut c) = (base_r, base_c);
                'raycast: loop {
                    r = r.wrapping_add(dr as usize);
                    c = c.wrapping_add(dc as usize);
                    if let Some(seat) = input.get(r).and_then(|row| row.get(c)) {
                        match seat {
                            b'.' => continue 'raycast,
                            b'L' | b'#' => {
                                visibility.entry((base_r, base_c)).or_default().push((r, c));
                                break 'raycast;
                            }
                            _ => return Err("invalid input".into()),
                        }
                    }
                    break;
                }
            }
        }
    }

    // now that it's calculated, freeze it
    let visibility = visibility;

    let visible_count = |input: &[Vec<u8>], (r, c): (usize, usize)| -> usize {
        visibility
            .get(&(r, c))
            .unwrap()
            .iter()
            .filter(|&(r, c)| input[*r][*c] == b'#')
            .count()
    };

    count_stable(input, visible_count, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 37 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 26 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
