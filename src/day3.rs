const _DUMMY_INPUT: &str = include_str!("data/day3-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day3-real.txt");

pub fn private_solve_part_1(values: &str) -> String {
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

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for value in counter {
        if 2 * value < count_lines {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }
    let int_gamma_rate = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let int_epsilon_rate = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();

    dbg!((int_gamma_rate * int_epsilon_rate).to_string())
}

pub fn private_solve_part_2(values: &str) -> String {
    String::new()
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
