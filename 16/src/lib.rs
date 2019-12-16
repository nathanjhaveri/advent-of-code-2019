pub const SIXTEEN_INPUT: &str = "59704176224151213770484189932636989396016853707543672704688031159981571127975101449262562108536062222616286393177775420275833561490214618092338108958319534766917790598728831388012618201701341130599267905059417956666371111749252733037090364984971914108277005170417001289652084308389839318318592713462923155468396822247189750655575623017333088246364350280299985979331660143758996484413769438651303748536351772868104792161361952505811489060546839032499706132682563962136170941039904873411038529684473891392104152677551989278815089949043159200373061921992851799948057507078358356630228490883482290389217471790233756775862302710944760078623023456856105493";
type Int = i16;

pub fn parse_input_vec(input: &str) -> Vec<Int> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d as Int)
        .collect()
}

pub fn phase_in_place(pattern: &[Int], signal: &[Int]) -> Vec<Int> {
    let n = signal.len();
    let mut values = Vec::with_capacity(n);
    for row in 1..=n {
        let mut sum: i64 = 0;
        for col in row..=n {
            let pattern_idx = (col / row) % pattern.len();
            let signal_idx = col - 1;
            sum += pattern[pattern_idx] as i64 * signal[signal_idx] as i64;
        }

        let truncated = (sum.abs() % 10) as Int;
        values.push(truncated);
    }

    values
}

pub fn process_signal(input: &str, phases: usize) -> Vec<Int> {
    let pattern = vec![0, 1, 0, -1];
    let mut signal = parse_input_vec(input);

    for i in 0..phases {
        println!("phase {}", i);
        signal = phase_in_place(&pattern, &signal);
    }

    signal
}

pub fn val_at_offset(signal: &[Int], offset: usize) -> i64 {
    (0..8)
        .map(|i| signal[offset + i] as i64 * 10i64.pow(7 - i as u32))
        .sum()
}

pub fn message_offset(signal: &[Int]) -> usize {
    (signal[0] as usize * 10usize.pow(6)
        + signal[1] as usize * 10usize.pow(5)
        + signal[2] as usize * 10usize.pow(4)
        + signal[3] as usize * 10usize.pow(3)
        + signal[4] as usize * 10usize.pow(2)
        + signal[5] as usize * 10usize.pow(1)
        + signal[6] as usize * 10usize.pow(0)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "80871224585914546619083218645595";
        let output = process_signal(input, 100);
        let val = val_at_offset(&output, 0);
        assert_eq!(val, 24_176_176);
    }

    #[test]
    fn verify_offset() {
        let input = "03036732577212944063491565474664";
        let signal = parse_input_vec(input);
        assert_eq!(303_673, message_offset(&signal));
    }

    #[test]
    fn sixteen_1() {
        let output = process_signal(SIXTEEN_INPUT, 100);
        let output = val_at_offset(&output, 0);
        assert_eq!(output, 28_430_146);
    }

    //#[test]
    //fn example_2_1() {
    //    let input: &str = &"03036732577212944063491565474664".repeat(10000);
    //    let signal = parse_input_vec(input);
    //    let offset = message_offset(&signal);

    //    let signal = process_signal(input, 100);

    //    let val = val_at_offset(&signal, offset);
    //    assert_eq!(val, 84_462_026);
    //}

    // #[test]
    // fn sixteen_2() {
    //     let input: &str = &SIXTEEN_INPUT.repeat(10000);
    //     let signal = parse_input_vec(input);
    //     let offset = message_offset(&signal);

    //     let output = process_signal(input, 100);
    //     let output = val_at_offset(&output, offset);
    //     assert_eq!(output, 284)
    // }
}
