const PAWN_TAKES: [i32; 2] = [7, 9];
const PAWN_MOVES: [i32; 1] = [8];
const KNIGHT_MOVES: [i32; 8] = [17, 15, 10, 6, -17, -15, -10, -6];
const BISHOP_MOVES: [i32; 13] = [72, 63, 54, 45, 36, 27, 21, 18, 14, 9, 7, -9, -7];
const ROOK_MOVES: [i32; 14] = [56, 48, 40, 32, 24, 16, 8, -8, -16, -24, -32, -40, -48, -56];
const QUEEN_MOVES: [i32; 27] = [
    72, 63, 54, 45, 36, 27, 21, 18, 14, 9, 7, -9, -7, 56, 48, 40, 32, 24, 16, 8, -8, -16, -24, -32,
    -40, -48, -56,
];
const KING_MOVES: [i32; 8] = [9, 8, 7, 1, -1, -7, -8, -9];

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
const CENTER: u64 = 103481868288;
const EXTENDED_CENTER: u64 = 66229406269440;
const KING_SIDE: u64 = 9295429630892703744;
const QUEEN_SIDE: u64 = 4755801206503243840;
const WHITE_SQUARES: u64 = 2863311530;
const BLACK_SQUARES: u64 = 1431655765;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
}

#[derive(Debug)]
pub struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
    /* All squares that black can capture (white but not king) */
    white_pieces: u64,
    /* All squares that white can capture (black but not king) */
    black_pieces: u64,
    /* All empty squares */
    empty_squares: u64,
    en_passant: u8,
    white_turn: bool,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    halfmove: u8,
    fullmove: u8,
}

impl Board {
    pub fn new(input: &str) -> Board {
        let mut row = 7;
        let mut col = 0;

        let fen: Vec<&str> = input.split_whitespace().collect();

        let fen_pieces = fen.get(0);
        let fen_turn = fen.get(1);
        let fen_castling = fen.get(2);
        let fen_en_passant = fen.get(3);
        let fen_half_move = fen.get(4);
        let fen_full_move = fen.get(5);

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
        let white_turn;
        let mut white_castle_kingside = false;
        let mut white_castle_queenside = false;
        let mut black_castle_kingside = false;
        let mut black_castle_queenside = false;
        let mut en_passant: u8 = 0;
        let halfmove;
        let fullmove;

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

        match fen_turn {
            Some(fen_turn) => match *fen_turn {
                "w" => white_turn = true,
                "b" => white_turn = false,
                _ => panic!("Invalid FEN string"),
            },
            None => panic!("Invalid FEN string"),
        }

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

        match fen_en_passant {
            Some(fen_en_passant) => {
                if *fen_en_passant != "-" {
                    let col = fen_en_passant.chars().nth(0).unwrap() as u8 - 97;
                    let row = fen_en_passant.chars().nth(1).unwrap() as u8 - 49;
                    en_passant = row * 8 + col;
                }
            }
            None => panic!("Invalid FEN string"),
        }

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

        let white_pieces: u64 =
            white_pawns | white_knights | white_bishops | white_rooks | white_queens;
        let black_pieces: u64 =
            black_pawns | black_knights | black_bishops | black_rooks | black_queens;
        let empty_squares: u64 = !(white_pieces | black_pieces);

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

    pub fn export_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;

        for row in 0..8 {
            for col in 0..8 {
                let piece = self.get_square(row, col);
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

        if self.en_passant > 0 {
            let row: u8 = (self.en_passant / 7) as u8;
            let col: u8 = (self.en_passant % 7) as u8;

            fen.push((col + 97) as char);
            fen.push((row + 49) as char);
        } else {
            fen.push('-');
        }

        fen.push(' ');

        fen.push_str(&self.halfmove.to_string());

        fen.push(' ');

        fen.push_str(&self.fullmove.to_string());

        fen
    }

    pub fn get_square(&self, row: u8, col: u8) -> Option<char> {
        if self.white_pawns & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('P');
        }
        if self.white_knights & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('N');
        }
        if self.white_bishops & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('B');
        }
        if self.white_rooks & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('R');
        }
        if self.white_queens & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('Q');
        }
        if self.white_king & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('K');
        }
        if self.black_pawns & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('p');
        }
        if self.black_knights & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('n');
        }
        if self.black_bishops & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('b');
        }
        if self.black_rooks & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('r');
        }
        if self.black_queens & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('q');
        }
        if self.black_king & (1u64 << (row as u32 * 8 + col as u32)) != 0 {
            return Some('k');
        }

        return None;
    }

    pub fn board_display(&self) -> String {
        let mut board = String::new();
        for row in (0..8).rev() {
            for col in 0..8 {
                let piece = self.get_square(row, col);
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

        board
    }

    pub fn possible_wp(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Pawn NE captures
        let mut pawn_moves = (self.white_pawns << 9) & !FILE_A & self.black_pieces & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move { from: i - 9, to: i });
            }
        }

        // Pawn NW captures
        pawn_moves = (self.white_pawns << 7) & !FILE_H & self.black_pieces & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move { from: i - 7, to: i });
            }
        }

        // Pawn forward one
        pawn_moves = (self.white_pawns << 8)
            & !(self.white_pieces | self.black_pieces | self.white_king | self.black_king)
            & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move { from: i - 8, to: i });
            }
        }

        // Pawn forward two
        pawn_moves = (pawn_moves << 8)
            & ((self.white_pawns & RANK_2) << 16
                & !(self.white_pieces | self.black_pieces | self.white_king | self.black_king)
                & !RANK_1
                & !RANK_2);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move {
                    from: i - 16,
                    to: i,
                });
            }
        }

        // Pawn NE en passant
        pawn_moves = (self.white_pawns << 9) & !FILE_A & !RANK_1 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(Move {
                from: self.en_passant - 9,
                to: self.en_passant,
            });
        }

        // Pawn NW en passant
        pawn_moves = (self.white_pawns << 7) & !FILE_H & !RANK_1 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(Move {
                from: self.en_passant - 7,
                to: self.en_passant,
            });
        }

        moves
    }

    pub fn possible_bp(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // Pawn SW captures
        let mut pawn_moves = (self.black_pawns >> 9) & !FILE_H & self.white_pieces & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move { from: i + 9, to: i });
            }
        }

        // Pawn SE captures
        pawn_moves = (self.black_pawns >> 7) & !FILE_A & self.white_pieces & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move {
                    from: i + 7,
                    to: i,
                });
            }
        }

        // Pawn forward one
        pawn_moves = (self.black_pawns >> 8)
            & !(self.white_pieces | self.black_pieces | self.white_king | self.black_king)
            & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move { from: i + 8, to: i });
            }
        }

        // Pawn forward two
        pawn_moves = (pawn_moves >> 8)
            & ((self.black_pawns & RANK_7) >> 16
                & !(self.white_pieces | self.black_pieces | self.white_king | self.black_king)
                & !RANK_8
                & !RANK_7);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Move {
                    from: i + 16,
                    to: i,
                });
            }
        }

        // Pawn SW en passant
        pawn_moves = (self.black_pawns >> 9) & !FILE_H & !RANK_8 & (1u64 << self.en_passant);
        
        if pawn_moves != 0 && self.white_turn {
            moves.push(Move {
                from: self.en_passant + 9,
                to: self.en_passant,
            });
        }

        // Pawn SE en passant
        pawn_moves = (self.black_pawns >> 7) & !FILE_A & !RANK_8 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(Move {
                from: self.en_passant + 7,
                to: self.en_passant,
            });
        }

        moves
    }
}
