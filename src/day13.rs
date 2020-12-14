use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let mut ln = input.split('\n');
        let time = ln.next().ok_or("missing timestamp")?.parse::<usize>()?;
        let buses = ln
            .next()
            .ok_or("missing buses")?
            .split(',')
            .map(|b| b.parse::<usize>().ok());
        (time, buses)
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (time, buses) = munge_input!(input);

    let valid_buses = buses.filter_map(|b| b);
    let (bus, wait_time) = valid_buses
        .map(|bus| (bus, bus - (time % bus)))
        .min_by_key(|&(_, wait_time)| wait_time)
        .unwrap();

    Ok(bus * wait_time)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let (_, buses) = munge_input!(input);

    // Right, I won't lie, I went down a real rabbit hole thinking there was some
    // easy lowest-common-multiple solution, but nah, that totally didn't work. Got
    // some neat new prelude methods thanks to that diversion, but alas, no answer
    // to this question...
    //
    // But yeah, I ended up looking up the solution on the internet.
    //
    // Chinese Remainder Theorem? Seriously? What the fuck! Sure, I learned about it
    // in my first year of undergrad (shout-out to MATH 135), but fuck me if you
    // expect me to still remember it 5 years later!
    //
    // Well, in any case... here's the solution for the problem which I shamelessly
    // stole from the internet. The best part? It's not even using the Chinese
    // Remainder Theorem lmaoooo.
    //
    // And here's a fantastic visual explanation of how it works:
    // https://www.reddit.com/r/adventofcode/comments/kcl7d2/2020_day_13_part_2_buses_in_a_slot_machine/

    let buses = buses.enumerate().filter_map(|(i, b)| Some((i, b?)));

    let mut ans = 0;
    let mut lcd = 1; // least common divisor
    for (offset, bus) in buses {
        while (ans + offset) % bus != 0 {
            ans += lcd;
        }
        lcd *= bus;
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
939
7,13,x,x,59,x,31,19
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 295 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 1068781 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
