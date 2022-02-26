const _DUMMY_INPUT: &str = include_str!("data/day3-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day3-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let mut count_lines = 0;
    let mut counter: Vec<u32> = Vec::new();
    const RADIX: u32 = 10;

    for line in values.lines().take(1) {
        count_lines += 1;
        for number in line.chars() {
            counter.push(number.to_digit(RADIX).unwrap());
        }
    }
    for line in values.lines().skip(1) {
        count_lines += 1;

        for (pos, number) in line.chars().enumerate() {
            counter[pos] += number.to_digit(RADIX).unwrap();
        }
    }

    let mut gamma_rate = 0;
    for value in &counter {
        if 2 * value < count_lines {
            gamma_rate = (gamma_rate << 1) + 0;
        } else {
            gamma_rate = (gamma_rate << 1) + 1;
        }
    }
    let counter_len = counter.len() as i32;
    let epsilon_rate = !gamma_rate & ((1 << counter_len) - 1);

    (gamma_rate * epsilon_rate).to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let numbers = values
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|char| char != '0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let o2 = extract_number(numbers.clone(), |one_bits, total_bits| {
        one_bits * 2 >= total_bits
    });
    let co2 = extract_number(numbers.clone(), |one_bits, total_bits| {
        one_bits * 2 < total_bits
    });
    (o2 * co2).to_string()
}

fn extract_number(mut numbers: Vec<Vec<bool>>, criteria: fn(usize, usize) -> bool) -> i32 {
    for pos in 0..numbers[0].len() {
        if numbers.len() == 1 {
            break;
        }
        number_filter(&mut numbers, pos, criteria);
    }
    numbers[0]
        .iter()
        .fold(0, |value, &bit| (value << 1) + if bit { 1 } else { 0 })
}

fn number_filter(numbers: &mut Vec<Vec<bool>>, pos: usize, criteria: fn(usize, usize) -> bool) {
    let num_ones = numbers
        .iter()
        .fold(0, |n, number| if number[pos] { n + 1 } else { n });
    let bit = criteria(num_ones, numbers.len());
    // numbers.retain()
    todo!()
}

fn _solve_part_1_dummy() -> String {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!("198", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("230", _solve_part_2_dummy());
    }
    #[test]
    fn test_part_1_real() {
        println!("{}", solve_part_1_real());
    }
    #[test]
    fn test_part_2_real() {
        println!("{}", solve_part_2_real());
    }
}
