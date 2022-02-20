pub const DUMMY_INPUT: &str = include_str!("data/day1-dummy.txt");
pub const REAL_INPUT: &str = include_str!("data/day1-real.txt");

pub fn solve_part_1(values: &str) -> String {
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

pub fn solve_part_2(values: &str) -> String {
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

fn main() {
    println!("{}", solve_part_1(REAL_INPUT));
    println!("{}", solve_part_2(REAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2, DUMMY_INPUT};

    #[test]
    fn test_part_1() {
        assert_eq!("7", solve_part_1(DUMMY_INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!("5", solve_part_2(DUMMY_INPUT));
    }
}
