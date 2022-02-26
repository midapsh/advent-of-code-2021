const _DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day2-real.txt");

enum Command {
    Up,
    Down,
    Forward,
}

impl From<&str> for Command {
    fn from(command: &str) -> Self {
        match command {
            "up" => Self::Up,
            "down" => Self::Down,
            "forward" => Self::Forward,
            _ => panic!("Not a valid command"),
        }
    }
}

fn private_solve_part_1(values: &str) -> String {
    let mut x = 0;
    let mut y = 0;

    values.lines().for_each(|line| {
        let mut iter = line.trim().split_ascii_whitespace();
        let command = iter.next().unwrap();
        let gain = iter.next().unwrap().parse::<i32>().unwrap();

        match command.into() {
            Command::Up => y -= gain,
            Command::Down => y += gain,
            Command::Forward => x += gain,
        }
    });
    (x * y).to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    values.lines().for_each(|line| {
        let mut iter = line.trim().split_ascii_whitespace();
        let command = iter.next().unwrap();
        let gain = iter.next().unwrap().parse::<i32>().unwrap();

        match command.into() {
            Command::Up => aim -= gain,
            Command::Down => aim += gain,
            Command::Forward => {
                x += gain;
                y += aim * gain;
            }
        }
    });
    (x * y).to_string()
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
        assert_eq!("150", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("900", _solve_part_2_dummy());
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
