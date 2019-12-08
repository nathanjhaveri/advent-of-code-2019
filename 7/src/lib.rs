use intcode::{IntCode, Ops};
use std::ops::Range;

const AMP_COUNT: usize = 5;
const PHASE_RANGE: Range<i32> = 0..5;
const PHASE_RANGE_REPEAT: Range<i32> = 5..10;
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
        let running = program.clone();
        let program_input = [phase_settings[i], input_signal];
        let mut computer = IntCode::init(running);
        for &input in program_input.iter() {
            computer.input(input);
        }

        input_signal = computer.compute_output().expect("no output");
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

fn valid_feedback_phase_setting(phase: &PhaseSettings) -> bool {
    let mut contains_all = true;
    for setting in PHASE_RANGE_REPEAT {
        contains_all &= phase.contains(&setting);
    }

    contains_all
}

fn feedback(program: &str, phase: &PhaseSettings) -> i32 {
    let mut output = Some(0);
    let mut amps: Vec<IntCode> = (0..AMP_COUNT).map(|_| IntCode::new(program)).collect();

    // Initialize phases settings
    for i in 0..AMP_COUNT {
        amps[i].input(phase[i]);
    }

    let mut amp_index = 0;
    while let Some(input) = output {
        let amp = &mut amps[amp_index];
        amp.input(input);
        output = amp.compute_output();
        amp_index = (amp_index + 1) % AMP_COUNT;
    }

    amps[AMP_COUNT - 1].last_output()
}

pub fn find_max_feedback_phase_setting(program: &str) -> (i32, PhaseSettings) {
    let mut max_thruster = 0;
    let mut phase_settings: PhaseSettings = [0; AMP_COUNT];

    for a in PHASE_RANGE_REPEAT {
        for b in PHASE_RANGE_REPEAT {
            for c in PHASE_RANGE_REPEAT {
                for d in PHASE_RANGE_REPEAT {
                    for e in PHASE_RANGE_REPEAT {
                        let test_settings = [a, b, c, d, e];
                        if valid_feedback_phase_setting(&test_settings) {
                            let test_signal = feedback(program, &test_settings);
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

#[cfg(test)]
mod tests {
    use super::*;
    use intcode::init_ops;
    const PROGRAM_7: &str = "3,8,1001,8,10,8,105,1,0,0,21,34,47,72,81,102,183,264,345,426,99999,3,9,102,5,9,9,1001,9,3,9,4,9,99,3,9,101,4,9,9,1002,9,3,9,4,9,99,3,9,102,3,9,9,101,2,9,9,102,5,9,9,1001,9,3,9,1002,9,4,9,4,9,99,3,9,101,5,9,9,4,9,99,3,9,101,3,9,9,1002,9,5,9,101,4,9,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99";

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
    fn seven_1() {
        let program = init_ops(PROGRAM_7).unwrap();
        let expected_max = 92663;
        let expected_phase: PhaseSettings = [3, 1, 4, 2, 0];

        let (actual_max, actual_phase) = find_max_phase_setting(&program);

        assert_eq!(actual_phase, expected_phase);
        assert_eq!(actual_max, expected_max);
    }

    #[test]
    fn seven_2() {
        let expected_max = 14_365_052;
        let expected_phase: PhaseSettings = [7, 8, 6, 9, 5];
        let (actual_max, actual_phase) = find_max_feedback_phase_setting(PROGRAM_7);

        assert_eq!(actual_max, expected_max);
        assert_eq!(actual_phase, expected_phase);
    }

    #[test]
    fn example_2_1() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let phase = [9, 8, 7, 6, 5];

        let output = feedback(program, &phase);
        assert_eq!(output, 139_629_729);
    }

    #[test]
    fn example_2_1_find() {
        let program =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let expected_phase = [9, 8, 7, 6, 5];

        let (max_thruster, phase_setting) = find_max_feedback_phase_setting(program);
        assert_eq!(max_thruster, 139_629_729);
        assert_eq!(phase_setting, expected_phase);
    }
}
