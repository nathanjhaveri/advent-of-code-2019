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
    let mut ip = 0; // Instruction pointer

    let mut op = ops[ip];

    while op != HCF {
        let ap = ops[ip + 1] as usize;
        let bp = ops[ip + 2] as usize;
        let rp = ops[ip + 3] as usize;

        let x = match op {
            ADD => ops[ap] + ops[bp],
            MULTIPLY => ops[ap] * ops[bp],
            _ => panic!("Unrecognized instruction"),
        };

        ops[rp] = x;
        ip += 4;
        op = ops[ip];
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn init() -> Result<Vec<u32>> {
    let input = read_to_string("input.txt")?;
    let mut ops: Vec<u32> = Vec::new();
    for op_str in input.split(",") {
        let op: u32 = op_str.parse()?;
        ops.push(op);
    }

    Ok(ops)
}

fn main() -> Result<()> {
    let target = 19690720;

    // Dumb brute force
    for a in 1..100 {
        for b in 1..100 {
            let mut ops = init()?;
            ops[1] = a;
            ops[2] = b;

            compute(&mut ops);
            if ops[0] == target {
                println!("perfect a: {}, b: {}", a, b);
            }
        }
    }

    println!("end");
    //println!("{}", ops[0]);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {}
}
