use crate::prelude::*;

#[derive(Debug)]
pub enum InstrParseError {
    MissingOpcode,
    MissingData,
    InvalidData(core::num::ParseIntError),
    InvalidOpcode(String),
}

impl core::fmt::Display for InstrParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InstrParseError {}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

impl Instr {
    pub fn parse(asm: &str) -> Result<Instr, InstrParseError> {
        use InstrParseError::*;

        let mut asm = asm.split(' ');
        let op = asm.next().ok_or(MissingOpcode)?;
        let data = asm.next().ok_or(MissingData)?;

        let instr = match op {
            "nop" => Instr::Nop(data.parse().map_err(InvalidData)?),
            "acc" => Instr::Acc(data.parse().map_err(InvalidData)?),
            "jmp" => Instr::Jmp(data.parse().map_err(InvalidData)?),
            _ => return Err(InvalidOpcode(op.into())),
        };

        Ok(instr)
    }
}

pub struct Vm {
    pub program: Vec<Instr>,

    pub pc: usize,
    pub acc: i64,
}

impl Vm {
    pub fn new(asm: &str) -> Result<Vm, InstrParseError> {
        let program = asm
            .split('\n')
            .map(Instr::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Vm {
            program,
            pc: 0,
            acc: 0,
        })
    }

    pub fn cycle(&mut self) -> DynResult<()> {
        if self.pc >= self.program.len() {
            return Err(format!("PC is out of bounds: {}", self.pc).into());
        }

        match self.program[self.pc] {
            Instr::Nop(_) => {}
            Instr::Acc(v) => self.acc += v,
            Instr::Jmp(offset) => self.pc = self.pc.wrapping_add((offset - 1) as usize),
        }

        self.pc = self.pc.wrapping_add(1);
        Ok(())
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }

    /// Returns true if the program terminates, or false if a loop was detected.
    pub fn run_with_loop_detect(&mut self) -> DynResult<bool> {
        let mut v = std::collections::HashSet::new();
        loop {
            if v.contains(&self.pc) {
                return Ok(false);
            }
            v.insert(self.pc);

            if self.pc == self.program.len() {
                return Ok(true);
            }

            self.cycle()?;
        }
    }
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<i64> {
    let mut vm = Vm::new(input)?;

    let terminates = vm.run_with_loop_detect()?;
    assert!(!terminates, "input unexpectedly terminated");

    Ok(vm.acc)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<i64> {
    let mut vm = Vm::new(input)?;

    for i in 0..vm.program.len() {
        let bak = vm.program[i];
        vm.program[i] = match vm.program[i] {
            Instr::Acc(_) => continue,
            Instr::Nop(offset) => Instr::Jmp(offset),
            Instr::Jmp(offset) => Instr::Nop(offset),
        };

        if vm.run_with_loop_detect()? {
            return Ok(vm.acc);
        }

        vm.reset();
        vm.program[i] = bak;
    }

    Err("could not find a valid mutation".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    #[test]
    fn q1_e1() {
        let input = EXAMPLE_1;
        let expected = 5;
        let q = q1;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }

    #[test]
    fn q2_e1() {
        let input = EXAMPLE_1;
        let expected = 8;
        let q = q2;

        assert_eq!(q(input.trim(), &[]).unwrap(), expected);
    }
}
