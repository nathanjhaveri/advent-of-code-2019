mod lib;
use lib::*;

fn main() {
    let input: &str = &SIXTEEN_INPUT.repeat(10000);
    let signal = parse_input_vec(input);
    let offset = message_offset(&signal);

    let output = process_signal(input, 100);
    let output = val_at_offset(&output, offset);

    println!("output {}", output);
}
