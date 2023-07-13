use crate::board::{Board, Move};

struct Osprey {
    board: Board,
}

impl Osprey {
    fn new() -> Osprey {
        Osprey {
            board: Board::new(),
        }
    }

    fn make_move(&mut self, m: Move) {
        self.board.make_move(m);
    }

    fn draw(&self) {
        draw_bit_board(self.board.get_bit_board());
    }
}