use rulinalg::matrix::*;

pub const SIXTEEN_INPUT: &str = "59704176224151213770484189932636989396016853707543672704688031159981571127975101449262562108536062222616286393177775420275833561490214618092338108958319534766917790598728831388012618201701341130599267905059417956666371111749252733037090364984971914108277005170417001289652084308389839318318592713462923155468396822247189750655575623017333088246364350280299985979331660143758996484413769438651303748536351772868104792161361952505811489060546839032499706132682563962136170941039904873411038529684473891392104152677551989278815089949043159200373061921992851799948057507078358356630228490883482290389217471790233756775862302710944760078623023456856105493";

pub fn parse_input(input: &str) -> Matrix<i64> {
    let digits: Vec<i64> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d as i64)
        .collect();
    Matrix::new(digits.len(), 1, digits)
}

pub fn create_pattern_matrix(pattern: &[i64], n: usize) -> Matrix<i64> {
    let mut values = Vec::with_capacity(n * n);
    for row in 1..=n {
        let mut items = 0;
        while items < n {
            items += 1;
            let idx = (items / row) % pattern.len();
            values.push(pattern[idx]);
        }
    }

    Matrix::new(n, n, values)
}

pub fn phase(pattern: &Matrix<i64>, signal: &Matrix<i64>) -> Matrix<i64> {
    truncate_digits(&(pattern * signal))
}

// For all matrix entries, use only 1's digit
pub fn truncate_digits(m: &Matrix<i64>) -> Matrix<i64> {
    let truncated: Vec<i64> = m.iter().map(|d| d.abs() % 10).collect();
    Matrix::new(truncated.len(), 1, truncated)
}

pub fn process_signal(input: &str, phases: usize) -> Vec<i64> {
    let pattern = vec![0, 1, 0, -1];
    let mut signal = parse_input(input);
    let pattern_matrix = create_pattern_matrix(&pattern, signal.rows() * signal.cols());

    for _ in 0..phases {
        signal = phase(&pattern_matrix, &signal);
    }

    vec![
        signal[[0, 0]],
        signal[[1, 0]],
        signal[[2, 0]],
        signal[[3, 0]],
        signal[[4, 0]],
        signal[[5, 0]],
        signal[[6, 0]],
        signal[[7, 0]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_0() {
        let input = "12345678";
        let pattern = vec![0, 1, 0, -1];
        let signal = parse_input(input);
        let pattern_mtx = create_pattern_matrix(&pattern, signal.rows() * signal.cols());
        let phase1 = phase(&pattern_mtx, &signal);
        assert_eq!(phase1, Matrix::new(8, 1, vec![4, 8, 2, 2, 6, 1, 5, 8]));

        let phase2 = phase(&pattern_mtx, &phase1);
        assert_eq!(phase2, Matrix::new(8, 1, vec![3, 4, 0, 4, 0, 4, 3, 8]));
    }

    #[test]
    fn example_1() {
        let input = "80871224585914546619083218645595";
        let output = process_signal(input, 100);
        assert_eq!(output, vec![2, 4, 1, 7, 6, 1, 7, 6])
    }

    #[test]
    fn sixteen_1() {
        let output = process_signal(SIXTEEN_INPUT, 100);
        assert_eq!(output, vec![2, 8, 4, 3, 0, 1, 4, 6])
    }
}
