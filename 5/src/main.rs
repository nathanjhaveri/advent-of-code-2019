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

type ResultX<T> = Result<T, Box<dyn Error>>;
type Ops = Vec<i32>;

fn parse_op(coded_op: i32) -> (i32, OpMode, OpMode, OpMode) {
    let op = nth_digit(0, coded_op) + 10 * nth_digit(1, coded_op);
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

fn compute<RF, WF>(ops: &mut Ops, read: RF, mut write: WF)
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
        let (op, mode1, mode2, mode3) = parse_op(coded_op);
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

fn init_ops(input: String) -> ResultX<Vec<i32>> {
    let mut ops: Vec<i32> = Vec::new();
    for op_str in input.split(",") {
        let op: i32 = op_str.parse()?;
        ops.push(op);
    }

    Ok(ops)
}

fn main() -> ResultX<()> {
    let input = read_to_string("input.txt")?;
    let mut ops = init_ops(input)?;

    let read = || 5;
    let write = |val: i32| println!("output: {}", val);
    compute(&mut ops, read, write);

    println!("{:?}", ops);

    Ok(())
}

#[test]
fn input_is_8_true() -> ResultX<()> {
    let mut ops = init_ops("3,9,8,9,10,9,4,9,99,-1,8".to_string())?;
    let read = || 8;
    let mut result: Option<i32> = { None };
    let write = |val: i32| result = Some(val);
    compute(&mut ops, read, write);
    assert_eq!(result, Some(1));
    Ok(())
}
