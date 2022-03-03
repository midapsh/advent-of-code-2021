use std::cmp::Ordering;

const _DUMMY_INPUT: &str = include_str!("data/day7-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day7-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let positions = values
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // The median minimizes the sum of absolute deviations
    // https://math.stackexchange.com/questions/113270/the-median-minimizes-the-sum-of-absolute-deviations-the-ell-1-norm
    if let Some(med) = median(&positions) {
        positions
            .iter()
            .map(|&x| (med - x).abs())
            .sum::<i32>()
            .to_string()
    } else {
        panic!("Median not found")
    }
}

fn private_solve_part_2(values: &str) -> String {
    let positions = values
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // The mean minimizes the sum of squared deviations
    // https://math.stackexchange.com/questions/2554243/understanding-the-mean-minimizes-the-mean-squared-error
    if let Some(result) = mean(&positions) {
        let score = [
            optimize(&positions, if result > 0 { result - 1 } else { 0 }),
            optimize(&positions, result),
            optimize(&positions, result + 1),
        ];

        score.iter().min().unwrap().to_string()
    } else {
        panic!("Mean not found")
    }
}

// Part 1

fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
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

fn select(data: &[i32], k: usize) -> Option<i32> {
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

fn median(data: &[i32]) -> Option<i32> {
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

// Part 2

fn mean(data: &[i32]) -> Option<i32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len() as f32;
    match count {
        positive if positive > 0.0 => {
            let a = sum / count;
            let result = (a).round() as i32;
            Some(result)
        }
        _ => None,
    }
}

fn optimize(positions: &[i32], result: i32) -> i32 {
    positions.iter().map(|&x| aps(result, x)).sum::<i32>()
}

fn aps(a1: i32, an: i32) -> i32 {
    let (q, p) = if a1 < an { (an, a1) } else { (a1, an) };
    let n = q - p + 1;
    (n * (n - 1)) / 2
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
        assert_eq!("37", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("168", _solve_part_2_dummy());
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
