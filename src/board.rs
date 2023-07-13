use std::fmt;

const FILE_A: u64 = 72340172838076673;
const FILE_B: u64 = 144680345676153346;
const FILE_C: u64 = 289360691352306692;
const FILE_D: u64 = 578721382704613384;
const FILE_E: u64 = 1157442765409226768;
const FILE_F: u64 = 2314885530818453536;
const FILE_G: u64 = 4629771061636907072;
const FILE_H: u64 = 9259542123273814144;
const RANK_1: u64 = 255;
const RANK_2: u64 = 65280;
const RANK_3: u64 = 16711680;
const RANK_4: u64 = 4278190080;
const RANK_5: u64 = 1095216660480;
const RANK_6: u64 = 280375465082880;
const RANK_7: u64 = 71776119061217280;
const RANK_8: u64 = 18374686479671623680;

// Left here for later use
// const CENTER: u64 = 103481868288;
// const EXTENDED_CENTER: u64 = 66229406269440;
// const KING_SIDE: u64 = 9295429630892703744;
// const QUEEN_SIDE: u64 = 4755801206503243840;
// const WHITE_SQUARES: u64 = 2863311530;
// const BLACK_SQUARES: u64 = 1431655765;

const KNIGHT_SPAN: u64 = 43234889994;
const KING_SPAN: u64 = 460039;

const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];
const DIAGONALS: [u64; 15] = [
    0x1,
    0x102,
    0x10204,
    0x1020408,
    0x102040810,
    0x10204081020,
    0x1020408102040,
    0x102040810204080,
    0x204081020408000,
    0x408102040800000,
    0x810204080000000,
    0x1020408000000000,
    0x2040800000000000,
    0x4080000000000000,
    0x8000000000000000,
];

const ANTI_DIAGONALS: [u64; 15] = [
    0x80,
    0x8040,
    0x804020,
    0x80402010,
    0x8040201008,
    0x804020100804,
    0x80402010080402,
    0x8040201008040201,
    0x4020100804020100,
    0x2010080402010000,
    0x1008040201000000,
    0x804020100000000,
    0x402010000000000,
    0x201000000000000,
    0x100000000000000,
];

/// A move of a piece on the board.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Move {
    ///
    /// A Move::Normal move.
    ///
    /// # Arguments
    ///
    /// * `from` - The position of the piece to be moved as a number between 0 and 63 (both included).
    /// * `to` - The position to move the piece to as a number between 0 and 63 (both included).
    ///
    Normal { from: u8, to: u8 },
    ///
    /// A castling move.
    ///
    /// # Arguments
    ///
    /// * `from` - The position of the king to be moved as a number between 0 and 63 (both included).
    /// * `to` - The position to move the king to as a number between 0 and 63 (both included).
    /// * `rook` - The position of the rook to be moved as a number between 0 and 63 (both included).
    ///
    Castle { from: u8, to: u8, rook: u8 },
    ///
    /// An en passant move.
    ///
    /// # Arguments
    ///
    /// * `from` - The position of the pawn to be moved as a number between 0 and 63 (both included).
    /// * `to` - The position to move the pawn to as a number between 0 and 63 (both included).
    /// * `captured` - The position of the pawn to be captured as a number between 0 and 63 (both included).
    ///
    EnPassant { from: u8, to: u8, captured: u8 },
    ///
    /// A promotion move.
    ///
    /// # Arguments
    ///
    /// * `from` - The position of the pawn to be moved as a number between 0 and 63 (both included).
    /// * `to` - The position to move the pawn to as a number between 0 and 63 (both included).
    /// * `promotion` - The piece to promote to as a character (b n r q B N R Q).
    ///
    Promotion { from: u8, to: u8, promotion: char },
}

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
    white_pawns: u64,
    /// A bitboard representing the white knights.
    white_knights: u64,
    /// A bitboard representing the white bishops.
    white_bishops: u64,
    /// A bitboard representing the white rooks.
    white_rooks: u64,
    /// A bitboard representing the white queens.
    white_queens: u64,
    /// A bitboard representing the white king.
    white_king: u64,
    /// A bitboard representing the black pawns.
    black_pawns: u64,
    /// A bitboard representing the black knights.
    black_knights: u64,
    /// A bitboard representing the black bishops.
    black_bishops: u64,
    /// A bitboard representing the black rooks.
    black_rooks: u64,
    /// A bitboard representing the black queens.
    black_queens: u64,
    /// A bitboard representing the black king.
    black_king: u64,
    /// A bitboard representing all squares (not the king) that can be captured by black pieces.
    white_pieces: u64,
    /// A bitboard representing all squares (not the king) that can be captured by white pieces.
    black_pieces: u64,
    /// A bitboard representing all empty squares.
    empty_squares: u64,
    /// The position of the en passant square as a number between 0 and 63 (both included).
    en_passant: Option<u8>,
    /// A boolean representing whose turn it is.
    pub white_turn: bool,
    /// A boolean representing whether white can castle kingside.
    white_castle_kingside: bool,
    /// A boolean representing whether white can castle queenside.
    white_castle_queenside: bool,
    /// A boolean representing whether black can castle kingside.
    black_castle_kingside: bool,
    /// A boolean representing whether black can castle queenside.
    black_castle_queenside: bool,
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
        let new_white_pawns = self.move_board(m, 'P');
        let new_white_knights = self.move_board(m, 'N');
        let new_white_bishops = self.move_board(m, 'B');
        let new_white_rooks = self.move_board(m, 'R');
        let new_white_queens = self.move_board(m, 'Q');
        let new_white_king = self.move_board(m, 'K');

        let new_black_pawns = self.move_board(m, 'p');
        let new_black_knights = self.move_board(m, 'n');
        let new_black_bishops = self.move_board(m, 'b');
        let new_black_rooks = self.move_board(m, 'r');
        let new_black_queens = self.move_board(m, 'q');
        let new_black_king = self.move_board(m, 'k');

        // get from and to
        let (from, to) = match m {
            Move::Normal { from, to } => (*from, *to),
            Move::Castle { from, to, .. } => (*from, *to),
            Move::EnPassant { from, to, .. } => (*from, *to),
            Move::Promotion {
                from,
                to,
                ..
            } => (*from, *to),
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
        if (1u64 << from & self.white_king) != 0 {
            new_white_castle_kingside = false;
            new_white_castle_queenside = false;
        } else if (1u64 << from & self.black_king) != 0 {
            new_black_castle_kingside = false;
            new_black_castle_queenside = false;
        }

        if ((1u64 << from | 1u64 << to) & self.white_rooks & (1u64 << 0)) != 0 {
            new_white_castle_queenside = false;
        } else if ((1u64 << from | 1u64 << to) & self.white_rooks & 1u64 << 7) != 0 {
            new_white_castle_kingside = false;
        }

        if ((1u64 << from | 1u64 << to) & self.black_rooks & (1u64 << 56)) != 0 {
            new_black_castle_queenside = false;
        } else if ((1u64 << from | 1u64 << to) & self.black_rooks & (1u64 << 63)) != 0 {
            new_black_castle_kingside = false;
        }

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
        if (new_board.white_king & new_board.unsafe_w() == 0 && self.white_turn)
            || (new_board.black_king & new_board.unsafe_b() == 0 && !self.white_turn)
        {
            // return new board
            return Ok(new_board);
        } else {
            // return old board with error
            return Err(self.clone());
        }
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
    pub fn move_board(&self, m: &Move, board_type: char) -> u64 {
        // get respective bitboard
        let board = match board_type {
            'P' => self.white_pawns,
            'N' => self.white_knights,
            'B' => self.white_bishops,
            'R' => self.white_rooks,
            'Q' => self.white_queens,
            'K' => self.white_king,

            'p' => self.black_pawns,
            'n' => self.black_knights,
            'b' => self.black_bishops,
            'r' => self.black_rooks,
            'q' => self.black_queens,
            'k' => self.black_king,

            _ => panic!("Invalid board type"),
        };

        // make move
        match m {
            Move::Normal { from, to } => {
                if board & (1u64 << from) == 0 {
                    // not "from" piece; empty "to" position
                    return board & !(1u64 << to);
                } else {
                    // "from" piece; move from "from" to "to" position
                    return (board & !(1u64 << from)) | (1u64 << to);
                }
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

    ///
    /// Calculates the positions possibly moved to by a horizontal or vertical slider.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the slider as a number between 0 and 63 (both included).
    ///
    /// # Returns
    ///
    /// A bitboard representing the possible positions.
    ///
    fn possible_hv(&self, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !self.empty_squares;

        let horizontal = (occupied.wrapping_sub(2u64.wrapping_mul(slider)))
            ^ (occupied
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits())))
            .reverse_bits();
        let vertical = ((occupied & FILES[position as usize % 8])
            .wrapping_sub(2u64.wrapping_mul(slider)))
            ^ ((occupied & FILES[position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits())))
            .reverse_bits();

        (horizontal & RANKS[(position / 8) as usize]) | (vertical & FILES[(position % 8) as usize])
    }

    ///
    /// Calculates the positions possibly moved to by a diagonal or anti-diagonal slider.
    ///
    /// # Arguments
    ///
    /// * `position` - The position of the slider as a number between 0 and 63 (both included).
    ///
    /// # Returns
    ///
    /// A bitboard representing the possible positions.
    ///
    fn possible_da(&self, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !self.empty_squares;

        let diagonal = (occupied & DIAGONALS[position as usize / 8 + position as usize % 8])
            .wrapping_sub(2u64.wrapping_mul(slider))
            ^ (occupied & DIAGONALS[position as usize / 8 + position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits()))
                .reverse_bits();
        let anti_diagonal = (occupied
            & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
            .wrapping_sub(2u64.wrapping_mul(slider))
            ^ (occupied & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits()))
                .reverse_bits();

        (diagonal & DIAGONALS[position as usize / 8 + position as usize % 8])
            | (anti_diagonal & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
    }

    ///
    /// Get all pseudo-legal moves (without worrying about check) white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal moves white can make.
    ///
    pub fn possible_white(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.append(&mut self.possible_wp());
        moves.append(&mut self.possible_wk());
        moves.append(&mut self.possible_wq());
        moves.append(&mut self.possible_wr());
        moves.append(&mut self.possible_wb());
        moves.append(&mut self.possible_wn());
        moves.append(&mut self.possible_wc());

        moves
    }

    ///
    /// Get all pseudo-legal moves (without worrying about check) black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal moves black can make.
    ///
    pub fn possible_black(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.append(&mut self.possible_bp());
        moves.append(&mut self.possible_bk());
        moves.append(&mut self.possible_bq());
        moves.append(&mut self.possible_br());
        moves.append(&mut self.possible_bb());
        moves.append(&mut self.possible_bn());
        moves.append(&mut self.possible_bc());

        moves
    }

    ///
    /// Get all pseudo-legal pawn moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal pawn moves white can make.
    ///
    fn possible_wp(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        // Pawn NE captures

        let mut pawn_moves =
            (self.white_pawns << 9) & !FILE_A & self.black_pieces & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i - 9, to: i });
            }
        }

        // Pawn NW captures

        pawn_moves = (self.white_pawns << 7) & !FILE_H & self.black_pieces & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i - 7, to: i });
            }
        }

        // Pawn forward one

        pawn_moves = (self.white_pawns << 8) & self.empty_squares & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i - 8, to: i });
            }
        }

        // Pawn forward two

        pawn_moves = (pawn_moves << 8)
            & ((self.white_pawns & RANK_2) << 16 & self.empty_squares & !RANK_1 & !RANK_2);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal {
                    from: i - 16,
                    to: i,
                });
            }
        }

        // Pawn Promotion

        pawn_moves = (self.white_pawns << 8) & self.empty_squares & RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['Q', 'R', 'B', 'N'].iter() {
                    moves.push(Promotion {
                        from: i - 8,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        // Pawn Promotion NE captures

        pawn_moves = (self.white_pawns << 9) & !FILE_A & self.black_pieces & RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['Q', 'R', 'B', 'N'].iter() {
                    moves.push(Promotion {
                        from: i - 9,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        // Pawn Promotion NW captures

        pawn_moves = (self.white_pawns << 7) & !FILE_H & self.black_pieces & RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['Q', 'R', 'B', 'N'].iter() {
                    moves.push(Promotion {
                        from: i - 7,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        match self.en_passant {
            Some(en_passant) => {
                pawn_moves = (self.white_pawns << 9) & !FILE_A & !RANK_1 & (1u64 << en_passant);

                if pawn_moves != 0 && self.white_turn {
                    moves.push(EnPassant {
                        from: en_passant - 9,
                        to: en_passant,
                        captured: en_passant - 8,
                    });
                }

                // Pawn NW en passant

                pawn_moves = (self.white_pawns << 7) & !FILE_H & !RANK_1 & (1u64 << en_passant);

                if pawn_moves != 0 && self.white_turn {
                    moves.push(EnPassant {
                        from: en_passant - 7,
                        to: en_passant,
                        captured: en_passant - 8,
                    });
                }
            }
            None => {}
        }

        moves
    }

    ///
    /// Get all pseudo-legal knight moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal knight moves white can make.
    fn possible_wn(&self) -> Vec<Move> {

        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.white_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.white_pieces & !self.white_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.white_pieces & !self.white_king;
                }

                for j in 0..64 {
                    if possibility & 1u64 << j != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal bishop moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal bishop moves white can make.
    ///
    fn possible_wb(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.white_bishops & (1u64 << i) != 0 {
                let bishop_moves = self.possible_da(i) & !self.white_pieces & !self.white_king;

                for j in 0..64 {
                    if bishop_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal rook moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal rook moves white can make.
    ///     
    fn possible_wr(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.white_rooks & (1u64 << i) != 0 {
                let rook_moves = self.possible_hv(i) & !self.white_pieces & !self.white_king;

                for j in 0..64 {
                    if rook_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal queen moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal queen moves white can make.
    ///
    fn possible_wq(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.white_queens & (1u64 << i) != 0 {
                let queen_moves = (self.possible_hv(i) | self.possible_da(i))
                    & !self.white_pieces
                    & !self.white_king;

                for j in 0..64 {
                    if queen_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal king moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal king moves white can make.
    ///
    fn possible_wk(&self) -> Vec<Move> {

        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.white_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.white_pieces;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.white_pieces;
                }

                for j in 0..64 {
                    if possibility & 1u64 << j != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }

                break;
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal pawn moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal pawn moves black can make.
    ///
    fn possible_bp(&self) -> Vec<Move> {

        let mut moves: Vec<Move> = Vec::new();

        // Pawn SW captures

        let mut pawn_moves =
            (self.black_pawns >> 9) & !FILE_H & self.white_pieces & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i + 9, to: i });
            }
        }

        // Pawn SE captures

        pawn_moves = (self.black_pawns >> 7) & !FILE_A & self.white_pieces & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i + 7, to: i });
            }
        }

        // Pawn forward one

        pawn_moves = (self.black_pawns >> 8) & self.empty_squares & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal { from: i + 8, to: i });
            }
        }

        // Pawn forward two

        pawn_moves = (pawn_moves >> 8)
            & ((self.black_pawns & RANK_7) >> 16 & self.empty_squares & !RANK_8 & !RANK_7);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move::Normal {
                    from: i + 16,
                    to: i,
                });
            }
        }

        // Pawn Promotion

        pawn_moves = (self.black_pawns >> 8) & self.empty_squares & RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['q', 'r', 'b', 'n'].iter() {
                    moves.push(Move::Promotion {
                        from: i + 8,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        // Pawn Promotion SW captures

        pawn_moves = (self.black_pawns >> 9) & !FILE_H & self.white_pieces & RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['q', 'r', 'b', 'n'].iter() {
                    moves.push(Move::Promotion {
                        from: i + 9,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        // Pawn Promotion SE captures

        pawn_moves = (self.black_pawns >> 7) & !FILE_A & self.white_pieces & RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                for promotion in ['q', 'r', 'b', 'n'].iter() {
                    moves.push(Move::Promotion {
                        from: i + 7,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        match self.en_passant {
            Some(en_passant) => {
                // Pawn SW en passant

                pawn_moves = (self.black_pawns >> 9) & !FILE_H & !RANK_8 & (1u64 << en_passant);

                if pawn_moves != 0 && !self.white_turn {
                    moves.push(Move::EnPassant {
                        from: en_passant + 9,
                        to: en_passant,
                        captured: en_passant + 8,
                    });
                }

                // Pawn SE en passant

                pawn_moves = (self.black_pawns >> 7) & !FILE_A & !RANK_8 & (1u64 << en_passant);

                if pawn_moves != 0 && !self.white_turn {
                    moves.push(Move::EnPassant {
                        from: en_passant + 7,
                        to: en_passant,
                        captured: en_passant + 8,
                    });
                }
            }
            None => {}
        }

        moves
    }

    ///
    /// Get all pseudo-legal knight moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal knight moves black can make.
    ///
    fn possible_bn(&self) -> Vec<Move> {

        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.black_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.black_pieces & !self.black_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.black_pieces & !self.black_king;
                }

                for j in 0..64 {
                    if possibility & 1u64 << j != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal bishop moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal bishop moves black can make.
    ///
    fn possible_bb(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.black_bishops & (1u64 << i) != 0 {
                let bishop_moves = self.possible_da(i) & !self.black_pieces & !self.black_king;

                for j in 0..64 {
                    if bishop_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal rook moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal rook moves black can make.
    ///
    fn possible_br(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.black_rooks & (1u64 << i) != 0 {
                let rook_moves = self.possible_hv(i) & !self.black_pieces & !self.black_king;

                for j in 0..64 {
                    if rook_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal queen moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal queen moves black can make.
    ///
    fn possible_bq(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.black_queens & (1u64 << i) != 0 {
                let queen_moves = (self.possible_hv(i) | self.possible_da(i))
                    & !self.black_pieces
                    & !self.black_king;

                for j in 0..64 {
                    if queen_moves & (1u64 << j) != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    ///
    /// Get all pseudo-legal king moves black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal king moves black can make.
    ///
    fn possible_bk(&self) -> Vec<Move> {

        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if self.black_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.black_pieces;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.black_pieces;
                }

                for j in 0..64 {
                    if possibility & 1u64 << j != 0 {
                        moves.push(Move::Normal { from: i, to: j });
                    }
                }

                break;
            }
        }

        moves
    }

    ///
    /// Get all castle moves white can make.
    ///
    /// # Returns
    ///
    /// A vector of all castle moves white can make.
    ///
    fn possible_wc(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        if self.white_castle_kingside {
            let unsafe_w = self.unsafe_w();

            if unsafe_w & (1u64 << 4) == 0
                && (unsafe_w | !self.empty_squares) & 1u64 << 5 == 0
                && (unsafe_w | !self.empty_squares) & 1u64 << 6 == 0
                && self.white_rooks & 1u64 << 7 != 0
            {
                moves.push(Castle {
                    from: 4,
                    to: 6,
                    rook: 7,
                });
            }
        }

        if self.white_castle_queenside {
            let unsafe_w = self.unsafe_w();
            if !self.empty_squares & (1u64 << 1) == 0
                && (unsafe_w | !self.empty_squares) & (1u64 << 2) == 0
                && (unsafe_w | !self.empty_squares) & (1u64 << 3) == 0
                && unsafe_w & (1u64 << 4) == 0
            {
                moves.push(Castle {
                    from: 4,
                    to: 2,
                    rook: 0,
                });
            }
        }

        moves
    }

    ///
    /// Get all castle moves black can make.
    ///     
    /// # Returns
    ///
    /// A vector of all castle moves black can make.
    ///
    fn possible_bc(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        if self.black_castle_kingside {
            let unsafe_b = self.unsafe_b();
            if unsafe_b & (1u64 << 60) == 0
                && (unsafe_b | !self.empty_squares) & (1u64 << 61) == 0
                && (unsafe_b | !self.empty_squares) & (1u64 << 62) == 0
                && self.black_rooks & (1u64 << 63) != 0
            {
                moves.push(Castle {
                    from: 60,
                    to: 62,
                    rook: 63,
                });
            }
        }

        if self.black_castle_queenside {
            let unsafe_b = self.unsafe_b();
            if self.black_rooks & (1u64 << 56) != 0
                && !self.empty_squares & (1u64 << 57) == 0
                && (unsafe_b | !self.empty_squares) & (1u64 << 58) == 0
                && (unsafe_b | !self.empty_squares) & (1u64 << 59) == 0
                && unsafe_b & (1u64 << 60) == 0
            {
                moves.push(Castle {
                    from: 60,
                    to: 58,
                    rook: 56,
                });
            }
        }

        moves
    }

    ///
    /// Get all unsafe squares for white.
    ///
    /// # Returns
    ///
    /// A bitboard representing all squares attacked by black.
    ///
    fn unsafe_w(&self) -> u64 {
        let mut unsafe_squares: u64 = 0;

        // pawn
        unsafe_squares |= (self.black_pawns >> 7) & !FILE_A;
        unsafe_squares |= (self.black_pawns >> 9) & !FILE_H;

        // knight

        for i in 0..64 {
            if self.black_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.black_pieces & !self.black_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.black_pieces & !self.black_king;
                };

                unsafe_squares |= possibility;
            }
        }

        // bishop | queen

        let bishop_queen = self.black_bishops | self.black_queens;

        for i in 0..64 {
            if bishop_queen & 1u64 << i != 0 {
                unsafe_squares |= self.possible_da(i);
            }
        }

        // rook | queen

        let rook_queen = self.black_rooks | self.black_queens;

        for i in 0..64 {
            if rook_queen & 1u64 << i != 0 {
                unsafe_squares |= self.possible_hv(i);
            }
        }

        // king

        for i in 0..64 {
            if self.black_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H);
                } else {
                    possibility &= !(FILE_A | FILE_B);
                }

                unsafe_squares |= possibility;
            }
        }

        unsafe_squares
    }

    ///
    /// Get all unsafe squares for black.
    ///
    /// # Returns
    ///
    /// A bitboard representing all squares attacked by white.
    ///
    fn unsafe_b(&self) -> u64 {
        let mut unsafe_squares: u64 = 0;

        // pawn
        unsafe_squares |= (self.white_pawns << 7) & !FILE_H;
        unsafe_squares |= (self.white_pawns << 9) & !FILE_A;

        // knight

        for i in 0..64 {
            if self.white_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !self.white_pieces & !self.white_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !self.white_pieces & !self.white_king;
                };

                unsafe_squares |= possibility;
            }
        }

        // bishop | queen

        let bishop_queen = self.white_bishops | self.white_queens;

        for i in 0..64 {
            if bishop_queen & 1u64 << i != 0 {
                unsafe_squares |= self.possible_da(i);
            }
        }

        // rook | queen

        let rook_queen = self.white_rooks | self.white_queens;

        for i in 0..64 {
            if rook_queen & 1u64 << i != 0 {
                unsafe_squares |= self.possible_hv(i);
            }
        }

        // king

        for i in 0..64 {
            if self.white_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H);
                } else {
                    possibility &= !(FILE_A | FILE_B);
                }

                unsafe_squares |= possibility;
            }
        }

        unsafe_squares
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

#[test]
fn make_pawn_move_n() {
    let board = Board::new("8/8/8/8/8/4P3/8/8 w - - 0 1");

    let m = Move::Normal { from: 20, to: 28 };
    let new_white_pawns = board.move_board(&m, 'P');

    assert_eq!(new_white_pawns, 1u64 << 28);
}

#[test]
fn make_rook_capture() {
    let board = Board::new("8/4r3/8/8/8/8/4R3/8 w - - 0 1");

    let m = Move::Normal { from: 12, to: 52 };

    let new_white_rooks = board.move_board(&m, 'R');
    let new_black_rooks = board.move_board(&m, 'r');

    assert_eq!(new_white_rooks, 1u64 << 52);
    assert_eq!(new_black_rooks, 0u64);
}

#[test]
fn make_pawn_promotion() {
    let board = Board::new("4p3/3P4/8/8/8/8/8/8 w - - 0 1");

    let m = Move::Promotion {
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
fn make_en_passant() {
    let board = Board::new("8/8/8/2pPp3/8/8/8/8 w - e6 0 1");

    let m = Move::EnPassant {
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
fn make_castle_w_k() {
    let board = Board::new("8/8/8/8/8/8/8/4K2R w KQ - 0 1");

    let m = Move::Castle {
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
fn make_castle_w_q() {
    let board = Board::new("8/8/8/8/8/8/8/R3K3 w KQ - 0 1");

    let m = Move::Castle {
        from: 4,
        to: 2,
        rook: 0,
    };

    let new_white_kings = board.move_board(&m, 'K');
    let new_white_rooks = board.move_board(&m, 'R');

    assert_eq!(new_white_kings, 1u64 << 2);
    assert_eq!(new_white_rooks, 1u64 << 3);
}

#[test]
fn pawn_capture_nw() {
    let board = Board::new("8/8/8/p5pp/P6P/8/8/8 w - - 0 1");
    let moves = board.possible_wp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 31, to: 38 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_ne() {
    let board = Board::new("8/8/8/pp5p/P6P/8/8/8 w - - 0 1");
    let moves = board.possible_wp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 24, to: 33 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_n() {
    let board = Board::new("8/8/2p5/4p3/2P1P3/8/8/8 w - - 0 1");
    let moves = board.possible_wp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 26, to: 34 }];
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
        Move::Normal { from: 8, to: 16 },
        Move::Normal { from: 10, to: 18 },
        Move::Normal { from: 10, to: 26 },
        Move::Normal { from: 20, to: 28 },
        Move::Normal { from: 22, to: 30 },
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
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: 'R',
        },
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: 'B',
        },
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: 'N',
        },
        Move::Promotion {
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
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: 'R',
        },
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: 'B',
        },
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: 'N',
        },
        Move::Promotion {
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
        Move::Normal { from: 35, to: 43 },
        Move::EnPassant {
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
        Move::Normal { from: 35, to: 43 },
        Move::EnPassant {
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
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 39, to: 47 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn en_passant_border_nw() {
    let board = Board::new("8/8/8/P6p/8/8/8/8 w - h6 0 1");
    let moves = board.possible_wp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 32, to: 40 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_sw() {
    let board = Board::new("8/8/8/p6p/P5PP/8/8/8 w - - 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 39, to: 30 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_se() {
    let board = Board::new("8/8/8/p6p/PP5P/8/8/8 w - - 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 32, to: 25 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_s() {
    let board = Board::new("8/8/8/2p1p3/4P3/2P5/8/8 w - - 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 34, to: 26 }];
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
        Move::Normal { from: 48, to: 40 },
        Move::Normal { from: 50, to: 42 },
        Move::Normal { from: 50, to: 34 },
        Move::Normal { from: 44, to: 36 },
        Move::Normal { from: 46, to: 38 },
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
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: 'r',
        },
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: 'b',
        },
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: 'n',
        },
        Move::Promotion {
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
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: 'r',
        },
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: 'b',
        },
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: 'n',
        },
        Move::Promotion {
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
    let board = Board::new("8/8/8/8/2PpP3/8/8/8 b - e3 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 19 },
        Move::EnPassant {
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
    let board = Board::new("8/8/8/8/2PpP3/8/8/8 b - c3 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 19 },
        Move::EnPassant {
            from: 27,
            to: 18,
            captured: 26,
        },
    ];
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn en_passant_border_se() {
    let board = Board::new("8/8/8/8/P6p/8/8/8 b - h6 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 31, to: 23 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn en_passant_border_sw() {
    let board = Board::new("8/8/8/8/p6P/8/8/8 b - a6 0 1");
    let moves = board.possible_bp();
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 24, to: 16 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_rook_move_border() {
    let board = Board::new("8/8/8/8/3R4/8/8/8 w - - 0 1");
    let moves = board.possible_wr();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 30 },
        Move::Normal { from: 27, to: 31 },
        Move::Normal { from: 27, to: 3 },
        Move::Normal { from: 27, to: 11 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 59 },
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
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
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
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
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
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 30 },
        Move::Normal { from: 27, to: 31 },
        Move::Normal { from: 27, to: 3 },
        Move::Normal { from: 27, to: 11 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 59 },
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
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
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
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn overflow() {
    let board = Board::new("8/8/8/8/8/8/8/B7 w - - 0 1");
    let moves = board.possible_wb();

    assert_eq!(moves.len(), 7);
}

#[test]
fn w_bishop_move_border() {
    let board = Board::new("8/8/8/8/3B4/8/8/8 w - - 0 1");
    let moves = board.possible_wb();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 54 },
        Move::Normal { from: 27, to: 63 },
        Move::Normal { from: 27, to: 48 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 13 },
        Move::Normal { from: 27, to: 6 },
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
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 34 },
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
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
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
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 54 },
        Move::Normal { from: 27, to: 63 },
        Move::Normal { from: 27, to: 48 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 13 },
        Move::Normal { from: 27, to: 6 },
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
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 34 },
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
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_queen_move_border() {
    let board = Board::new("8/8/8/8/3Q4/8/8/8 w - - 0 1");
    let moves = board.possible_wq();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 54 },
        Move::Normal { from: 27, to: 63 },
        Move::Normal { from: 27, to: 48 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 13 },
        Move::Normal { from: 27, to: 6 },
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 30 },
        Move::Normal { from: 27, to: 31 },
        Move::Normal { from: 27, to: 3 },
        Move::Normal { from: 27, to: 11 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 59 },
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
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 34 },
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
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
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
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 54 },
        Move::Normal { from: 27, to: 63 },
        Move::Normal { from: 27, to: 48 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 13 },
        Move::Normal { from: 27, to: 6 },
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 30 },
        Move::Normal { from: 27, to: 31 },
        Move::Normal { from: 27, to: 3 },
        Move::Normal { from: 27, to: 11 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 59 },
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
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 34 },
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
        Move::Normal { from: 27, to: 24 },
        Move::Normal { from: 27, to: 25 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 29 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 43 },
        Move::Normal { from: 27, to: 51 },
        Move::Normal { from: 27, to: 0 },
        Move::Normal { from: 27, to: 9 },
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 36 },
        Move::Normal { from: 27, to: 45 },
        Move::Normal { from: 27, to: 41 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 20 },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_knight_move() {
    let board = Board::new("8/8/8/8/3N4/8/8/8 w - - 0 1");
    let moves = board.possible_wn();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 10 },
        Move::Normal { from: 27, to: 12 },
        Move::Normal { from: 27, to: 17 },
        Move::Normal { from: 27, to: 21 },
        Move::Normal { from: 27, to: 33 },
        Move::Normal { from: 27, to: 37 },
        Move::Normal { from: 27, to: 42 },
        Move::Normal { from: 27, to: 44 },
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
        Move::Normal { from: 27, to: 10 },
        Move::Normal { from: 27, to: 12 },
        Move::Normal { from: 27, to: 17 },
        Move::Normal { from: 27, to: 21 },
        Move::Normal { from: 27, to: 33 },
        Move::Normal { from: 27, to: 37 },
        Move::Normal { from: 27, to: 42 },
        Move::Normal { from: 27, to: 44 },
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
        Move::Normal { from: 0, to: 10 },
        Move::Normal { from: 0, to: 17 },
        Move::Normal { from: 7, to: 13 },
        Move::Normal { from: 7, to: 22 },
        Move::Normal { from: 56, to: 41 },
        Move::Normal { from: 56, to: 50 },
        Move::Normal { from: 63, to: 46 },
        Move::Normal { from: 63, to: 53 },
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
        Move::Normal { from: 27, to: 10 },
        Move::Normal { from: 27, to: 12 },
        Move::Normal { from: 27, to: 17 },
        Move::Normal { from: 27, to: 21 },
        Move::Normal { from: 27, to: 33 },
        Move::Normal { from: 27, to: 37 },
        Move::Normal { from: 27, to: 42 },
        Move::Normal { from: 27, to: 44 },
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
        Move::Normal { from: 27, to: 10 },
        Move::Normal { from: 27, to: 12 },
        Move::Normal { from: 27, to: 17 },
        Move::Normal { from: 27, to: 21 },
        Move::Normal { from: 27, to: 33 },
        Move::Normal { from: 27, to: 37 },
        Move::Normal { from: 27, to: 42 },
        Move::Normal { from: 27, to: 44 },
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
        Move::Normal { from: 0, to: 10 },
        Move::Normal { from: 0, to: 17 },
        Move::Normal { from: 7, to: 13 },
        Move::Normal { from: 7, to: 22 },
        Move::Normal { from: 56, to: 41 },
        Move::Normal { from: 56, to: 50 },
        Move::Normal { from: 63, to: 46 },
        Move::Normal { from: 63, to: 53 },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_king_move_capture() {
    let board = Board::new("8/8/8/2p1p3/3K4/2p1p3/8/8 w - - 0 1");
    let moves = board.possible_wk();
    let correct_moves: Vec<Move> = vec![
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 36 },
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
        Move::Normal { from: 27, to: 18 },
        Move::Normal { from: 27, to: 19 },
        Move::Normal { from: 27, to: 20 },
        Move::Normal { from: 27, to: 26 },
        Move::Normal { from: 27, to: 28 },
        Move::Normal { from: 27, to: 34 },
        Move::Normal { from: 27, to: 35 },
        Move::Normal { from: 27, to: 36 },
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

#[test]
fn castling_w_kq() {
    let board = Board::new("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![
        Move::Castle {
            from: 4,
            to: 6,
            rook: 7,
        },
        Move::Castle {
            from: 4,
            to: 2,
            rook: 0,
        },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_w_k() {
    let board = Board::new("8/8/8/8/8/8/8/R3K2R w K - 0 1");
    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![Move::Castle {
        from: 4,
        to: 6,
        rook: 7,
    }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_w_q() {
    let board = Board::new("8/8/8/8/8/8/8/R3K2R w Q - 0 1");
    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![Move::Castle {
        from: 4,
        to: 2,
        rook: 0,
    }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_w_unsafe() {
    let board = Board::new("2r3r1/8/8/8/8/8/8/R3K2R w KQ - 0 1");

    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_w_checked() {
    let board = Board::new("4r3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_w_blocked() {
    let board = Board::new("8/8/8/8/8/8/8/R2PKP1R w KQ - 0 1");
    let moves = board.possible_wc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_kq() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/8 w kq - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![
        Move::Castle {
            from: 60,
            to: 62,
            rook: 63,
        },
        Move::Castle {
            from: 60,
            to: 58,
            rook: 56,
        },
    ];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_b_k() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/8 w k - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![Move::Castle {
        from: 60,
        to: 62,
        rook: 63,
    }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_b_q() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/8 w q - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![Move::Castle {
        from: 60,
        to: 58,
        rook: 56,
    }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_b_unsafe() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/2R2R2 w q - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_checked() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/4R3 w q - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_blocked() {
    let board = Board::new("r2pkp1r/8/8/8/8/8/8/8 w q - 0 1");
    let moves = board.possible_bc();
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

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
