const _DUMMY_INPUT: &str = include_str!("data/day6-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day6-real.txt");
const PART_1_NUMBER_OF_DAYS: usize = 80;
const PART_2_NUMBER_OF_DAYS: usize = 256;

fn private_solve_part_1(values: &str) -> u64 {
    solution_via_array(values, PART_1_NUMBER_OF_DAYS)
}

fn solution_via_array(values: &str, days: usize) -> u64 {
    (0..days)
        .into_iter()
        .fold(get_counters(values), |mut counters, _day| {
            let gen = counters[0];
            counters[0] = counters[1];
            counters[1] = counters[2];
            counters[2] = counters[3];
            counters[3] = counters[4];
            counters[4] = counters[5];
            counters[5] = counters[6];
            counters[6] = counters[7] + gen;
            counters[7] = counters[8];
            counters[8] = gen;
            counters
        })
        .into_iter()
        .sum::<u64>()
}

fn get_counters(values: &str) -> [u64; 9] {
    values
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .fold([0; 9], |mut counters, n| {
            counters[n] += 1;
            counters
        })
}

fn private_solve_part_2(values: &str) -> u64 {
    solution_via_array(values, PART_2_NUMBER_OF_DAYS)
}

fn _solve_part_1_dummy() -> u64 {
    private_solve_part_1(_DUMMY_INPUT)
}

pub fn solve_part_1_real() -> u64 {
    private_solve_part_1(REAL_INPUT)
}

fn _solve_part_2_dummy() -> u64 {
    private_solve_part_2(_DUMMY_INPUT)
}

pub fn solve_part_2_real() -> u64 {
    private_solve_part_2(REAL_INPUT)
}

#[cfg(test)]
mod tests {
    use super::{_solve_part_1_dummy, _solve_part_2_dummy, solve_part_1_real, solve_part_2_real};

    #[test]
    fn test_part_1_dummy() {
        assert_eq!(5934, _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!(26984457539, _solve_part_2_dummy());
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
