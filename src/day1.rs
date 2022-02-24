const _DUMMY_INPUT: &str = include_str!("data/day1-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day1-real.txt");

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .fold((0, None), |(mut n, prev), line| {
            let depth = line.trim().parse::<i32>().unwrap();
            if let Some(prev) = prev {
                if depth > prev {
                    n += 1;
                }
            }
            (n, Some(depth))
        })
        .0
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    const WINDOW_SIZE: usize = 3;
    let values = values
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(WINDOW_SIZE)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>();
    values
        .iter()
        .zip(values.clone().iter().skip(1))
        .filter(|(cur, next)| cur < next)
        .count()
        .to_string()
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
        assert_eq!("7", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("5", _solve_part_2_dummy());
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
