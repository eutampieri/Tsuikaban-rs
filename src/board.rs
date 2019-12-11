type Colour = (u8, u8, u8);
type Position = (usize, usize);
type Size = Position;

#[derive(Clone)]
pub enum Block{
    Door,
    Wall,
    Block(Colour, i32),
    Empty,
}

pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl std::ops::AddAssign<&Direction> for Position {
    fn add_assign(&mut self, other: &Direction) {
        match other {
            Direction::Up => {if self.1 != 0 {self.1 = self.1 - 1}},
            Direction::Left => {if self.0 != 0 {self.0 = self.0 - 1}},
            Direction::Down => {self.1 = self.1 + 1},
            Direction::Right => {self.0 = self.0 - 1},
        }
    }
}

impl std::ops::Add<&Direction> for Position {
    type Output = Self;
    fn add(self, other: &Direction) -> Self{
        let mut res = self.clone();
        match other {
            Direction::Up => {if self.0 != 0 {res.0 = self.0 - 1}},
            Direction::Left => {if self.1 != 0 {res.1 = self.1 - 1}},
            Direction::Down => {res.0 = self.0 + 1},
            Direction::Right => {res.1 = self.1 - 1},
        }
        res
    }
}

pub struct Board {
    size: Size,
    board: Vec<Vec<Block>>,
    player_position: Position,
    player_has_won: bool,
    snapshots: Vec<(Position, Vec<Vec<Block>>)>,
}

impl Board {
    pub fn get_player_position(&self) -> Position {
        self.player_position
    }

    pub fn player_has_won(&self) -> bool {
        self.player_has_won
    }

    pub fn get_board_size(&self) -> Size {
        self.size
    }

    //pub fn get_blocks(&self) ->

    fn try_move(&mut self, coord: Position, to: &Direction) -> bool {
        let final_position = coord + &to;
        if final_position > self.size {
            return false;
        }
        // If we're trying to move the player or a block
        let move_block = coord != self.player_position;
        match self.board[final_position.0][final_position.1] {
            Block::Empty => {
                if move_block {
                    let old = self.board[coord.0][coord.1].clone();
                    self.board[coord.0][coord.1] = self.board[final_position.0][final_position.1].clone();
                    self.board[final_position.0][final_position.1] = old;
                }
                return true;
            },
            Block::Wall => {
                return false;
            },
            Block::Door => {
                // If we're trying to move a block over a door then forbid it!
                self.player_has_won = !move_block;
                return !move_block;
            },
            Block::Block(colour, value) => {
                if move_block {
                    // If we're moving a block check if we can add it
                    match self.board[coord.0][coord.1] {
                        Block::Block(current_colour, current_value) => {
                            if colour == current_colour {
                                self.board[coord.0][coord.1] = Block::Empty
                            }
                            if value == current_value {
                                // If the two blocks cancel each other
                                self.board[final_position.0][final_position.1] = Block::Empty
                            } else {
                                // Sum the blocks
                                self.board[final_position.0][final_position.1] = Block::Block(colour, value + current_value);
                            }
                        },
                        _ => {}
                    }
                }
                let can_move = self.try_move(final_position, to);
                if can_move && move_block {
                    let old = self.board[coord.0][coord.1].clone();
                    self.board[coord.0][coord.1] = self.board[final_position.0][final_position.1].clone();
                    self.board[final_position.0][final_position.1] = old;
                }
                return can_move;
            }
        }
    }

    pub fn handle_move(&mut self, direction: Direction) {
        if !self.try_move(self.player_position, &direction) {
            return;
        }
        self.player_position += &direction;
    }

    pub fn undo(&mut self) {
        unimplemented!();
    }

    pub fn from(level_data: &str) -> Self {
        let rows: Vec<String> = level_data.split("\n").map(|x| x.to_string()).collect();
        let size: Vec<usize> = rows[0].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        let board_size: Size = (size[0], size[1]);
        let mut board: Vec<Vec<Block>> = Vec::new();
        for i in 0..size[0] {
            let row: Vec<Block> = rows[i+1].chars().map(|x| {
                match x {
                    '#' => Block::Wall,
                    'D' => Block::Door,
                    _ => Block::Empty
                }
            }).collect();
            board.push(row);
        }
        let player_position_vec: Vec<usize> = rows[size[0] + 1].split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        let player_position: Position = (player_position_vec[0], player_position_vec[1]);
        let number_of_blocks = rows[size[0] + 2].parse::<u32>().unwrap();
        for i in 0..number_of_blocks {
            let block_data: Vec<i64> = rows[i as usize + size[0] + 3].split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            board[block_data[1] as usize][block_data[0] as usize] = Block::Block(
                (block_data[2] as u8, block_data[3] as u8, block_data[4] as u8),
                block_data[5] as i32
            )
        }
        Board{
            board: board,
            player_has_won: false,
            player_position: player_position,
            size: board_size,
            snapshots: Vec::new(),
        }
    }
}
