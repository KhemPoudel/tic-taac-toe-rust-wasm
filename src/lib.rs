use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_two_nums(a: i32, b:i32) -> i32 {
    a + b
}

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State {
    DRAW,
    RESULTED,
    INPROGRESS,
}

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Player {
    X = 1,
    O = 2,
    EMPTY = 0
}

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Board {
    matrix: Vec<Player>,
    moves: Vec<usize>,
    status: State,
    turn: Player,
    winner: Player,
}
#[wasm_bindgen]
impl Board {

    #[wasm_bindgen(constructor)]
    pub fn new(start_player: Player) -> Self {
        Board {
            matrix: vec![Player::EMPTY, Player::EMPTY, Player::EMPTY,
                Player::EMPTY, Player::EMPTY, Player::EMPTY,
                Player::EMPTY, Player::EMPTY, Player::EMPTY
            ],
            moves: vec![],
            status: State::INPROGRESS,
            turn: start_player,
            winner: Player::EMPTY,
        }
    }

    #[wasm_bindgen]
    pub fn get_current_turn(&self) -> Player {
        self.turn.clone()
    }

    #[wasm_bindgen(catch)]
    pub fn make_move(&mut self, move_position: usize) -> Result<(), JsValue> {
        if move_position > 8 {
            Err(JsValue::from("Illegal Position Supplied. Try Again."))
        } else if self.moves.contains(&move_position) {
            Err(JsValue::from("Position Already Filled. Try Again"))
        } else {
            self.matrix[move_position] = self.turn.clone();
            self.moves.push(move_position);
            self.change_turn();
            self.change_board_state();
            Ok(())
        }
    }

    fn undo_move(&mut self) {
        let move_position = self.moves.pop().unwrap();
        self.matrix[move_position] = Player::EMPTY;
        self.change_turn();
        self.change_board_state();
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            Player::X => Player::O,
            Player::O => Player::X,
            _ => self.turn.clone(),
        };
    }

    fn change_board_state(&mut self) {
        let len: usize = self.moves.len();
        if len == 0 {
            return ();
        }

        let &move_position = self.moves.get(len - 1).unwrap();

        let row: usize = move_position.div_euclid(3);
        let col: usize = move_position.rem_euclid(3);

        let row_complete = self.matrix[move_position] == self.matrix[row * 3]
            && self.matrix[row * 3] == self.matrix[row * 3 + 1]
            && self.matrix[row * 3 + 1] == self.matrix[row * 3 + 2];

        let col_complete = self.matrix[move_position] == self.matrix[col]
            && self.matrix[col] == self.matrix[col + 3]
            && self.matrix[col + 3] == self.matrix[col + 6];

        let main_diag_complete = self.matrix[move_position] == self.matrix[0]
            && self.matrix[0] == self.matrix[4]
            && self.matrix[4] == self.matrix[8];

        let sec_diag_complete = self.matrix[move_position] == self.matrix[2]
            && self.matrix[2] == self.matrix[4]
            && self.matrix[4] == self.matrix[6];

        if row_complete || col_complete || main_diag_complete || sec_diag_complete {
            self.status = State::RESULTED;
            self.winner = self.matrix[move_position].clone();
        } else if self.moves.len() >= 9 {
            self.status = State::DRAW;
        } else {
            self.status = State::INPROGRESS;
        }
    }

    #[wasm_bindgen]
    pub fn get_best_move(&mut self) -> usize {
        let mut best_score = -1000;
        let mut best_move: usize = 0;
        for mv in find_available_moves(self) {
            self.make_move(mv);
            let score = minimax(self, &self.turn.clone());
            if score > best_score {
                best_score = score;
                best_move = mv;
            }
            self.undo_move();
        }

        best_move
    }

    #[wasm_bindgen]
    pub fn get_board_state(&self) -> State {
        self.status.clone()
    }

    #[wasm_bindgen]
    pub fn get_winner(&self) -> Player {
        self.winner.clone()
    }
}

fn find_available_moves(board: &Board) -> Vec<usize> {
    let mut available_moves: Vec<usize> = vec![];

    for (index, player) in board.matrix.iter().enumerate() {
        if player == &Player::EMPTY {
            available_moves.push(index);
        }
    }

    available_moves
}

fn minimax(board: &mut Board, mover: &Player) -> i32 {
    if board.status == State::RESULTED {
        if &board.winner != mover { return 1; } else { return -1; };
    } else if board.status == State::DRAW { return 0; }

    let is_max = &board.turn != mover;
    let mut best_score = if is_max{ -1000 } else { 1000 };
    for mv in find_available_moves(&board) {
        board.make_move(mv);
        let score = minimax(board, mover);
        if is_max && score > best_score {
            best_score = score;
        }
        if !is_max && score < best_score {
            best_score = score;
        }
        board.undo_move();
    }

    best_score

}
