use std::{collections::HashSet, ops::RangeBounds};

const _DUMMY_INPUT: &str = include_str!("data/day11-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day11-real.txt");

const NUMBER_OF_ROUNDS: usize = 100;
const NEIGHBORS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
];

fn private_solve_part_1(values: &str) -> String {
    let mut grid = values
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let y_range = 0..grid.len() as isize;
    let x_range = 0..grid[0].len() as isize;

    let mut flashes = 0;
    for _ in 0..NUMBER_OF_ROUNDS {
        let mut boundary = HashSet::new();
        for (y, row) in grid.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                if 9 <= *col {
                    *col = 0;
                    flashes += 1;
                    boundary.insert((x as isize, y as isize));
                } else {
                    *col += 1;
                }
            }
        }

        while !boundary.is_empty() {
            let point = *boundary.iter().next().unwrap();
            boundary.remove(&point);
            let (x, y) = point;
            for n in &NEIGHBORS {
                let y2 = y + n.0;
                let x2 = x + n.1;
                if y_range.contains(&y2) && x_range.contains(&x2) {
                    let n_energy = &mut grid[y2 as usize][x2 as usize];
                    match *n_energy {
                        0 => {}
                        9 => {
                            *n_energy = 0;
                            flashes += 1;
                            boundary.insert((x2, y2));
                        }
                        _ => {
                            *n_energy += 1;
                        }
                    }
                }
            }
        }
    }

    flashes.to_string()
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
        assert_eq!("1656", _solve_part_1_dummy());
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
