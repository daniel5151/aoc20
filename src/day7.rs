use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input.split('\n').map(|ln| -> Option<_> {
            let mut s = ln.split(" bags contain ");
            let mut p = s.next()?.split(' ');
            let p = (p.next()?, p.next()?);

            let c = s
                .next()?
                .strip_suffix('.')?
                .split(", ")
                .map(|e| -> Option<_> {
                    let mut e = e.strip_suffix("bag").or(e.strip_suffix("bags"))?.split(' ');
                    let n = match e.next()? {
                        "no" => return Some((0, ("", ""))),
                        n => n.parse::<usize>().ok()?,
                    };
                    let a = e.next()?;
                    let c = e.next()?;
                    Some((n, (a, c)))
                })
                .filter(|e| e.map(|(n, _)| n == 0) != Some(true))
                .collect::<Option<Vec<(_, _)>>>()?;

            Some((p, c))
        })
    }};
}

type Bag<'a> = (&'a str, &'a str);
type Bags<'a> = HashMap<Bag<'a>, Vec<(usize, Bag<'a>)>>;

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    // HashMap<Bag, HashSet<Parent Bag>>, discarding the number
    let mut g: HashMap<_, HashSet<_>> = HashMap::new();
    for p in input {
        let (p, c) = p.ok_or("invalid input")?;
        for (_, b) in c {
            g.entry(b).or_default().insert(p);
        }
    }

    let initial = g.get(&("shiny", "gold")).unwrap();

    let mut v = initial.iter().collect::<HashSet<_>>();
    let mut q = initial.iter().collect::<Vec<_>>();
    let mut total = 0;

    while let Some(k) = q.pop() {
        total += 1;
        if let Some(bags) = g.get(&k) {
            for bag in bags {
                if v.contains(bag) {
                    continue;
                }
                v.insert(bag);
                q.push(bag)
            }
        }
    }

    Ok(total)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let bags = input.collect::<Option<Bags>>().ok_or("invalid input")?;

    fn num_bags(bags: &Bags, parent_bag: &Bag) -> usize {
        bags.get(parent_bag)
            .unwrap()
            .iter()
            .map(|(n, bag)| n * (1 + num_bags(bags, bag)))
            .sum::<usize>()
    };

    Ok(num_bags(&bags, &("shiny", "gold")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    const EXAMPLE_2: &str = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 4;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = 32;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e2() {
        let input = EXAMPLE_2;
        let expected = 126;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
