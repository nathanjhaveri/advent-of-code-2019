use std::error::Error;

pub type OpSize = i64;
const MEMORY_SIZE: usize = 1500;

// OpCodes - would be better as an enum, but needs lots
// of annoying conversion logic
const ADD: OpSize = 1;
const MULTIPLY: OpSize = 2;
const INPUT: OpSize = 3;
const OUTPUT: OpSize = 4;
const JUMP_IF_TRUE: OpSize = 5;
const JUMP_IF_FALSE: OpSize = 6;
const LESS_THAN: i64 = 7;
const EQUALS: i64 = 8;
const ADJUST_RELATIVE_BASE: OpSize = 9;
const HCF: OpSize = 99;

#[derive(PartialEq, Debug)]
enum OpMode {
    Positional = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<OpSize> for OpMode {
    fn from(num: OpSize) -> OpMode {
        match num {
            0 => OpMode::Positional,
            1 => OpMode::Immediate,
            2 => OpMode::Relative,
            _ => panic!("invalid opmode"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Ops = Vec<OpSize>;

fn parse_op(coded_op: OpSize) -> (OpSize, OpMode, OpMode, OpMode) {
    let op = coded_op % 100;
    let mode1 = OpMode::from(nth_digit(2, coded_op));
    let mode2 = OpMode::from(nth_digit(3, coded_op));
    let mode3 = OpMode::from(nth_digit(4, coded_op));

    (op, mode1, mode2, mode3)
}

pub struct IntCode {
    ops: Ops,
    ip: usize,
    input_pos: usize,
    relative_base: OpSize,
    input: Vec<OpSize>,
    output: Vec<OpSize>,
}

impl IntCode {
    pub fn new(program: &str) -> IntCode {
        let ops = init_ops(program).expect("Failed parsing program");
        IntCode::init(ops)
    }

    pub fn init(ops: Ops) -> IntCode {
        IntCode {
            ops,
            ip: 0,
            input_pos: 0,
            relative_base: 0,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    pub fn input(&mut self, num: OpSize) {
        self.input.push(num);
    }

    pub fn last_output(&self) -> OpSize {
        *self.output.last().expect("No output")
    }

    pub fn output(&self) -> &Vec<OpSize> {
        &self.output
    }

    pub fn run(&mut self) {
        while let Some(_) = self.compute_output() {}
    }

    pub fn compute_output(&mut self) -> Option<OpSize> {
        loop {
            let coded_op = self.ops[self.ip];
            let (op, mode1, mode2, mode3) = parse_op(coded_op);
            self.ip += 1;

            if op == HCF {
                return None;
            }

            match op {
                ADD => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;
                    let rp = self.op_ptr(mode3);
                    self.ip += 1;

                    self.ops[rp] = a + b;
                }
                MULTIPLY => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;
                    let rp = self.op_ptr(mode3);
                    self.ip += 1;

                    self.ops[rp] = a * b;
                }
                INPUT => {
                    if mode1 == OpMode::Immediate {
                        panic!("Illigal op - immediate mode imput storage");
                    }

                    let input_storage_index = self.op_ptr(mode1);
                    self.ip += 1;
                    self.ops[input_storage_index] = self.input[self.input_pos];
                    self.input_pos += 1;
                }
                OUTPUT => {
                    let out = self.resolve_op(mode1);
                    self.ip += 1;
                    self.output.push(out);
                    return Some(out);
                }
                JUMP_IF_TRUE => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;

                    if a != 0 {
                        self.ip = b as usize;
                    }
                }
                JUMP_IF_FALSE => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;

                    if a == 0 {
                        self.ip = b as usize;
                    }
                }
                LESS_THAN => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;

                    let rp = self.op_ptr(mode3);
                    self.ip += 1;

                    self.ops[rp] = if a < b { 1 } else { 0 };
                }
                EQUALS => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    let b = self.resolve_op(mode2);
                    self.ip += 1;

                    let rp = self.op_ptr(mode3);
                    self.ip += 1;

                    self.ops[rp] = if a == b { 1 } else { 0 };
                }
                ADJUST_RELATIVE_BASE => {
                    let a = self.resolve_op(mode1);
                    self.ip += 1;
                    self.relative_base += a;
                }
                _ => panic!("Unrecognized instruction {}", op),
            };
        }
    }

    fn op_ptr(&self, mode: OpMode) -> usize {
        let op = self.ops[self.ip];
        match mode {
            OpMode::Positional => op as usize,
            OpMode::Immediate => self.ip,
            OpMode::Relative => (self.relative_base + op) as usize,
        }
    }

    fn resolve_op(&self, mode: OpMode) -> OpSize {
        let op = self.ops[self.ip];
        match mode {
            OpMode::Positional => self.ops[op as usize],
            OpMode::Immediate => self.ops[self.ip],
            OpMode::Relative => {
                let index: usize = (self.relative_base + op) as usize;
                self.ops[index]
            }
        }
    }
}

fn nth_digit(n: u32, number: OpSize) -> OpSize {
    (number / 10i64.pow(n)) % 10
}

pub fn init_ops(input: &str) -> Result<Ops> {
    let mut ops: Vec<OpSize> = Vec::with_capacity(MEMORY_SIZE);
    for op_str in input.split(',') {
        let op: OpSize = op_str.trim().parse()?;
        ops.push(op);
    }

    for _ in ops.len()..MEMORY_SIZE {
        ops.push(0);
    }

    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PROGRAM_2: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,10,19,23,1,6,23,27,1,5,27,31,1,10,31,35,2,10,35,39,1,39,5,43,2,43,6,47,2,9,47,51,1,51,5,55,1,5,55,59,2,10,59,63,1,5,63,67,1,67,10,71,2,6,71,75,2,6,75,79,1,5,79,83,2,6,83,87,2,13,87,91,1,91,6,95,2,13,95,99,1,99,5,103,2,103,10,107,1,9,107,111,1,111,6,115,1,115,2,119,1,119,10,0,99,2,14,0,0";
    const PROGRAM_5: &str = "3,225,1,225,6,6,1100,1,238,225,104,0,101,67,166,224,1001,224,-110,224,4,224,102,8,223,223,1001,224,4,224,1,224,223,223,2,62,66,224,101,-406,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,76,51,225,1101,51,29,225,1102,57,14,225,1102,64,48,224,1001,224,-3072,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1001,217,90,224,1001,224,-101,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1101,57,55,224,1001,224,-112,224,4,224,102,8,223,223,1001,224,7,224,1,223,224,223,1102,5,62,225,1102,49,68,225,102,40,140,224,101,-2720,224,224,4,224,1002,223,8,223,1001,224,4,224,1,223,224,223,1101,92,43,225,1101,93,21,225,1002,170,31,224,101,-651,224,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1,136,57,224,1001,224,-138,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,11,85,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,226,226,224,102,2,223,223,1006,224,329,1001,223,1,223,1007,226,677,224,1002,223,2,223,1005,224,344,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,359,101,1,223,223,1008,226,226,224,1002,223,2,223,1005,224,374,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,226,224,102,2,223,223,1006,224,404,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,419,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,434,1001,223,1,223,1008,677,677,224,1002,223,2,223,1005,224,449,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,1108,226,677,224,1002,223,2,223,1005,224,479,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,494,1001,223,1,223,1108,677,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,524,1001,223,1,223,7,226,677,224,1002,223,2,223,1005,224,539,1001,223,1,223,8,677,677,224,102,2,223,223,1005,224,554,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,584,1001,223,1,223,1108,677,226,224,102,2,223,223,1006,224,599,1001,223,1,223,1008,677,226,224,102,2,223,223,1006,224,614,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,629,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,644,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,659,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226";
    const PROGRAM_9: &str = "
            1102,34463338,34463338,63,
            1007,63,34463338,63,
            1005,63,53,
            1101,0,3,1000,
            109,988,
            209,12,
            9,1000,
            209,6,
            209,3,
            203,0,
            1008,1000,1,63,
            1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1102,1,39,1013,1102,1,21,1018,1101,0,336,1027,1102,1,38,1012,1101,534,0,1025,1101,539,0,1024,1101,0,380,1023,1102,1,23,1014,1102,29,1,1000,1102,24,1,1019,1102,1,28,1011,1101,339,0,1026,1101,31,0,1005,1102,36,1,1017,1102,26,1,1007,1102,1,407,1028,1101,387,0,1022,1101,0,30,1001,1101,34,0,1010,1102,1,32,1006,1101,0,1,1021,1102,27,1,1008,1102,22,1,1004,1102,1,20,1015,1101,0,37,1016,1101,0,0,1020,1102,1,398,1029,1101,25,0,1009,1101,0,35,1003,1101,33,0,1002,109,27,1206,-6,197,1001,64,1,64,1105,1,199,4,187,1002,64,2,64,109,-22,2107,26,3,63,1005,63,217,4,205,1105,1,221,1001,64,1,64,1002,64,2,64,109,17,21107,40,39,-8,1005,1014,241,1001,64,1,64,1105,1,243,4,227,1002,64,2,64,109,-8,1206,6,261,4,249,1001,64,1,64,1106,0,261,1002,64,2,64,109,-7,2108,24,0,63,1005,63,281,1001,64,1,64,1105,1,283,4,267,1002,64,2,64,109,11,21102,41,1,-3,1008,1015,42,63,1005,63,303,1105,1,309,4,289,1001,64,1,64,1002,64,2,64,109,1,1205,2,327,4,315,1001,64,1,64,1105,1,327,1002,64,2,64,109,10,2106,0,-2,1106,0,345,4,333,1001,64,1,64,1002,64,2,64,109,-15,21102,42,1,3,1008,1017,42,63,1005,63,367,4,351,1105,1,371,1001,64,1,64,1002,64,2,64,109,-1,2105,1,10,1001,64,1,64,1105,1,389,4,377,1002,64,2,64,109,24,2106,0,-9,4,395,1001,64,1,64,1105,1,407,1002,64,2,64,109,-30,1208,-2,32,63,1005,63,427,1001,64,1,64,1106,0,429,4,413,1002,64,2,64,109,2,1201,0,0,63,1008,63,27,63,1005,63,449,1106,0,455,4,435,1001,64,1,64,1002,64,2,64,109,5,21107,43,44,0,1005,1014,473,4,461,1106,0,477,1001,64,1,64,1002,64,2,64,109,-16,1202,3,1,63,1008,63,33,63,1005,63,501,1001,64,1,64,1106,0,503,4,483,1002,64,2,64,109,10,1207,-4,21,63,1005,63,523,1001,64,1,64,1106,0,525,4,509,1002,64,2,64,109,11,2105,1,5,4,531,1106,0,543,1001,64,1,64,1002,64,2,64,109,-8,21101,44,0,5,1008,1016,47,63,1005,63,563,1106,0,569,4,549,1001,64,1,64,1002,64,2,64,109,-13,2102,1,8,63,1008,63,34,63,1005,63,593,1001,64,1,64,1105,1,595,4,575,1002,64,2,64,109,8,1208,-1,31,63,1005,63,617,4,601,1001,64,1,64,1106,0,617,1002,64,2,64,109,-8,2108,33,4,63,1005,63,635,4,623,1105,1,639,1001,64,1,64,1002,64,2,64,109,10,1202,-1,1,63,1008,63,26,63,1005,63,665,4,645,1001,64,1,64,1105,1,665,1002,64,2,64,109,-9,2107,30,1,63,1005,63,685,1001,64,1,64,1105,1,687,4,671,1002,64,2,64,109,25,1205,-4,703,1001,64,1,64,1105,1,705,4,693,1002,64,2,64,109,-19,2101,0,-5,63,1008,63,26,63,1005,63,725,1105,1,731,4,711,1001,64,1,64,1002,64,2,64,109,6,1207,-2,26,63,1005,63,749,4,737,1105,1,753,1001,64,1,64,1002,64,2,64,109,-10,21108,45,46,9,1005,1010,769,1105,1,775,4,759,1001,64,1,64,1002,64,2,64,109,-10,1201,10,0,63,1008,63,30,63,1005,63,801,4,781,1001,64,1,64,1106,0,801,1002,64,2,64,109,21,21108,46,46,3,1005,1015,819,4,807,1106,0,823,1001,64,1,64,1002,64,2,64,109,-4,2102,1,-3,63,1008,63,31,63,1005,63,849,4,829,1001,64,1,64,1106,0,849,1002,64,2,64,109,-5,2101,0,1,63,1008,63,22,63,1005,63,875,4,855,1001,64,1,64,1105,1,875,1002,64,2,64,109,17,21101,47,0,-3,1008,1017,47,63,1005,63,897,4,881,1105,1,901,1001,64,1,64,4,64,99,21101,0,27,1,21102,1,915,0,1105,1,922,21201,1,38480,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1106,0,922,21202,1,1,-1,21201,-2,-3,1,21101,957,0,0,1105,1,922,22201,1,-1,-2,1106,0,968,22101,0,-2,-2,109,-3,2105,1,0";

    fn run_program(program: &str, input: OpSize) -> OpSize {
        let mut computer = IntCode::new(program);
        computer.input(input);
        computer.run();
        println!("output: {:?}", computer.output);
        computer.last_output()
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
        let mut computer = IntCode::new(PROGRAM_2);
        computer.ops[1] = 12;
        computer.ops[2] = 2;
        computer.input(1);
        computer.run();

        assert_eq!(5_482_655, computer.ops[0]);
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

    #[test]
    fn nine_1() {
        // MUL -> [63]
        // LT -> [63]
        // JIT [63], ip = 53
        // [1000] = 3
        // bp += 998
        // bp += [1000], bp =
        // [1008] = 1
        let output = run_program(PROGRAM_9, 1);
        assert_eq!(output, 3_906_448_201);
    }

    #[test]
    fn nine_2() {
        let output = run_program(PROGRAM_9, 2);
        assert_eq!(output, 59785);
    }

    #[test]
    fn nine_example_1() {
        let program = "
            109,1,
            204,-1,
            1001,100,1,100,
            1008,100,16,101,
            1006,101,0,
            99";
        let mut computer = IntCode::new(program);
        computer.run();
        assert_eq!(
            computer.output,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn nine_example_2() {
        let program = "1102,34915192,34915192,7,4,7,99,0";
        let output = run_program(program, 1);
        assert_eq!(output, 34_915_192 * 34_915_192);
    }

    #[test]
    fn nine_example_3() {
        let program = "104,1125899906842624,99";
        let output = run_program(program, 1);
        assert_eq!(output, 1_125_899_906_842_624);
    }

    #[test]
    fn find_first_digit() {
        let number = 43210;
        assert_eq!(nth_digit(0, number), 0);
        assert_eq!(nth_digit(1, number), 1);
        assert_eq!(nth_digit(2, number), 2);
        assert_eq!(nth_digit(3, number), 3);
        assert_eq!(nth_digit(4, number), 4);
    }

    #[test]
    fn verify_parse_op() {
        let coded_op = 109;
        let (op, mode1, _, _) = parse_op(coded_op);
        assert_eq!(op, ADJUST_RELATIVE_BASE);
        assert_eq!(mode1, OpMode::Immediate);
    }
}
