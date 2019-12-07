use std::error::Error;
use std::fs::read_to_string;

// OpCodes - would be better as an enum, but needs lots
// of annoying conversion logic
const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMP_IF_TRUE: i32 = 5;
const JUMP_IF_FALSE: i32 = 6;
const LESS_THAN: i32 = 7;
const EQUALS: i32 = 8;
const HCF: i32 = 99;

enum OpMode {
    Positional = 0,
    Immediate = 1,
}

impl From<i32> for OpMode {
    fn from(num: i32) -> OpMode {
        match num {
            0 => OpMode::Positional,
            1 => OpMode::Immediate,
            _ => panic!("invalid opmode"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Ops = [i32];

fn parse_op(coded_op: i32) -> (i32, OpMode, OpMode, OpMode) {
    let op = coded_op % 100;
    let mode1 = OpMode::from(nth_digit(2, coded_op));
    let mode2 = OpMode::from(nth_digit(3, coded_op));
    let mode3 = OpMode::from(nth_digit(4, coded_op));

    (op, mode1, mode2, mode3)
}

fn resolve_op(ops: &Ops, index: usize, mode: OpMode) -> i32 {
    match mode {
        OpMode::Immediate => ops[index],
        OpMode::Positional => ops[ops[index] as usize],
    }
}

pub fn compute<RF, WF>(ops: &mut Ops, read: RF, mut write: WF)
where
    RF: Fn() -> i32,
    WF: FnMut(i32),
{
    // ip - instruction pointer
    // a - first register
    // b - second register
    // r - result register
    let mut ip = 0; // Instruction pointer

    loop {
        let coded_op = ops[ip];
        let (op, mode1, mode2, _mode3) = parse_op(coded_op);
        ip += 1;

        if op == HCF {
            return;
        }

        match op {
            ADD => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;
                let rp = ops[ip] as usize;
                ip += 1;

                ops[rp] = a + b;
            }
            MULTIPLY => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;
                let rp = ops[ip] as usize;
                ip += 1;

                ops[rp] = a * b;
            }
            INPUT => {
                let input_index = ops[ip] as usize;
                ops[input_index] = read();
                ip += 1
            }
            OUTPUT => {
                let input_index = ops[ip] as usize;
                write(ops[input_index]);
                ip += 1;
            }
            JUMP_IF_TRUE => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;

                if a != 0 {
                    ip = b as usize;
                }
            }
            JUMP_IF_FALSE => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;

                if a == 0 {
                    ip = b as usize;
                }
            }
            LESS_THAN => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;

                let rp = ops[ip] as usize;
                ip += 1;

                ops[rp] = if a < b { 1 } else { 0 };
            }
            EQUALS => {
                let a = resolve_op(ops, ip, mode1);
                ip += 1;
                let b = resolve_op(ops, ip, mode2);
                ip += 1;

                let rp = ops[ip] as usize;
                ip += 1;

                ops[rp] = if a == b { 1 } else { 0 };
            }
            _ => panic!("Unrecognized instruction {}", op),
        };
    }
}

fn nth_digit(n: u32, number: i32) -> i32 {
    (number / 10i32.pow(n)) % 10
}

pub fn init_ops(input: &str) -> Result<Vec<i32>> {
    let mut ops: Vec<i32> = Vec::new();
    for op_str in input.split(',') {
        let op: i32 = op_str.parse()?;
        ops.push(op);
    }

    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PROGRAM_2: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,10,19,23,1,6,23,27,1,5,27,31,1,10,31,35,2,10,35,39,1,39,5,43,2,43,6,47,2,9,47,51,1,51,5,55,1,5,55,59,2,10,59,63,1,5,63,67,1,67,10,71,2,6,71,75,2,6,75,79,1,5,79,83,2,6,83,87,2,13,87,91,1,91,6,95,2,13,95,99,1,99,5,103,2,103,10,107,1,9,107,111,1,111,6,115,1,115,2,119,1,119,10,0,99,2,14,0,0";
    const PROGRAM_5: &str = "3,225,1,225,6,6,1100,1,238,225,104,0,101,67,166,224,1001,224,-110,224,4,224,102,8,223,223,1001,224,4,224,1,224,223,223,2,62,66,224,101,-406,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,76,51,225,1101,51,29,225,1102,57,14,225,1102,64,48,224,1001,224,-3072,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1001,217,90,224,1001,224,-101,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1101,57,55,224,1001,224,-112,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1102,5,62,225,1102,49,68,225,102,40,140,224,101,-2720,224,224,4,224,1002,223,8,223,1001,224,4,224,1,223,224,223,1101,92,43,225,1101,93,21,225,1002,170,31,224,101,-651,224,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1,136,57,224,1001,224,-138,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,11,85,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,226,226,224,102,2,223,223,1006,224,329,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,344,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,359,101,1,223,223,1008,226,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,226,224,102,2,223,223,1006,224,404,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,419,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,434,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,449,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,479,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,494,1001,223,1,223,1108,677,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,524,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,677,677,224,102,2,223,223,1005,224,554,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,584,1001,223,1,223,1108,677,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1008,677,226,224,102,2,223,223,1006,224,614,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,629,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,644,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,659,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226";

    fn run_program(program: &str, input: i32) -> i32 {
        let mut ops = init_ops(program).unwrap();
        let read = || input;
        let mut result: Option<i32> = { None };
        let write = |val: i32| result = Some(val);
        compute(&mut ops, read, write);

        result.expect("No ouput")
    }

    #[test]
    fn five_2() {
        let output = run_program(PROGRAM_5, 5);
        assert_eq!(5_893_654, output);
    }

    #[test]
    fn five_1() {
        let output = run_program(PROGRAM_5, 1);
        assert_eq!(9_219_874, output);
    }

    #[test]
    fn two_1() {
        let mut ops = init_ops(PROGRAM_2).unwrap();
        ops[1] = 12;
        ops[2] = 2;
        let read = || 1;
        let write = |_: i32| ();
        compute(&mut ops, read, write);

        assert_eq!(5_482_655, ops[0]);
    }

    #[test]
    fn input_is_8_true() {
        let output = run_program("3,9,8,9,10,9,4,9,99,-1,8", 8);
        assert_eq!(output, 1);
    }

    #[test]
    fn input_is_not_8_false() {
        let output = run_program("3,9,8,9,10,9,4,9,99,-1,8", 0);
        assert_eq!(output, 0);
    }

    #[test]
    fn equal_8_immediate() {
        let program = "3,3,1108,-1,8,3,4,3,99";
        let equal = run_program(program, 8);
        let notequal = run_program(program, 1);
        assert_eq!(equal, 1);
        assert_eq!(notequal, 0);
    }

    #[test]
    fn input_less_than_8() {
        let program = "3,3,1107,-1,8,3,4,3,99";
        let lessthan = run_program(program, 1);
        let morethan = run_program(program, 9);
        assert_eq!(lessthan, 1);
        assert_eq!(morethan, 0);
    }
}