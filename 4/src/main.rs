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

        if two_adjacent_same(digits) && increasing(digits){
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
    digits[0] <= digits[1] &&
    digits[1] <= digits[2] &&
    digits[2] <= digits[3] &&
    digits[3] <= digits[4] &&
    digits[4] <= digits[5]
}

fn two_adjacent_same(digits: Password) -> bool {
    digits[0] == digits[1] ||
    digits[1] == digits[2] ||
    digits[2] == digits[3] ||
    digits[3] == digits[4] ||
    digits[4] == digits[5]
}

fn nth_digit(n: u32, number: i32) -> i32 {
    (number / 10i32.pow(n)) % 10
}

#[test]
fn name() {
    let lower = 111110;
    let upper = 111112;
    println!("YO {}", (1234 / 10) % 10);
    assert_eq!(1, find_valid(lower, upper));
}