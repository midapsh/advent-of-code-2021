use std::cmp::Ordering;

const _DUMMY_INPUT: &str = include_str!("data/day10-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day10-real.txt");

fn partition(data: &[usize]) -> Option<(Vec<usize>, usize, Vec<usize>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[usize], k: usize) -> Option<usize> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn median(data: &[usize]) -> Option<usize> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) / 2),
                _ => None,
            }
        }
        odd => select(data, odd / 2),
    }
}

fn private_solve_part_1(values: &str) -> String {
    values
        .lines()
        .map(|line| {
            let mut delimiters = Vec::new();
            line.trim()
                .chars()
                .find_map(|c| match c {
                    '(' => {
                        delimiters.push(')');
                        None
                    }
                    '[' => {
                        delimiters.push(']');
                        None
                    }
                    '{' => {
                        delimiters.push('}');
                        None
                    }
                    '<' => {
                        delimiters.push('>');
                        None
                    }
                    c if delimiters.last() == Some(&c) => {
                        delimiters.pop();
                        None
                    }
                    ')' => Some(3),
                    ']' => Some(57),
                    '}' => Some(1_197),
                    '>' => Some(25_137),
                    _ => panic!("Bad input!"),
                })
                .unwrap_or(0)
        })
        .sum::<usize>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    let scores = values
        .lines()
        .map(|line| {
            let mut delimiters = Vec::new();
            for c in line.trim().chars() {
                match c {
                    '(' => {
                        delimiters.push(')');
                    }
                    '[' => {
                        delimiters.push(']');
                    }
                    '{' => {
                        delimiters.push('}');
                    }
                    '<' => {
                        delimiters.push('>');
                    }
                    c if delimiters.last() == Some(&c) => {
                        delimiters.pop();
                    }
                    ')' | ']' | '}' | '>' => return 0,
                    _ => panic!("Bad delimiter input!"),
                }
            }
            let mut score = 0usize;
            for c in delimiters.into_iter().rev() {
                score *= 5;
                score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Bad score input!"),
                }
            }
            score
        })
        .filter(|&score| score != 0)
        .collect::<Vec<_>>();

    median(&scores).unwrap().to_string()
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
        assert_eq!(
            ((2 * 3) + (1 * 57) + (1 * 1197) + (1 * 25137)).to_string(),
            _solve_part_1_dummy()
        );
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("288957", _solve_part_2_dummy());
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
