use crate::prelude::*;

const REQ_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; //, "cid"];

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
            .split("\n\n")
            .map(|praw| {
                praw.split('\n')
                    .flat_map(|ln| ln.split(' '))
                    .map(|f| {
                        let mut f = f.split(':');
                        let k = f.next().unwrap();
                        let v = f.next().unwrap();
                        (k, v)
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>()
    }};
}

fn validate<'a>(p: HashMap<&'a str, &'a str>) -> DynResult<()> {
    for f in REQ_FIELDS {
        if !p.contains_key(f) {
            return Err("".into());
        }
    }

    const EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for (k, v) in p {
        let ok = match k {
            "byr" => (1920..=2002).contains(&v.parse::<usize>()?),
            "iyr" => (2010..=2020).contains(&v.parse::<usize>()?),
            "eyr" => (2020..=2030).contains(&v.parse::<usize>()?),
            "hgt" => {
                if let Some(n) = v.strip_suffix("cm") {
                    (150..=193).contains(&n.parse::<usize>()?)
                } else if let Some(n) = v.strip_suffix("in") {
                    (59..=76).contains(&n.parse::<usize>()?)
                } else {
                    false
                }
            }
            "hcl" => match v.strip_prefix('#') {
                None => false,
                Some(v) => v.chars().all(|c| matches!(c, 'a'..='f' | '0'..='9')),
            },
            "ecl" => EYE_COLOR.contains(&v),
            "pid" => v.len() == 9 && v.parse::<usize>().is_ok(),
            "cid" => true,
            _ => return Err("".into()),
        };

        if !ok {
            return Err("".into());
        }
    }

    Ok(())
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    eprintln!("{:#?}", input);

    let mut valid = 0;
    for p in input {
        valid += validate(p).is_ok() as usize;
    }

    Ok(valid)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut valid = 0;
    for p in input {
        valid += validate(p).is_ok() as usize;
    }

    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 2;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    const EXAMPLE_2: &str = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    const EXAMPLE_3: &str = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_2;
        let expected = 0;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e2() {
        let input = EXAMPLE_3;
        let expected = 4;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
