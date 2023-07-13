extern crate osprey;

#[cfg(test)]
mod board_move {
    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn pawn_move_n() {
        let board = Board::new("8/8/8/8/8/4P3/8/8 w - - 0 1");

        let m = Normal { from: 20, to: 28 };
        let new_white_pawns = board.move_board(&m, 'P');

        assert_eq!(new_white_pawns, 1u64 << 28);
    }

    #[test]
    fn rook_capture() {
        let board = Board::new("8/4r3/8/8/8/8/4R3/8 w - - 0 1");

        let m = Normal { from: 12, to: 52 };

        let new_white_rooks = board.move_board(&m, 'R');
        let new_black_rooks = board.move_board(&m, 'r');

        assert_eq!(new_white_rooks, 1u64 << 52);
        assert_eq!(new_black_rooks, 0u64);
    }

    #[test]
    fn pawn_promotion() {
        let board = Board::new("4p3/3P4/8/8/8/8/8/8 w - - 0 1");

        let m = Promotion {
            from: 51,
            to: 60,
            promotion: 'Q',
        };

        let new_white_queens = board.move_board(&m, 'Q');
        let new_white_pawns = board.move_board(&m, 'P');
        let new_black_pawns = board.move_board(&m, 'p');

        assert_eq!(new_white_queens, 1u64 << 60);
        assert_eq!(new_white_pawns, 0u64);
        assert_eq!(new_black_pawns, 0u64);
    }

    #[test]
    fn en_passant() {
        let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - e6 0 1");

        let m = EnPassant {
            from: 35,
            to: 44,
            captured: 36,
        };

        let new_white_pawns = board.move_board(&m, 'P');
        let new_black_pawns = board.move_board(&m, 'p');

        assert_eq!(new_white_pawns, 1u64 << 44);
        assert_eq!(new_black_pawns, 1u64 << 34);
    }

    #[test]
    fn castle_w_k() {
        let board = Board::new("8/8/8/8/8/8/8/4K2R w KQ - 0 1");

        let m = Castle {
            from: 4,
            to: 6,
            rook: 7,
        };

        let new_white_kings = board.move_board(&m, 'K');
        let new_white_rooks = board.move_board(&m, 'R');

        assert_eq!(new_white_kings, 1u64 << 6);
        assert_eq!(new_white_rooks, 1u64 << 5);
    }

    #[test]
    fn castle_w_q() {
        let board = Board::new("8/8/8/8/8/8/8/R3K3 w KQ - 0 1");

        let m = Castle {
            from: 4,
            to: 2,
            rook: 0,
        };

        let new_white_kings = board.move_board(&m, 'K');
        let new_white_rooks = board.move_board(&m, 'R');

        assert_eq!(new_white_kings, 1u64 << 2);
        assert_eq!(new_white_rooks, 1u64 << 3);
    }
}
