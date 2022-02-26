const _DUMMY_INPUT: &str = include_str!("data/day4-dummy.txt");
const REAL_INPUT: &str = include_str!("data/day4-real.txt");
const BINGO_BOARD_SIZE: usize = 5;

type BoardCell = (i32, bool);
type BoardRow = [BoardCell; BINGO_BOARD_SIZE];
type Board = [BoardRow; BINGO_BOARD_SIZE];
type Boards = Vec<Board>;

fn parse_bingo_games(values: &str) -> (impl Iterator<Item = i32> + '_, Boards) {
    let mut lines = values.lines().peekable();
    let draws = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|number| number.parse::<i32>().unwrap());
    let boards = get_boards(lines);

    (draws, boards)
}

fn get_boards(mut lines: std::iter::Peekable<std::str::Lines>) -> Boards {
    let mut boards: Boards = Vec::new();

    loop {
        // Consume empty line
        if let None = lines.peek() {
            break;
        }
        lines.next();
        boards.push([
            get_cells(&mut lines),
            get_cells(&mut lines),
            get_cells(&mut lines),
            get_cells(&mut lines),
            get_cells(&mut lines),
        ]);
    }
    boards
}

fn get_cells(lines: &mut std::iter::Peekable<std::str::Lines>) -> BoardRow {
    let mut cells = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| (v.parse::<i32>().unwrap(), false));
    [
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
        cells.next().unwrap(),
    ]
}

fn play(board: &mut Board, draw: i32) -> Option<i32> {
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

    has_score(board).then(|| get_score_sum(board))
    // if has_score(&board) {
    //     Some(get_score_sum(board))
    // } else {
    //     None
    // }
}

fn has_score(board: &[BoardRow]) -> bool {
    for n in 0..BINGO_BOARD_SIZE {
        let mut r = 0;
        let mut c = 0;
        for m in 0..BINGO_BOARD_SIZE {
            if board[n][m].1 {
                r += 1
            }
            if board[m][n].1 {
                c += 1
            }
        }
        if r == BINGO_BOARD_SIZE || c == BINGO_BOARD_SIZE {
            return true;
        }
    }
    false
}

fn get_score_sum(board: &mut Board) -> i32 {
    board.iter().fold(0, |score, row| {
        row.iter().fold(score, |score, cell| {
            // Funcional solution
            // score + cell.1.then(|| 0).unwrap_or(cell.0)
            if cell.1 {
                score
            } else {
                score + cell.0
            }
        })
    })
}

fn private_solve_part_1(values: &str) -> String {
    let (draws, mut boards) = parse_bingo_games(values);

    for draw in draws {
        for board in &mut boards {
            if let Some(score) = play(board, draw) {
                return (draw * score).to_string();
            }
        }
    }
    panic!("No winner!");
}

fn private_solve_part_2(values: &str) -> String {
    let (draws, mut boards) = parse_bingo_games(values);

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
