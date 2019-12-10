type Password = [i32; 6];

fn main() {
    let lower = 372304;
    let upper = 847060;
    let valid = find_valid(lower, upper);

    println!("valid: {}", valid);
}

fn find_valid(lower: i32, upper: i32) -> i32 {
    let mut count = 0;
    for number in lower..upper {
        let digits = digits(number);
        //println!("number {:?}", number);
        //println!("digits {:?}", digits);

        if only_two_adjacent_same(digits) && increasing(digits) {
            count += 1;
        }
    }

    count
}

fn digits(number: i32) -> Password {
    [
        nth_digit(5, number),
        nth_digit(4, number),
        nth_digit(3, number),
        nth_digit(2, number),
        nth_digit(1, number),
        nth_digit(0, number),
    ]
}

fn increasing(digits: Password) -> bool {
    digits[0] <= digits[1]
        && digits[1] <= digits[2]
        && digits[2] <= digits[3]
        && digits[3] <= digits[4]
        && digits[4] <= digits[5]
}

//fn two_adjacent_same(digits: Password) -> bool {
//    digits[0] == digits[1] ||
//    digits[1] == digits[2] ||
//    digits[2] == digits[3] ||
//    digits[3] == digits[4] ||
//    digits[4] == digits[5]
//}

fn only_two_adjacent_same(digits: Password) -> bool {
    let mut i = 0;
    let end = digits.len() - 1;
    while i < end {
        let mut run_len = 1;
        let mut candidate = digits[i];
        let mut next = digits[i + 1];

        while candidate == next && i < end - 1 {
            i += 1;
            run_len += 1;
            candidate = digits[i];
            next = digits[i + 1];
        }

        if candidate == next {
            run_len += 1
        }

        if run_len == 2 {
            return true;
        }

        i += 1;
    }

    return false;
}

fn nth_digit(n: u32, number: i32) -> i32 {
    (number / 10i32.pow(n)) % 10
}

#[test]
fn name() {
    let lower = 111110;
    let upper = 111112;
    assert_eq!(0, find_valid(lower, upper));
}
