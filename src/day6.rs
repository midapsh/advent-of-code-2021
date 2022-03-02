const _DUMMY_INPUT: &str = include_str!("data/day6-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day6-real.txt");
const PART_1_NUMBER_OF_DAYS: usize = 80;
const PART_2_NUMBER_OF_DAYS: usize = 256;

struct FishStates(u64, u64, u64, u64, u64, u64, u64, u64, u64);

impl FishStates {
    fn new() -> Self {
        Self(0, 0, 0, 0, 0, 0, 0, 0, 0)
    }
    fn get(&self, key: usize) -> u64 {
        match key {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            4 => self.4,
            5 => self.5,
            6 => self.6,
            7 => self.7,
            8 => self.8,
            _ => panic!("Not a valid value"),
        }
    }
    fn insert(&mut self, key: usize, value: u64) {
        match key {
            0 => {
                self.0 = value;
            }
            1 => {
                self.1 = value;
            }
            2 => {
                self.2 = value;
            }
            3 => {
                self.3 = value;
            }
            4 => {
                self.4 = value;
            }
            5 => {
                self.5 = value;
            }
            6 => {
                self.6 = value;
            }
            7 => {
                self.7 = value;
            }
            8 => {
                self.8 = value;
            }
            _ => panic!("Not a valid value"),
        }
    }
    fn add(&mut self, key: usize, value: u64) {
        match key {
            0 => {
                self.0 += value;
            }
            1 => {
                self.1 += value;
            }
            2 => {
                self.2 += value;
            }
            3 => {
                self.3 += value;
            }
            4 => {
                self.4 += value;
            }
            5 => {
                self.5 += value;
            }
            6 => {
                self.6 += value;
            }
            7 => {
                self.7 += value;
            }
            8 => {
                self.8 += value;
            }
            _ => panic!("Not a valid value"),
        }
    }
    fn get_total_values(self) -> u64 {
        self.0 + self.1 + self.2 + self.3 + self.4 + self.5 + self.6 + self.7 + self.8
    }
}

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
    // solution_via_struct(values, PART_2_NUMBER_OF_DAYS)
}

fn solution_via_struct(values: &str, days: usize) -> u64 {
    let mut fish_states = FishStates::new();
    values
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .for_each(|x| {
            fish_states.add(x, 1);
        });
    for _ in 0..days {
        let count = fish_states.0;
        fish_states.insert(0, fish_states.1);
        for fish_state in 0..8 {
            fish_states.insert(fish_state, fish_states.get(fish_state + 1));
        }
        fish_states.add(6, count);
        fish_states.insert(8, count);
    }
    fish_states.get_total_values()
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
