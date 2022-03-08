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
    let mut heightmap = values
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_digit(10).unwrap(), 0usize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_size = heightmap.len();
    let col_size = heightmap[0].len();

    let first_row: usize = 0;
    let first_col: usize = 0;

    let last_row = row_size.saturating_sub(1);
    let last_col = col_size.saturating_sub(1);

    let mut basin_marker: usize = 0;
    let mut total: Vec<usize> = vec![];
    for row_index in first_row..row_size {
        for col_index in first_col..col_size {
            let (current_value, _) = heightmap[row_index][col_index];
            find_basin_points(
                &mut heightmap,
                row_size,
                col_size,
                last_row,
                last_col,
                //
                current_value,
                &mut basin_marker,
                row_index,
                col_index,
            );
            if basin_marker != 0 {
                total.push(basin_marker);
            }

            basin_marker = 0;
        }
    }
    total.sort_unstable();
    let prod = total.iter().rev().take(3).product::<usize>();
    // println!("{:?}", heightmap);
    prod.to_string()
}

fn find_basin_points(
    heightmap: &mut Vec<Vec<(u32, usize)>>,
    row_size: usize,
    col_size: usize,
    last_row: usize,
    last_col: usize,
    //
    current_value: u32,
    basin_marker: &mut usize,
    row_index: usize,
    col_index: usize,
) {
    for r in row_index.saturating_sub(1)..=(row_index + 1).min(last_row) {
        for c in col_index.saturating_sub(1)..=(col_index + 1).min(last_col) {
            // Manhattan distance
            if !(r == row_index || c == col_index)
                || heightmap[r][c].0 == 9
                || heightmap[r][c].1 == 1
            {
                continue;
            }
            if current_value == 9 || current_value == 0 {
                return;
            }
            heightmap[r][c].1 = 1;
            *basin_marker += 1;
            find_basin_points(
                heightmap,
                row_size,
                col_size,
                last_row,
                last_col,
                //
                heightmap[r][c].0,
                basin_marker,
                r,
                c,
            );

            // // If you want to find only the value that is one less/more
            // // than the current value
            // if (current_value + 1).min(9) == heightmap[r][c].0 {
            //     find_basin_points(
            //         heightmap,
            //         row_size,
            //         col_size,
            //         last_row,
            //         last_col,
            //         //
            //         (current_value + 1).min(9),
            //         basin_marker,
            //         r,
            //         c,
            //     );
            // } else if (current_value).saturating_sub(1) == heightmap[r][c].0 {
            //     find_basin_points(
            //         heightmap,
            //         row_size,
            //         col_size,
            //         last_row,
            //         last_col,
            //         //
            //         (current_value).saturating_sub(1),
            //         basin_marker,
            //         r,
            //         c,
            //     );
            // }
        }
    }
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
        assert_eq!("1134", _solve_part_2_dummy());
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
