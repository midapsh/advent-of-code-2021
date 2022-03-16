const _DUMMY0_INPUT: &str = include_str!("data/day12-dummy0.txt");
const _DUMMY1_INPUT: &str = include_str!("data/day12-dummy1.txt");
const _DUMMY2_INPUT: &str = include_str!("data/day12-dummy2.txt");
const REAL_INPUT: &str = include_str!("data/day12-real.txt");

enum Room<'a> {
    Big(&'a str),
    End,
    Small(&'a str),
    Start,
}

fn private_solve_part_1(values: &str) -> String {
    unimplemented!()
}

fn private_solve_part_2(values: &str) -> String {
    unimplemented!()
}

fn _solve_part_1_dummy0() -> String {
    private_solve_part_1(_DUMMY0_INPUT)
}

fn _solve_part_1_dummy1() -> String {
    private_solve_part_1(_DUMMY1_INPUT)
}

fn _solve_part_1_dummy2() -> String {
    private_solve_part_1(_DUMMY2_INPUT)
}

pub fn solve_part_1_real() -> String {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> String {
    private_solve_part_2(_DUMMY0_INPUT)
}

pub fn solve_part_2_real() -> String {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{
        _solve_part_1_dummy0, _solve_part_1_dummy1, _solve_part_1_dummy2, _solve_part_2_dummy,
        solve_part_1_real, solve_part_2_real,
    };

    #[test]
    fn test_part_1_dummy0() {
        assert_eq!("10", _solve_part_1_dummy0());
    }
    #[test]
    fn test_part_1_dummy1() {
        assert_eq!("19", _solve_part_1_dummy1());
    }
    #[test]
    fn test_part_1_dummy2() {
        assert_eq!("226", _solve_part_1_dummy2());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("", _solve_part_2_dummy());
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
