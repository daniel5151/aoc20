use crate::prelude::*;

struct Mask {
    /// positions of 'X'
    float: u64,
    /// positions of either '0' or '1'
    fixed: u64,
    /// positions of '1'
    val: u64,
}

enum Instr {
    Mask(Mask),
    Mem { addr: u64, val: u64 },
}

fn parse_mask(s: &str) -> DynResult<Mask> {
    s.split(" = ")
        .nth(1)
        .ok_or("missing bitmask")?
        .chars()
        .try_fold(
            Mask {
                float: 0,
                fixed: 0,
                val: 0,
            },
            |mut mask, b| -> DynResult<_> {
                mask.float <<= 1;
                mask.fixed <<= 1;
                mask.val <<= 1;
                match b {
                    'X' => mask.float |= 1,
                    '0' => mask.fixed |= 1,
                    '1' => {
                        mask.fixed |= 1;
                        mask.val |= 1;
                    }
                    _ => return Err("invalid mask char".into()),
                };
                Ok(mask)
            },
        )
}

fn parse_mem(s: &str) -> DynResult<(u64, u64)> {
    let mut s = s.split(" = ");
    let addr = s
        .next()
        .unwrap()
        .split(|c| matches!(c, '[' | ']'))
        .nth(1)
        .ok_or("invalid memwrite line")?
        .parse::<u64>()
        .map_err(|_| "could not parse memwrite addr")?;
    let val = s
        .next()
        .ok_or("missing memwrite val")?
        .parse::<u64>()
        .map_err(|_| "could not parse memwrite val")?;
    Ok((addr, val))
}

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        let instrs = input.split('\n').map(|ln| -> DynResult<_> {
            if ln.starts_with("mask") {
                Ok(Instr::Mask(parse_mask(ln)?))
            } else {
                let (addr, val) = parse_mem(ln)?;
                Ok(Instr::Mem { addr, val })
            }
        });
        instrs
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<u64> {
    let input = munge_input!(input);

    let mut mask = Mask {
        float: 0,
        fixed: 0,
        val: 0,
    };
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for instr in input {
        let instr = instr?;
        match instr {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Mem { addr, val } => {
                mem.insert(addr, (val & !mask.fixed) | mask.val);
            }
        }
    }

    let memsum = mem.into_iter().map(|(_, v)| v).sum();
    Ok(memsum)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<u64> {
    let input = munge_input!(input);

    let mut mask = Mask {
        float: 0,
        fixed: 0,
        val: 0,
    };
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for instr in input {
        let instr = instr?;
        match instr {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Mem { addr, val } => {
                let set_bits = {
                    let mut v = Vec::new(); // could be a fixed-size array
                    let mut f = mask.float;
                    while f != 0 {
                        v.push(f.trailing_zeros());
                        f = f & (f - 1);
                    }
                    v
                };

                let float_vals = (0..=set_bits.len())
                    .flat_map(|c| set_bits.iter().combinations(c))
                    .map(|set| set.into_iter().fold(0, |a, i| a | (1 << i)));

                for float_val in float_vals {
                    let addr = (addr & !mask.float) | mask.val | float_val;
                    mem.insert(addr, val);
                }
            }
        }
    }

    let memsum = mem.into_iter().map(|(_, v)| v).sum();
    Ok(memsum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 165 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_2;
        let expected = { 208 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
