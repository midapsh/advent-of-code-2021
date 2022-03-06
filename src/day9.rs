const _DUMMY_INPUT: &str = include_str!("data/day9-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day9-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let mut heightmap = values
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap(), false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_size = heightmap.len();
    let col_size = heightmap[0].len();

    let first_row: usize = 0;
    let first_col: usize = 0;

    let last_row = row_size.saturating_sub(1);
    let last_col = col_size.saturating_sub(1);

    for row_index in first_row..row_size {
        for col_index in first_col..col_size {
            let (current_value, _) = heightmap[row_index][col_index];
            let mut min = current_value;
            for r in row_index.saturating_sub(1)..=(row_index + 1).min(last_row) {
                for c in col_index.saturating_sub(1)..=(col_index + 1).min(last_col) {
                    if heightmap[r][c].0 < min {
                        min = heightmap[r][c].0;
                    }
                }
            }
            heightmap[row_index][col_index].1 = current_value == min;
        }
    }

    heightmap
        .iter()
        .flatten()
        .map(|(value, checker)| if *checker { value + 1 } else { 0 })
        .sum::<u32>()
        .to_string()
}

fn private_solve_part_2(values: &str) -> String {
    unimplemented!()
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
        assert_eq!("15", _solve_part_1_dummy());
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
