const _DUMMY_INPUT: &str = include_str!("data/day4-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day4-real.txt");

fn private_solve_part_1(values: &str) -> String {
    let mut lines = values.lines().peekable();
    if let Some(moves) = lines.next() {
        let draws = moves
            .trim()
            .split(',')
            .map(|number| number.parse::<i32>().unwrap());

        let mut boards = get_boards(lines);

        for draw in draws {
            for board in &mut boards {
                if let Some(score) = play(board, draw) {
                    return (draw * score).to_string();
                }
            }
        }

        String::new()
    } else {
        panic!("No moves found!");
    }
}

fn get_boards(mut lines: std::iter::Peekable<std::str::Lines>) -> Vec<Vec<Vec<(i32, bool)>>> {
    let mut boards = Vec::new();

    loop {
        // Consume empty line
        if let None = lines.peek() {
            break;
        }
        lines.next();
        boards.push(vec![
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| (v.parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>(),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| (v.parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>(),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| (v.parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>(),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| (v.parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>(),
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|v| (v.parse::<i32>().unwrap(), false))
                .collect::<Vec<_>>(),
        ]);
    }
    boards
}

fn play(board: &mut Vec<Vec<(i32, bool)>>, draw: i32) -> Option<i32> {
    let mut found = false;
    'outer: for row in board.iter_mut() {
        for cell in row {
            if cell.0 == draw {
                cell.1 = true;
                found = true;
                break 'outer;
            }
        }
    }
    if !found {
        return None;
    }

    if has_row_score(board) || has_col_score(board) {
        Some(get_score_sum(board))
    } else {
        None
    }
}

fn get_score_sum(board: &mut Vec<Vec<(i32, bool)>>) -> i32 {
    board.iter().fold(0, |score, row| {
        row.iter().fold(
            score,
            |score, cell| {
                if cell.1 {
                    score
                } else {
                    score + cell.0
                }
            },
        )
    })
}

fn has_row_score(board: &mut Vec<Vec<(i32, bool)>>) -> bool {
    board.iter().any(|row| row.iter().all(|cell| cell.1))
}

fn has_col_score(board: &mut Vec<Vec<(i32, bool)>>) -> bool {
    board
        .iter()
        .fold([0; 5], |mut matches, row| {
            for (count, cell) in matches.iter_mut().zip(row.iter()) {
                if cell.1 {
                    *count += 1;
                }
            }
            matches
        })
        .iter()
        .any(|&col| col == 5)
}

fn private_solve_part_2(values: &str) -> String {
    let mut lines = values.lines().peekable();
    if let Some(moves) = lines.next() {
        let draws = moves
            .trim()
            .split(',')
            .map(|number| number.parse::<i32>().unwrap());

        let mut boards = get_boards(lines);

        let mut final_score = None;
        for draw in draws {
            boards = boards
                .into_iter()
                .fold(Vec::new(), |mut boards, mut board| {
                    if let Some(score) = play(&mut board, draw) {
                        final_score = Some((draw * score).to_string());
                    } else {
                        boards.push(board);
                    }
                    boards
                });
            if boards.is_empty() {
                return final_score.unwrap();
            }
        }
        panic!("No winner!")
    } else {
        panic!("No moves found!");
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
        assert_eq!((188 * 24).to_string(), _solve_part_1_dummy());
    }
    #[test]
    fn test_part_2_dummy() {
        assert_eq!((148 * 13).to_string(), _solve_part_2_dummy());
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
