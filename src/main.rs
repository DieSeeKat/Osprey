use crate::utils::Board;
use clap::Parser;

mod utils;

#[derive(Parser)]
struct App {
    #[clap(short, long)]
    load_file: Option<String>,
    #[clap(short, long)]
    fen_string: Option<String>,
}

fn main() {
    let args = App::parse();

    // Construct the Board

    let mut fen: Option<String>;
    fen = match args.load_file {
        Some(file) => {
            let file = std::fs::read_to_string(file);

            match file {
                Ok(file) => Some(file),
                Err(_) => panic!("File not found"),
            }
        }
        None => None,
    };

    fen = match args.fen_string {
        Some(fen_string) => Some(fen_string),
        None => fen,
    };

    let fen = match fen {
        Some(fen) => fen,
        None => panic!("No FEN string provided"),
    };

    let start_board = Board::new(&fen);
    println!("====FEN String===");
    println!("-----------------");
    println!("{}", start_board.export_fen());
    println!("-----------------");
    println!("==Board Display==");
    println!("-----------------");
    println!("{}", start_board);
    println!("-----------------");
}

#[cfg(test)]
mod pawn_moves {
    use crate::utils::{Board, Move};

    #[test]
    fn pawn_capture_nw() {
        let board = Board::new("8/8/8/p5pp/P6P/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 31, to: 38 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_ne() {
        let board = Board::new("8/8/8/pp5p/P6P/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 24, to: 33 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_n() {
        let board = Board::new("8/8/2p5/4p3/2P1P3/8/8/8 w - - 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 26, to: 34 }];
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
            Move { from: 8, to: 16 },
            Move { from: 10, to: 18 },
            Move { from: 10, to: 26 },
            Move { from: 20, to: 28 },
            Move { from: 22, to: 30 },
        ];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_ne() {
        let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - e6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 35, to: 43 }, Move { from: 35, to: 44 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_nw() {
        let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - c6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 35, to: 43 }, Move { from: 35, to: 42 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_ne() {
        let board = Board::new("8/8/8/p6P/8/8/8/8 w - a6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 39, to: 47 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_nw() {
        let board = Board::new("8/8/8/P6p/8/8/8/8 w - h6 0 1");
        let moves = board.possible_wp();
        let correct_moves: Vec<Move> = vec![Move { from: 32, to: 40 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_sw() {
        let board = Board::new("8/8/8/p6p/P5PP/8/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 39, to: 30 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_capture_se() {
        let board = Board::new("8/8/8/p6p/PP5P/8/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 32, to: 25 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn pawn_move_s() {
        let board = Board::new("8/8/8/2p1p3/4P3/2P5/8/8 w - - 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 34, to: 26 }];
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
            Move { from: 48, to: 40 },
            Move { from: 50, to: 42 },
            Move { from: 50, to: 34 },
            Move { from: 44, to: 36 },
            Move { from: 46, to: 38 },
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
        let correct_moves: Vec<Move> = vec![Move { from: 27, to: 19 }, Move { from: 27, to: 20 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_sw() {
        let board = Board::new("8/8/8/8/2PpP3/8/8/8 w - c3 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 27, to: 19 }, Move { from: 27, to: 18 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_se() {
        let board = Board::new("8/8/8/8/P6p/8/8/8 w - h6 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 31, to: 23 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }

    #[test]
    fn en_passant_border_sw() {
        let board = Board::new("8/8/8/8/p6P/8/8/8 w - a6 0 1");
        let moves = board.possible_bp();
        let correct_moves: Vec<Move> = vec![Move { from: 24, to: 16 }];
        assert_eq!(moves.len(), correct_moves.len());
        for m in moves {
            assert!(correct_moves.contains(&m));
        }
    }
}
