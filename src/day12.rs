use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

enum Act {
    Dir(Dir, isize),
    Rotate(isize), // normalized clockwise
    Forward(isize),
}

impl std::str::FromStr for Act {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> DynResult<Act> {
        let (act, val) = s.split_at(1);
        let val = val.parse::<isize>()?;
        let act = act.chars().next().ok_or("invalid input")?;

        if val < 0 {
            return Err("vals must be positive integers".into());
        }

        if matches!(act, 'R' | 'L') && val % 90 != 0 {
            return Err("degrees must be multiples of 90".into());
        }

        let act = match act {
            'N' => Act::Dir(Dir::N, val),
            'S' => Act::Dir(Dir::S, val),
            'E' => Act::Dir(Dir::E, val),
            'W' => Act::Dir(Dir::W, val),
            'R' => Act::Rotate(val),
            'L' => Act::Rotate(-val + 360),
            'F' => Act::Forward(val),
            _ => return Err("invalid action".into()),
        };
        Ok(act)
    }
}

trait Ship {
    fn do_act(&mut self, act: Act) -> DynResult<()>;
    fn manhattan_dist(&self) -> usize;

    fn run(&mut self, acts: impl Iterator<Item = DynResult<Act>>) -> DynResult<usize> {
        for act in acts {
            self.do_act(act?)?
        }
        Ok(self.manhattan_dist())
    }
}

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input.split('\n').map(|ln| ln.parse::<Act>())
    }};
}

struct NaiveShip {
    pos: (isize, isize),
    dir: Dir,
}

impl Ship for NaiveShip {
    fn do_act(&mut self, act: Act) -> DynResult<()> {
        match act {
            Act::Dir(dir, val) => match dir {
                Dir::N => self.pos.1 += val,
                Dir::S => self.pos.1 -= val,
                Dir::E => self.pos.0 += val,
                Dir::W => self.pos.0 -= val,
            },
            Act::Rotate(degs) => {
                const DIRS_CLOCKWISE: &[Dir] = &[Dir::N, Dir::E, Dir::S, Dir::W];
                self.dir = DIRS_CLOCKWISE[(self.dir as usize + degs as usize / 90) % 4]
            }
            Act::Forward(val) => self.do_act(Act::Dir(self.dir, val))?,
        }
        Ok(())
    }

    fn manhattan_dist(&self) -> usize {
        (self.pos.0.abs() + self.pos.1.abs()) as usize
    }
}

struct WaypointShip {
    pos: (isize, isize),
    waypoint: (isize, isize),
}

impl Ship for WaypointShip {
    fn do_act(&mut self, act: Act) -> DynResult<()> {
        match act {
            Act::Dir(dir, val) => match dir {
                Dir::N => self.waypoint.1 += val,
                Dir::S => self.waypoint.1 -= val,
                Dir::E => self.waypoint.0 += val,
                Dir::W => self.waypoint.0 -= val,
            },
            Act::Rotate(degs) => {
                let waypoint = self.waypoint;
                let steps = (degs / 90) % 4;
                match steps {
                    0 => {}
                    1 => (self.waypoint = (waypoint.1, -waypoint.0)),
                    2 => (self.waypoint = (-waypoint.0, -waypoint.1)),
                    3 => (self.waypoint = (-waypoint.1, waypoint.0)),
                    _ => unreachable!(),
                }
            }
            Act::Forward(val) => {
                self.pos.0 += self.waypoint.0 * val;
                self.pos.1 += self.waypoint.1 * val;
            }
        }
        Ok(())
    }

    fn manhattan_dist(&self) -> usize {
        (self.pos.0.abs() + self.pos.1.abs()) as usize
    }
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let mut ship = NaiveShip {
        pos: (0, 0),
        dir: Dir::E,
    };
    ship.run(input)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<usize> {
    let input = munge_input!(input);
    let mut ship = WaypointShip {
        pos: (0, 0),
        waypoint: (10, 1),
    };
    ship.run(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
F10
N3
F7
R90
F11
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = { 25 };
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = { 286 };
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
