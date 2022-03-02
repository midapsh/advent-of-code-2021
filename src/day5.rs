const _DUMMY_INPUT: &str = include_str!("data/day5-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day5-real.txt");
const MAX_BOARD_SIZE: usize = 1_000;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn private_solve_part_1(values: &str) -> String {
    // let mut board = [[0 as usize; 10]; 10];
    let mut board = vec![vec![0 as usize; MAX_BOARD_SIZE]; MAX_BOARD_SIZE];
    values.lines().for_each(|line| {
        let (start, end) = get_coords(line);

        let dy = end.y as isize - start.y as isize;
        let dx = end.x as isize - start.x as isize;
        if dx == 0 || dy == 0 {
            // Manhattan distance
            let dist = dx.abs() + dy.abs();
            let direction_x = dx / dist;
            let direction_y = dy / dist;
            let (mut pos_x, mut pos_y) = (start.x, start.y);
            for _ in 0..=dist {
                board[pos_x][pos_y] += 1;
                pos_x = (pos_x as isize + direction_x) as usize;
                pos_y = (pos_y as isize + direction_y) as usize;
            }
        }
    });

    board
        .iter()
        .map(|row| row.iter().filter(|&&cell| 2 <= cell).count())
        .sum::<usize>()
        .to_string()
}

fn get_coords(line: &str) -> (Coord, Coord) {
    line.split_once("->")
        .map(|(start, end)| {
            let start = start.trim().split_once(',').unwrap();
            let end = end.trim().split_once(',').unwrap();
            (
                Coord {
                    x: start.0.parse::<usize>().unwrap(),
                    y: start.1.parse::<usize>().unwrap(),
                },
                Coord {
                    x: end.0.parse::<usize>().unwrap(),
                    y: end.1.parse::<usize>().unwrap(),
                },
            )
        })
        .unwrap()
}

fn private_solve_part_2(values: &str) -> String {
    // let mut board = [[0 as usize; 10]; 10];
    let mut board = vec![vec![0 as usize; MAX_BOARD_SIZE]; MAX_BOARD_SIZE];
    values.lines().for_each(|line| {
        let (start, end) = get_coords(line);

        let dy = end.y as isize - start.y as isize;
        let dx = end.x as isize - start.x as isize;
        // Manhattan distance
        let dist = (dx.abs() + dy.abs()) / {
            if dx == 0 || dy == 0 {
                1
            } else {
                2
            }
        };
        let direction_x = dx / dist;
        let direction_y = dy / dist;
        let (mut pos_x, mut pos_y) = (start.x, start.y);
        for _ in 0..=dist {
            board[pos_x][pos_y] += 1;
            pos_x = (pos_x as isize + direction_x) as usize;
            pos_y = (pos_y as isize + direction_y) as usize;
        }
    });

    board
        .iter()
        .map(|row| row.iter().filter(|&&cell| 2 <= cell).count())
        .sum::<usize>()
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
