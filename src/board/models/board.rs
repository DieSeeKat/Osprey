use std::fmt;
use crate::board::{MoveService, Move, Piece, DIAGONALS, ANTI_DIAGONALS, RANKS, FILES, FILE_A, NOT_RANK_1_8, FILE_H, NOT_RANK_1_2, RANK_2, RANK_8, KING_SPAN, FILE_G, FILE_B, NOT_RANK_7_8, RANK_7, RANK_1, KNIGHT_SPAN,};

///
/// A chess board.
///
/// Representation of pieces are done by bitboards. Each bitboard represents a type of piece.
///
/// The bitboard is a 64-bit unsigned integer. Each bit represents a square on the board.
///
/// The least significant bit represents the square a1 and the most significant bit represents the square h8.
/// The bits are ordered first from left to right and next from top to bottom.
///
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Board {
    /// A bitboard representing the white pawns.
    pub white_pawns: u64,
    /// A bitboard representing the white knights.
    pub white_knights: u64,
    /// A bitboard representing the white bishops.
    pub white_bishops: u64,
    /// A bitboard representing the white rooks.
    pub white_rooks: u64,
    /// A bitboard representing the white queens.
    pub white_queens: u64,
    /// A bitboard representing the white king.
    pub white_king: u64,
    /// A bitboard representing the black pawns.
    pub black_pawns: u64,
    /// A bitboard representing the black knights.
    pub black_knights: u64,
    /// A bitboard representing the black bishops.
    pub black_bishops: u64,
    /// A bitboard representing the black rooks.
    pub black_rooks: u64,
    /// A bitboard representing the black queens.
    pub black_queens: u64,
    /// A bitboard representing the black king.
    pub black_king: u64,
    /// A bitboard representing all squares (not the king) that can be captured by black pieces.
    pub white_pieces: u64,
    /// A bitboard representing all squares (not the king) that can be captured by white pieces.
    pub black_pieces: u64,
    /// A bitboard representing all empty squares.
    pub empty_squares: u64,
    /// The position of the en passant square as a number between 0 and 63 (both included).
    pub en_passant: Option<u8>,
    /// A boolean representing whose turn it is.
    pub white_turn: bool,
    /// A boolean representing whether white can castle kingside.
    pub white_castle_kingside: bool,
    /// A boolean representing whether white can castle queenside.
    pub white_castle_queenside: bool,
    /// A boolean representing whether black can castle kingside.
    pub black_castle_kingside: bool,
    /// A boolean representing whether black can castle queenside.
    pub black_castle_queenside: bool,
    /// The number of halfmoves since the last capture or pawn advance.
    pub halfmove: u8,
    /// The number of the full move.
    pub fullmove: u8,
}

#[allow(dead_code)]
impl Board {
    ///
    /// Creates a new board from a FEN string.
    ///
    /// Example FEN string for the starting position:
    ///
    /// ```
    /// use osprey::Board;
    ///
    /// let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    ///
    /// # Arguments
    ///
    /// * `input` - The FEN string.
    ///
    pub fn new(input: &str) -> Board {
        let mut row = 7;
        let mut col = 0;

        // split FEN string by spaces
        let fen: Vec<&str> = input.split_whitespace().collect();

        // get FEN string parts
        let fen_pieces = fen.get(0);
        let fen_turn = fen.get(1);
        let fen_castling = fen.get(2);
        let fen_en_passant = fen.get(3);
        let fen_half_move = fen.get(4);
        let fen_full_move = fen.get(5);

        // initialize bitboards
        let mut white_pawns: u64 = 0;
        let mut white_knights: u64 = 0;
        let mut white_bishops: u64 = 0;
        let mut white_rooks: u64 = 0;
        let mut white_queens: u64 = 0;
        let mut white_king: u64 = 0;
        let mut black_pawns: u64 = 0;
        let mut black_knights: u64 = 0;
        let mut black_bishops: u64 = 0;
        let mut black_rooks: u64 = 0;
        let mut black_queens: u64 = 0;
        let mut black_king: u64 = 0;

        // initialize meta data
        let white_turn;
        let mut white_castle_kingside = false;
        let mut white_castle_queenside = false;
        let mut black_castle_kingside = false;
        let mut black_castle_queenside = false;
        let mut en_passant: Option<u8> = None;
        let halfmove;
        let fullmove;

        // build bitboards from FEN string
        match fen_pieces {
            Some(fen_pieces) => {
                for c in fen_pieces.chars() {
                    if (c as u8) >= 48 && (c as u8) <= 57 {
                        col += c as u8 - 48;
                    } else if c == '/' {
                        row -= 1;
                        col = 0;
                    } else {
                        let pos: u64 = 1u64 << (row as u32 * 8 + col as u32);
                        match c {
                            'P' => white_pawns += pos,
                            'N' => white_knights += pos,
                            'B' => white_bishops += pos,
                            'R' => white_rooks += pos,
                            'Q' => white_queens += pos,
                            'K' => white_king += pos,
                            'p' => black_pawns += pos,
                            'n' => black_knights += pos,
                            'b' => black_bishops += pos,
                            'r' => black_rooks += pos,
                            'q' => black_queens += pos,
                            'k' => black_king += pos,
                            _ => panic!("Invalid FEN string"),
                        }
                        col += 1;
                    }
                }
            }
            None => panic!("Invalid FEN string"),
        }

        // set turn
        match fen_turn {
            Some(fen_turn) => match *fen_turn {
                "w" => white_turn = true,
                "b" => white_turn = false,
                _ => panic!("Invalid FEN string"),
            },
            None => panic!("Invalid FEN string"),
        }

        // set castling
        match fen_castling {
            Some(fen_castling) => {
                for c in fen_castling.chars() {
                    match c {
                        'K' => white_castle_kingside = true,
                        'Q' => white_castle_queenside = true,
                        'k' => black_castle_kingside = true,
                        'q' => black_castle_queenside = true,
                        '-' => (),
                        _ => panic!("Invalid FEN string"),
                    }
                }
            }
            None => panic!("Invalid FEN string"),
        }

        // set en passant
        match fen_en_passant {
            Some(fen_en_passant) => {
                if *fen_en_passant != "-" {
                    let col = fen_en_passant.chars().nth(0).unwrap() as u8 - 97;
                    let row = fen_en_passant.chars().nth(1).unwrap() as u8 - 49;
                    en_passant = Some(row * 8 + col);
                }
            }
            None => panic!("Invalid FEN string"),
        }

        // set halfmove and fullmove
        match fen_half_move {
            Some(fen_half_move) => {
                let half_move = fen_half_move.parse::<u8>();
                match half_move {
                    Ok(half_move) => halfmove = half_move,
                    Err(_) => panic!("Invalid FEN string"),
                }
            }
            None => panic!("Invalid FEN string"),
        }

        match fen_full_move {
            Some(fen_full_move) => {
                let full_move = fen_full_move.parse::<u8>();
                match full_move {
                    Ok(full_move) => fullmove = full_move,
                    Err(_) => panic!("Invalid FEN string"),
                }
            }
            None => panic!("Invalid FEN string"),
        }

        // set white and black pieces
        let white_pieces: u64 =
            white_pawns | white_knights | white_bishops | white_rooks | white_queens;
        let black_pieces: u64 =
            black_pawns | black_knights | black_bishops | black_rooks | black_queens;
        let empty_squares: u64 = !(white_pieces | black_pieces | white_king | black_king);

        Board {
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_king,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_king,
            white_pieces,
            black_pieces,
            empty_squares,
            white_turn,
            white_castle_kingside,
            white_castle_queenside,
            black_castle_kingside,
            black_castle_queenside,
            en_passant,
            halfmove,
            fullmove,
        }
    }

    ///
    /// Exports the board as a FEN string.
    ///
    pub fn export_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;

        for row in 0..8 {
            for col in 0..8 {
                let piece = self.square(row * 8 + col);
                match piece {
                    Some(piece) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(piece);
                    }
                    None => {
                        empty += 1;
                    }
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            if row > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');

        if self.white_turn {
            fen.push('w');
        } else {
            fen.push('b');
        }

        fen.push(' ');

        if self.white_castle_kingside {
            fen.push('K');
        }
        if self.white_castle_queenside {
            fen.push('Q');
        }
        if self.black_castle_kingside {
            fen.push('k');
        }
        if self.black_castle_queenside {
            fen.push('q');
        }
        if !self.white_castle_kingside
            && !self.white_castle_queenside
            && !self.black_castle_kingside
            && !self.black_castle_queenside
        {
            fen.push('-');
        }

        fen.push(' ');

        match self.en_passant {
            Some(en_passant) => {
                let row: u8 = (en_passant / 7) as u8;
                let col: u8 = (en_passant % 7) as u8;

                fen.push((col + 97) as char);
                fen.push((row + 49) as char);
            }
            None => {
                fen.push('-');
            }
        }

        fen.push(' ');

        fen.push_str(&self.halfmove.to_string());

        fen.push(' ');

        fen.push_str(&self.fullmove.to_string());

        fen
    }

    ///
    /// The the piece on the given square.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the square as a number between 0 and 63 (both included).
    ///
    /// # Returns
    ///
    /// The character representing the piece on the given square.
    /// [p, b, n, r, q, k, P, B, N, R, Q, K] for [black pawn, black bishop, black knight, black rook, black queen, black king, white pawn, white bishop, white knight, white rook, white queen, white king].
    fn square(&self, position: u8) -> Option<char> {
        if self.white_pawns & (1u64 << position) != 0 {
            return Some('P');
        }
        if self.white_knights & (1u64 << position) != 0 {
            return Some('N');
        }
        if self.white_bishops & (1u64 << position) != 0 {
            return Some('B');
        }
        if self.white_rooks & (1u64 << position) != 0 {
            return Some('R');
        }
        if self.white_queens & (1u64 << position) != 0 {
            return Some('Q');
        }
        if self.white_king & (1u64 << position) != 0 {
            return Some('K');
        }
        if self.black_pawns & (1u64 << position) != 0 {
            return Some('p');
        }
        if self.black_knights & (1u64 << position) != 0 {
            return Some('n');
        }
        if self.black_bishops & (1u64 << position) != 0 {
            return Some('b');
        }
        if self.black_rooks & (1u64 << position) != 0 {
            return Some('r');
        }
        if self.black_queens & (1u64 << position) != 0 {
            return Some('q');
        }
        if self.black_king & (1u64 << position) != 0 {
            return Some('k');
        }

        return None;
    }

    ///
    /// Make a move on the board.
    ///
    /// # Arguments
    ///
    /// * `m` - The move to make.
    ///
    /// # Returns
    ///
    /// A new board if the move is legal, otherwise the old board.
    ///
    #[inline]
    pub fn make_move(&self, m: &Move) -> Result<Board, Board> {
        // initialize meta data
        let mut new_en_passant = None;
        let new_white_turn = !self.white_turn;
        let mut new_white_castle_kingside = self.white_castle_kingside;
        let mut new_white_castle_queenside = self.white_castle_queenside;
        let mut new_black_castle_kingside = self.black_castle_kingside;
        let mut new_black_castle_queenside = self.black_castle_queenside;
        let new_halfmove = self.halfmove + 1;
        let new_fullmove = self.fullmove + 1;

        // set new boards
        let new_white_pawns = self.move_board(self.white_pawns, m, Piece::WhitePawn);
        let new_white_knights = self.move_board(self.white_knights, m, Piece::WhiteKnight);
        let new_white_bishops = self.move_board(self.white_bishops, m, Piece::WhiteBishop);
        let new_white_rooks = self.move_board(self.white_rooks, m, Piece::WhiteRook);
        let new_white_queens = self.move_board(self.white_queens, m, Piece::WhiteQueen);
        let new_white_king = self.move_board(self.white_king, m, Piece::WhiteKing);

        let new_black_pawns = self.move_board(self.black_pawns, m, Piece::BlackPawn);
        let new_black_knights = self.move_board(self.black_knights, m, Piece::BlackKnight);
        let new_black_bishops = self.move_board(self.black_bishops, m, Piece::BlackBishop);
        let new_black_rooks = self.move_board(self.black_rooks, m, Piece::BlackRook);
        let new_black_queens = self.move_board(self.black_queens, m, Piece::BlackQueen);
        let new_black_king = self.move_board(self.black_king, m, Piece::BlackKing);

        // get from and to
        let (from, to) = match m {
            Move::Normal { from, to } => (*from, *to),
            Move::Castle { from, to, .. } => (*from, *to),
            Move::EnPassant { from, to, .. } => (*from, *to),
            Move::Promotion { from, to, .. } => (*from, *to),
        };

        // en passant
        if to.abs_diff(from) == 16 {
            if (1u64 << from & self.black_pawns) != 0 {
                new_en_passant = Some(to + 8);
            } else if (1u64 << from & self.white_pawns) != 0 {
                new_en_passant = Some(to - 8);
            }
        }

        // castling
        new_white_castle_queenside =
            (1u64 << from & self.white_king) == 0 && new_white_castle_queenside;
        new_white_castle_kingside =
            (1u64 << from & self.white_king) == 0 && new_white_castle_kingside;

        new_black_castle_queenside =
            (1u64 << from & self.black_king) == 0 && new_black_castle_queenside;
        new_black_castle_kingside =
            (1u64 << from & self.black_king) == 0 && new_black_castle_kingside;

        new_white_castle_queenside = ((1u64 << from | 1u64 << to) & self.white_rooks & (1u64 << 0))
            == 0
            && new_white_castle_queenside;
        new_white_castle_kingside = ((1u64 << from | 1u64 << to) & self.white_rooks & 1u64 << 7)
            == 0
            && new_white_castle_kingside;

        new_black_castle_queenside =
            ((1u64 << from | 1u64 << to) & self.black_rooks & (1u64 << 56)) == 0
                && new_black_castle_queenside;
        new_black_castle_kingside = ((1u64 << from | 1u64 << to) & self.black_rooks & (1u64 << 63))
            == 0
            && new_black_castle_kingside;

        // set white and black pieces
        let new_white_pieces = new_white_pawns
            | new_white_knights
            | new_white_bishops
            | new_white_rooks
            | new_white_queens;

        let new_black_pieces = new_black_pawns
            | new_black_knights
            | new_black_bishops
            | new_black_rooks
            | new_black_queens;

        // set empty squares
        let new_empty_squares =
            !(new_white_pieces | new_black_pieces | new_white_king | new_black_king);

        // create new board
        let new_board = Board {
            white_pawns: new_white_pawns,
            black_pawns: new_black_pawns,
            white_knights: new_white_knights,
            black_knights: new_black_knights,
            white_bishops: new_white_bishops,
            black_bishops: new_black_bishops,
            white_rooks: new_white_rooks,
            black_rooks: new_black_rooks,
            white_queens: new_white_queens,
            black_queens: new_black_queens,
            white_king: new_white_king,
            black_king: new_black_king,
            white_pieces: new_white_pieces,
            black_pieces: new_black_pieces,
            empty_squares: new_empty_squares,
            en_passant: new_en_passant,
            white_turn: new_white_turn,
            white_castle_kingside: new_white_castle_kingside,
            white_castle_queenside: new_white_castle_queenside,
            black_castle_kingside: new_black_castle_kingside,
            black_castle_queenside: new_black_castle_queenside,
            halfmove: new_halfmove,
            fullmove: new_fullmove,
        };

        // check if move is legal
        if (new_board.white_king & MoveService::unsafe_w(&new_board) == 0 && self.white_turn)
            || (new_board.black_king & MoveService::unsafe_b(&new_board) == 0 && !self.white_turn)
        {
            // return new board
            return Ok(new_board);
        }

        // return old board with error
        return Err(self.clone());
    }

    ///
    /// Modifies and returns the board with the given move applied.
    ///
    /// # Arguments
    ///
    /// * `m` - The move to be applied.
    /// * `board_type` - The type of piece's board to be modified.
    ///
    /// # Returns
    ///
    /// The modified board.
    ///
    #[inline]
    fn move_board(&self, board: u64, m: &Move, board_type: Piece) -> u64 {
        // make move
        match m {
            Move::Normal { from, to } => {
                if board & (1u64 << from) == 0 {
                    // not "from" piece; empty "to" position
                    return board & !(1u64 << to);
                }

                // "from" piece; move from "from" to "to" position
                (board & !(1u64 << from)) | (1u64 << to)
            }
            Move::Castle { from, to, rook } => {
                if board & (1u64 << from) != 0 {
                    // the king bitboard
                    // move the king from "from" to "to" position
                    return (board & !(1u64 << from)) | (1u64 << to);
                }
                // not king

                if board & (1u64 << rook) != 0 {
                    // the rook bitboard
                    // calulate new rook position
                    let new_rook = if to > from { to - 1 } else { to + 1 };

                    // move the rook from "rook" to "new_rook" position
                    return (board & !(1u64 << rook)) | (1u64 << new_rook);
                }

                board
            }
            Move::EnPassant { from, to, captured } => {
                if board & (1u64 << from) != 0 {
                    // the pawn bitboard
                    // move the pawn from "from" to "to" position
                    return (board & !(1u64 << from)) | 1u64 << to;
                }

                // not pawn
                if board & (1u64 << captured) != 0 {
                    // the captured bitboard
                    // remove the captured piece from the board
                    return board & !(1u64 << captured);
                }

                board
            }
            Move::Promotion {
                from,
                to,
                promotion,
            } => {
                if board & (1u64 << from) != 0 {
                    // the pawn bitboard
                    // remove the pawn from the board
                    return board & !(1u64 << from);
                }

                if promotion == &board_type {
                    // the promoted piece bitboard
                    // add the promoted piece to the board
                    return board | (1u64 << to);
                }

                if board & (1u64 << to) != 0 {
                    // captured bitboard
                    // remove the captured piece from the board
                    return board & !(1u64 << to);
                }

                board
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for row in (0..8).rev() {
            for col in 0..8 {
                let piece = self.square(row * 8 + col);
                match piece {
                    Some(piece) => {
                        board.push(piece);
                    }
                    None => {
                        board.push('.');
                    }
                }
                if col < 7 {
                    board.push(' ');
                }
            }
            if row > 0 {
                board.push('\n');
            }
        }

        write!(f, "{}", board)
    }
}
