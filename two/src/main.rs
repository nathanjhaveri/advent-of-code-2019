use std::error::Error;
use std::fs::read_to_string;

const ADD: u32 = 1;
const MULTIPLY: u32 = 2;
const HCF: u32 = 99;

fn compute(ops: &mut Vec<u32>) {
    // ip - instruction pointer
    // a - first register
    // b - second register
    // r - result register
    let mut ip = 0;  // Instruction pointer

    let mut op = ops[ip];

    while op != HCF {
        let ap = ops[ip + 1] as usize;
        let bp = ops[ip + 2] as usize;
        let rp = ops[ip + 3] as usize;

        let x = match op {
            ADD => ops[ap] + ops[bp],
            MULTIPLY => ops[ap] * ops[bp],
            _ => panic!("Unrecognized instruction")
        };

        ops[rp] = x;
        ip += 4;
        op = ops[ip];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;
    let mut ops: Vec<u32> = Vec::new();
    for op_str in input.split(",") {
        let op: u32 = op_str.parse()?;
        ops.push(op);
    }

    ops[1] = 12;
    ops[2] = 2;

    compute(&mut ops);


    println!("end");
    println!("{}", ops[0]);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
    }
}