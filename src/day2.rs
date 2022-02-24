pub const DUMMY_INPUT: &str = include_str!("data/day2-dummy.txt");
pub const REAL_INPUT: &str = include_str!("data/day2-real.txt");

pub fn solve_part_1(values: &str) -> String {
    let mut x = 0;
    let mut y = 0;

    values.lines().for_each(|line| {
        let mut iter = line.trim().split_ascii_whitespace();
        let command = iter.next();
        let gain = iter.next().unwrap().parse::<i32>().unwrap();

        match command {
            Some("up") => y -= gain,
            Some("down") => y += gain,
            Some("forward") => x += gain,
            _ => panic!("Not a valid command"),
        }
    });
    (x * y).to_string()
}

pub fn solve_part_2(values: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    values.lines().for_each(|line| {
        let mut iter = line.trim().split_ascii_whitespace();
        let command = iter.next();
        let gain = iter.next().unwrap().parse::<i32>().unwrap();

        match command {
            Some("up") => aim -= gain,
            Some("down") => aim += gain,
            Some("forward") => {
                x += gain;
                y += aim * gain;
            }
            _ => panic!("Not a valid command"),
        }
    });
    (x * y).to_string()
}

fn main() {
    println!("{}", solve_part_1(REAL_INPUT));
    println!("{}", solve_part_2(REAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, DUMMY_INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!("150", solve_part_1(DUMMY_INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("900", solve_part_2(DUMMY_INPUT));
    }
}
