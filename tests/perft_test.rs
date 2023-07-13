extern crate osprey;

#[cfg(test)]
mod perft_tests {
    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn starting_position_w() {
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let moves = board.possible_white();

        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn starting_position_b() {
        let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        let moves = board.possible_black();

        assert_eq!(moves.len(), 20);
    }
}