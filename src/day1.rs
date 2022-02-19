pub const DUMMY_INPUT: &str = include_str!("data/day1-dummy.txt");
pub const REAL_INPUT: &str = include_str!("data/day1-real.txt");

pub fn solve_part_1(value: &str) -> String {
    unimplemented!()
}

pub fn solve_part_2(value: &str) -> String {
    unimplemented!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, DUMMY_INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!("", solve_part_1(DUMMY_INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("", solve_part_2(DUMMY_INPUT));
    }
}
