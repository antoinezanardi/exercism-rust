fn get_digit_power_of(digit: char, power: usize) -> u32 {
    let digit = digit.to_digit(10).unwrap_or(0);

    digit.pow(power as u32)
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let power = digits.len();
    let sum: u32 = digits.chars().map(|c| get_digit_power_of(c, power)).sum();

    sum == num
}
