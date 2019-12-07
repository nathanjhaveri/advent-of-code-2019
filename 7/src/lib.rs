use intcode::{compute, Ops};
use std::ops::Range;

const AMP_COUNT: usize = 5;
const PHASE_RANGE: Range<i32> = 0..5;
pub type PhaseSettings = [i32; AMP_COUNT];

pub fn find_max_phase_setting(program: &Ops) -> (i32, PhaseSettings) {
    let mut max_thruster = 0;
    let mut phase_settings: PhaseSettings = [0; AMP_COUNT];

    for a in PHASE_RANGE {
        for b in PHASE_RANGE {
            for c in PHASE_RANGE {
                for d in PHASE_RANGE {
                    for e in PHASE_RANGE {
                        let test_settings = [a, b, c, d, e];
                        if valid_phase_setting(&test_settings) {
                            let test_signal = thruster_signal(program, &test_settings);
                            if max_thruster < test_signal {
                                max_thruster = test_signal;
                                phase_settings = test_settings;
                            }
                        }
                    }
                }
            }
        }
    }

    (max_thruster, phase_settings)
}

fn thruster_signal(program: &Ops, phase_settings: &PhaseSettings) -> i32 {
    let mut input_signal = 0;
    for i in 0..AMP_COUNT {
        let mut running = program.clone();
        let program_input = [phase_settings[i], input_signal];
        let output = compute(&mut running, &program_input);
        input_signal = *output.last().expect("No ouput")
    }

    input_signal
}

fn valid_phase_setting(phase: &PhaseSettings) -> bool {
    let mut contains_all = true;
    for setting in PHASE_RANGE {
        contains_all &= phase.contains(&setting);
    }

    contains_all
}

#[cfg(test)]
mod tests {
    use super::*;
    use intcode::init_ops;

    #[test]
    fn example1() {
        let ops = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = init_ops(ops).unwrap();
        let max = 43210;
        let phase_settings: PhaseSettings = [4, 3, 2, 1, 0];

        let signal = thruster_signal(&program, &phase_settings);
        assert_eq!(max, signal);
    }

    #[test]
    fn example1_find() {
        let ops = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let program = init_ops(ops).unwrap();
        let expected_max = 43210;
        let expected_phase: PhaseSettings = [4, 3, 2, 1, 0];

        let (actual_max, actual_phase) = find_max_phase_setting(&program);

        assert_eq!(actual_phase, expected_phase);
        assert_eq!(actual_max, expected_max);
    }

    #[test]
    fn seven_one() {
        let ops = "3,8,1001,8,10,8,105,1,0,0,21,34,47,72,81,102,183,264,345,426,99999,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,4,9,9,1002,9,3,9,4,9,99,3,9,102,3,9,9,101,2,9,9,102,5,9,9,1001,9,3,9,1002,9,4,9,4,9,99,3,9,101,5,9,9,4,9,99,3,9,101,3,9,9,1002,9,5,9,101,4,9,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99";
        let program = init_ops(ops).unwrap();
        let expected_max = 92663;
        let expected_phase: PhaseSettings = [3, 1, 4, 2, 0];

        let (actual_max, actual_phase) = find_max_phase_setting(&program);

        assert_eq!(actual_phase, expected_phase);
        assert_eq!(actual_max, expected_max);
    }
}
