const _DUMMY_INPUT: &str = include_str!("data/day5-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day5-real.txt");
const MAX_BOARD_SIZE: usize = 1_000;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn private_solve_part_1(values: &str) -> String {
    let mut board = vec![vec![0; MAX_BOARD_SIZE]; MAX_BOARD_SIZE];
    values.lines().for_each(|line| {
        let result = line
            .split("->")
            .map(|x| {
                let mut value = x.trim().split(',');
                Coord {
                    x: value.next().unwrap().parse::<usize>().unwrap(),
                    y: value.next().unwrap().parse::<usize>().unwrap(),
                }
            })
            .take(2)
            .collect::<Vec<_>>();

        let start = &result[0];
        let finish = &result[1];

        if start.x == finish.x {
            if start.y < finish.y {
                for index in start.y..=finish.y {
                    board[start.x][index] += 1;
                }
            } else {
                for index in (finish.y..=start.y).rev() {
                    board[start.x][index] += 1;
                }
            }
        } else if start.y == finish.y {
            if start.x < finish.x {
                for index in start.x..=finish.x {
                    board[index][start.y] += 1;
                }
            } else {
                for index in (finish.x..=start.x).rev() {
                    board[index][start.y] += 1;
                }
            }
        }
    });
    let mut points = 0;
    for row in board {
        for cell in row {
            if 2 <= cell {
                points += 1;
            }
        }
    }
    points.to_string()
}

fn private_solve_part_2(values: &str) -> String {
    // let mut board = [[0; 10]; 10];
    let mut board = vec![vec![0; MAX_BOARD_SIZE]; MAX_BOARD_SIZE];
    values.lines().for_each(|line| {
        let result = line
            .split("->")
            .map(|x| {
                let mut value = x.trim().split(',');
                Coord {
                    x: value.next().unwrap().parse::<usize>().unwrap(),
                    y: value.next().unwrap().parse::<usize>().unwrap(),
                }
            })
            .take(2)
            .collect::<Vec<_>>();

        let start = &result[0];
        let finish = &result[1];

        if start.x == finish.x {
            if start.y < finish.y {
                for index in start.y..=finish.y {
                    board[start.x][index] += 1;
                }
            } else {
                for index in (finish.y..=start.y).rev() {
                    board[start.x][index] += 1;
                }
            }
        } else if start.y == finish.y {
            if start.x < finish.x {
                for index in start.x..=finish.x {
                    board[index][start.y] += 1;
                }
            } else {
                for index in (finish.x..=start.x).rev() {
                    board[index][start.y] += 1;
                }
            }
        } else if start.x == start.y && finish.x == finish.y {
            if start.x < finish.x {
                for index in start.x..=finish.x {
                    board[index][index] += 1;
                }
            } else {
                for index in finish.x..=start.x {
                    board[index][index] += 1;
                }
            }
        } else if (finish.x as i16 - start.x as i16).abs()
            == (finish.y as i16 - start.y as i16).abs()
        {
            if start.x < finish.x && start.y < finish.y {
                for (row_index, col_index) in (start.x..=finish.x).zip(start.y..=finish.y) {
                    board[row_index][col_index] += 1;
                }
            } else if finish.x < start.x && finish.y < start.y {
                for (row_index, col_index) in
                    ((finish.x..=start.x).rev()).zip((finish.y..=start.y).rev())
                {
                    board[row_index][col_index] += 1;
                }
            } else if start.x < finish.x && finish.y < start.y {
                for (row_index, col_index) in (start.x..=finish.x).zip((finish.y..=start.y).rev()) {
                    board[row_index][col_index] += 1;
                }
            } else {
                for (row_index, col_index) in (finish.x..=start.x).rev().zip(start.y..=finish.y) {
                    board[row_index][col_index] += 1;
                }
            }
        }
    });
    let mut points = 0;
    for row in board {
        for cell in row {
            if 2 <= cell {
                points += 1;
            }
        }
    }
    points.to_string()
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
        // There is 5 numbers '2'
        // .......1..
        // ..1....1..
        // ..1....1..
        // .......1..
        // .112111211
        // ..........
        // ..........
        // ..........
        // ..........
        // 222111....
        assert_eq!("5", _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!("12", _solve_part_2_dummy());
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
