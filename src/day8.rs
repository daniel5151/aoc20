use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = $input;
        input
    }};
}

pub fn q1(input: &str, _args: &[&str]) -> DynResult<i64> {
    let input = munge_input!(input);
    let mut vm = vm::Vm::new(input)?;

    let terminates = vm.run_with_loop_detect()?;
    assert!(!terminates, "input unexpectedly terminated");

    Ok(vm.acc)
}

pub fn q2(input: &str, _args: &[&str]) -> DynResult<i64> {
    let input = munge_input!(input);
    let mut vm = vm::Vm::new(input)?;

    for i in 0..vm.program.len() {
        let bak = vm.program[i];
        vm.program[i] = match vm.program[i] {
            vm::Instr::Acc(_) => continue,
            vm::Instr::Nop(offset) => vm::Instr::Jmp(offset),
            vm::Instr::Jmp(offset) => vm::Instr::Nop(offset),
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
