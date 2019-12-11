mod board;

fn main() {
    println!("Hello, world!");
}

#[test]
fn gameplay_test() {
    let mut board = board::Board::from("4 12
############
#..........#
######...#D#
############
1 1
4
3 1 255 0 0 1
5 1 0 0 255 1
6 1 0 0 255 -1
7 1 255 0 0 -2");
    let moves = vec![
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Down,
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Up,
        board::Direction::Right,
        board::Direction::Right,
        board::Direction::Down,
    ];
    for m in moves {
        board.handle_move(m);
    }
    assert!(board.player_has_won());
}