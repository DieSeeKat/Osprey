extern crate osprey;

#[cfg(test)]
mod pawn_moves {
    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn pawn_capture_nw() {
        let board = Board::new("8/8/8/p5pp/P6P/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Normal { from: 31, to: 38 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_ne() {
        let board = Board::new("8/8/8/pp5p/P6P/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Normal { from: 24, to: 33 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_n() {
        let board = Board::new("8/8/2p5/4p3/2P1P3/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Normal { from: 26, to: 34 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_nn() {
        let board = Board::new("8/8/6p1/2p1p3/p7/4P1P1/P1P5/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 8, to: 16 },
            Normal { from: 10, to: 18 },
            Normal { from: 10, to: 26 },
            Normal { from: 20, to: 28 },
            Normal { from: 22, to: 30 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_promotion_n() {
        let board = Board::new("8/3P4/8/8/8/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![
            Promotion {
                from: 51,
                to: 59,
                promotion: 'R',
            },
            Promotion {
                from: 51,
                to: 59,
                promotion: 'B',
            },
            Promotion {
                from: 51,
                to: 59,
                promotion: 'N',
            },
            Promotion {
                from: 51,
                to: 59,
                promotion: 'Q',
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_promotion_captures_n() {
        let board = Board::new("3pp3/3P4/8/8/8/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![
            Promotion {
                from: 51,
                to: 60,
                promotion: 'R',
            },
            Promotion {
                from: 51,
                to: 60,
                promotion: 'B',
            },
            Promotion {
                from: 51,
                to: 60,
                promotion: 'N',
            },
            Promotion {
                from: 51,
                to: 60,
                promotion: 'Q',
            },
        ];
        println!("{:?}", moves);
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_ne() {
        let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - e6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 35, to: 43 },
            EnPassant {
                from: 35,
                to: 44,
                captured: 36,
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_nw() {
        let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - c6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 35, to: 43 },
            EnPassant {
                from: 35,
                to: 42,
                captured: 34,
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_ne() {
        let board = Board::new("8/8/8/p6P/8/8/8/8 w - a6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Normal { from: 39, to: 47 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_nw() {
        let board = Board::new("8/8/8/P6p/8/8/8/8 w - h6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Normal { from: 32, to: 40 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_sw() {
        let board = Board::new("8/8/8/p6p/P5PP/8/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Normal { from: 39, to: 30 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_se() {
        let board = Board::new("8/8/8/p6p/PP5P/8/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Normal { from: 32, to: 25 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_s() {
        let board = Board::new("8/8/8/2p1p3/4P3/2P5/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Normal { from: 34, to: 26 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_ss() {
        let board = Board::new("8/p1p5/4p1p1/P7/2P1P3/6P1/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 48, to: 40 },
            Normal { from: 50, to: 42 },
            Normal { from: 50, to: 34 },
            Normal { from: 44, to: 36 },
            Normal { from: 46, to: 38 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_promotion_s() {
        let board = Board::new("8/8/8/8/8/8/3p4/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![
            Promotion {
                from: 11,
                to: 3,
                promotion: 'r',
            },
            Promotion {
                from: 11,
                to: 3,
                promotion: 'b',
            },
            Promotion {
                from: 11,
                to: 3,
                promotion: 'n',
            },
            Promotion {
                from: 11,
                to: 3,
                promotion: 'q',
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_promotion_captures_s() {
        let board = Board::new("8/8/8/8/8/8/3p4/3PP3 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![
            Promotion {
                from: 11,
                to: 4,
                promotion: 'r',
            },
            Promotion {
                from: 11,
                to: 4,
                promotion: 'b',
            },
            Promotion {
                from: 11,
                to: 4,
                promotion: 'n',
            },
            Promotion {
                from: 11,
                to: 4,
                promotion: 'q',
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_se() {
        let board = Board::new("8/8/8/8/2PpP3/8/8/8 w - e3 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 19 },
            EnPassant {
                from: 27,
                to: 20,
                captured: 28,
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_sw() {
        let board = Board::new("8/8/8/8/2PpP3/8/8/8 w - c3 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 19 },
            EnPassant {
                from: 27,
                to: 18,
                captured: 26,
            },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_se() {
        let board = Board::new("8/8/8/8/P6p/8/8/8 w - h6 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Normal { from: 31, to: 23 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_sw() {
        let board = Board::new("8/8/8/8/p6P/8/8/8 w - a6 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Normal { from: 24, to: 16 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod rook_moves {

    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn w_rook_move_border() {
        let board = Board::new("8/8/8/8/3R4/8/8/8 w - - 0 1");
        let moves = board.possible_wr();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 30 },
            Normal { from: 27, to: 31 },
            Normal { from: 27, to: 3 },
            Normal { from: 27, to: 11 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 59 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_rook_move_block() {
        let board = Board::new("8/3P4/8/8/P2R1P2/3P4/8/8 w - - 0 1");
        let moves = board.possible_wr();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_rook_move_capture() {
        let board = Board::new("8/3p4/8/8/p2R1p2/3p4/8/8 w - - 0 1");
        let moves = board.possible_wr();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_rook_move_border() {
        let board = Board::new("8/8/8/8/3r4/8/8/8 w - - 0 1");
        let moves = board.possible_br();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 30 },
            Normal { from: 27, to: 31 },
            Normal { from: 27, to: 3 },
            Normal { from: 27, to: 11 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 59 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_rook_move_block() {
        let board = Board::new("8/3p4/8/8/p2r1p2/3p4/8/8 w - - 0 1");
        let moves = board.possible_br();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_rook_move_capture() {
        let board = Board::new("8/3P4/8/8/P2r1P2/3P4/8/8 w - - 0 1");
        let moves = board.possible_br();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod bishop_moves {

    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn w_bishop_move_border() {
        let board = Board::new("8/8/8/8/3B4/8/8/8 w - - 0 1");
        let moves = board.possible_wb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 54 },
            Normal { from: 27, to: 63 },
            Normal { from: 27, to: 48 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 13 },
            Normal { from: 27, to: 6 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_bishop_move_block() {
        let board = Board::new("8/8/1P3P2/8/3B4/4P3/8/P7 w - - 0 1");
        let moves = board.possible_wb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 34 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_bishop_move_capture() {
        let board = Board::new("8/8/1p3p2/8/3B4/4p3/8/p7 w - - 0 1");
        let moves = board.possible_wb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_bishop_move_border() {
        let board = Board::new("8/8/8/8/3b4/8/8/8 w - - 0 1");
        let moves = board.possible_bb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 54 },
            Normal { from: 27, to: 63 },
            Normal { from: 27, to: 48 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 13 },
            Normal { from: 27, to: 6 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_bishop_move_block() {
        let board = Board::new("8/8/1p3p2/8/3b4/4p3/8/p7 w - - 0 1");
        let moves = board.possible_bb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 34 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_bishop_move_capture() {
        let board = Board::new("8/8/1P3P2/8/3b4/4P3/8/P7 w - - 0 1");
        let moves = board.possible_bb();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod queen_moves {
    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn w_queen_move_border() {
        let board = Board::new("8/8/8/8/3Q4/8/8/8 w - - 0 1");
        let moves = board.possible_wq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 54 },
            Normal { from: 27, to: 63 },
            Normal { from: 27, to: 48 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 13 },
            Normal { from: 27, to: 6 },
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 30 },
            Normal { from: 27, to: 31 },
            Normal { from: 27, to: 3 },
            Normal { from: 27, to: 11 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 59 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_queen_move_block() {
        let board = Board::new("8/3P4/1P3P2/8/P2Q1P2/3PP3/8/P7 w - - 0 1");
        let moves = board.possible_wq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 34 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_queen_move_capture() {
        let board = Board::new("8/3p4/1p3p2/8/p2Q1p2/3pp3/8/p7 w - - 0 1");
        let moves = board.possible_wq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_queen_move_border() {
        let board = Board::new("8/8/8/8/3q4/8/8/8 w - - 0 1");
        let moves = board.possible_bq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 54 },
            Normal { from: 27, to: 63 },
            Normal { from: 27, to: 48 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 13 },
            Normal { from: 27, to: 6 },
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 30 },
            Normal { from: 27, to: 31 },
            Normal { from: 27, to: 3 },
            Normal { from: 27, to: 11 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 59 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_queen_move_block() {
        let board = Board::new("8/3p4/1p3p2/8/p2q1p2/3pp3/8/p7 w - - 0 1");
        let moves = board.possible_bq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 34 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_queen_move_capture() {
        let board = Board::new("8/3P4/1P3P2/8/P2q1P2/3PP3/8/P7 w - - 0 1");
        let moves = board.possible_bq();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 24 },
            Normal { from: 27, to: 25 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 29 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 43 },
            Normal { from: 27, to: 51 },
            Normal { from: 27, to: 0 },
            Normal { from: 27, to: 9 },
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 36 },
            Normal { from: 27, to: 45 },
            Normal { from: 27, to: 41 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 20 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod knight_moves {

    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn w_knight_move() {
        let board = Board::new("8/8/8/8/3N4/8/8/8 w - - 0 1");
        let moves = board.possible_wn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 10 },
            Normal { from: 27, to: 12 },
            Normal { from: 27, to: 17 },
            Normal { from: 27, to: 21 },
            Normal { from: 27, to: 33 },
            Normal { from: 27, to: 37 },
            Normal { from: 27, to: 42 },
            Normal { from: 27, to: 44 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            println!("{:?}", m);
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_knight_capture() {
        let board = Board::new("8/8/2p1p3/1p3p2/3N4/1p3p2/2p1p3/8 w - - 0 1");
        let moves = board.possible_wn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 10 },
            Normal { from: 27, to: 12 },
            Normal { from: 27, to: 17 },
            Normal { from: 27, to: 21 },
            Normal { from: 27, to: 33 },
            Normal { from: 27, to: 37 },
            Normal { from: 27, to: 42 },
            Normal { from: 27, to: 44 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_knight_block() {
        let board = Board::new("8/8/2P1P3/1P3P2/3N4/1P3P2/2P1P3/8 w - - 0 1");
        let moves = board.possible_wn();
        let correct_moves: Vec<Move> = vec![];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_knight_border() {
        let board = Board::new("N6N/8/8/8/8/8/8/N6N w - - 0 1");
        let moves = board.possible_wn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 0, to: 10 },
            Normal { from: 0, to: 17 },
            Normal { from: 7, to: 13 },
            Normal { from: 7, to: 22 },
            Normal { from: 56, to: 41 },
            Normal { from: 56, to: 50 },
            Normal { from: 63, to: 46 },
            Normal { from: 63, to: 53 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_knight_move() {
        let board = Board::new("8/8/8/8/3n4/8/8/8 w - - 0 1");
        let moves = board.possible_bn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 10 },
            Normal { from: 27, to: 12 },
            Normal { from: 27, to: 17 },
            Normal { from: 27, to: 21 },
            Normal { from: 27, to: 33 },
            Normal { from: 27, to: 37 },
            Normal { from: 27, to: 42 },
            Normal { from: 27, to: 44 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_knight_capture() {
        let board = Board::new("8/8/2P1P3/1P3P2/3n4/1P3P2/2P1P3/8 w - - 0 1");
        let moves = board.possible_bn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 10 },
            Normal { from: 27, to: 12 },
            Normal { from: 27, to: 17 },
            Normal { from: 27, to: 21 },
            Normal { from: 27, to: 33 },
            Normal { from: 27, to: 37 },
            Normal { from: 27, to: 42 },
            Normal { from: 27, to: 44 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_knight_block() {
        let board = Board::new("8/8/2p1p3/1p3p2/3n4/1p3p2/2p1p3/8 w - - 0 1");
        let moves = board.possible_bn();
        let correct_moves: Vec<Move> = vec![];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_knight_border() {
        let board = Board::new("n6n/8/8/8/8/8/8/n6n w - - 0 1");
        let moves = board.possible_bn();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 0, to: 10 },
            Normal { from: 0, to: 17 },
            Normal { from: 7, to: 13 },
            Normal { from: 7, to: 22 },
            Normal { from: 56, to: 41 },
            Normal { from: 56, to: 50 },
            Normal { from: 63, to: 46 },
            Normal { from: 63, to: 53 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod king_move {

    use crate::osprey::{Board, Move};
    use Move::*;

    #[test]
    fn w_king_move_capture() {
        let board = Board::new("8/8/8/2p1p3/3K4/2p1p3/8/8 w - - 0 1");
        let moves = board.possible_wk();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 36 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn w_king_block() {
        let board = Board::new("8/8/8/2PPP3/2PKP3/2PPP3/8/8 w - - 0 1");
        let moves = board.possible_wk();
        let correct_moves: Vec<Move> = vec![];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_king_move_capture() {
        let board = Board::new("8/8/8/2P1P3/3k4/2P1P3/8/8 w - - 0 1");
        let moves = board.possible_bk();
        let correct_moves: Vec<Move> = vec![
            Normal { from: 27, to: 18 },
            Normal { from: 27, to: 19 },
            Normal { from: 27, to: 20 },
            Normal { from: 27, to: 26 },
            Normal { from: 27, to: 28 },
            Normal { from: 27, to: 34 },
            Normal { from: 27, to: 35 },
            Normal { from: 27, to: 36 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn b_king_block() {
        let board = Board::new("8/8/8/2ppp3/2pkp3/2ppp3/8/8 w - - 0 1");
        let moves = board.possible_bk();
        let correct_moves: Vec<Move> = vec![];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}

#[cfg(test)]
mod unsafe_squares {
    use crate::osprey::Board;

    #[test]
    fn w_unsafe_squares() {
        let board = Board::new("8/4r3/3n2b1/3p2n1/4K3/8/6q1/8 w - - 0 1");
        let unsafe_squares: u64 = board.unsafe_w();
        let correct_unsafe_squares: u64 = 1508443033184550880;
        assert_eq!(unsafe_squares, correct_unsafe_squares);
    }

    #[test]
    fn b_unsafe_squares() {
        let board = Board::new("8/4R3/3N2B1/6N1/4k3/3P4/6Q1/8 w - - 0 1");
        let unsafe_squares: u64 = board.unsafe_b();
        let correct_unsafe_squares: u64 = 1508443033184550880;
        assert_eq!(unsafe_squares, correct_unsafe_squares);
    }
}

