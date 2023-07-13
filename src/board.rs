use std::fmt;

pub const FILE_A: u64 = 72340172838076673;
pub const FILE_B: u64 = 144680345676153346;
pub const FILE_C: u64 = 289360691352306692;
pub const FILE_D: u64 = 578721382704613384;
pub const FILE_E: u64 = 1157442765409226768;
pub const FILE_F: u64 = 2314885530818453536;
pub const FILE_G: u64 = 4629771061636907072;
pub const FILE_H: u64 = 9259542123273814144;
pub const RANK_1: u64 = 255;
pub const RANK_2: u64 = 65280;
pub const RANK_3: u64 = 16711680;
pub const RANK_4: u64 = 4278190080;
pub const RANK_5: u64 = 1095216660480;
pub const RANK_6: u64 = 280375465082880;
pub const RANK_7: u64 = 71776119061217280;
pub const RANK_8: u64 = 18374686479671623680;
pub const CENTER: u64 = 103481868288;
pub const EXTENDED_CENTER: u64 = 66229406269440;
pub const KING_SIDE: u64 = 9295429630892703744;
pub const QUEEN_SIDE: u64 = 4755801206503243840;
pub const WHITE_SQUARES: u64 = 2863311530;
pub const BLACK_SQUARES: u64 = 1431655765;
pub const KNIGHT_SPAN: u64 = 43234889994;
pub const KING_SPAN: u64 = 460039;

pub const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
pub const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];
pub const DIAGONALS: [u64; 15] = [
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

pub const ANTI_DIAGONALS: [u64; 15] = [
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

#[derive(Debug, PartialEq)]
pub enum Move {
    Normal { from: u8, to: u8 },
    Castle { from: u8, to: u8, rook: u8 },
    EnPassant { from: u8, to: u8, captured: u8 },
    Promotion { from: u8, to: u8, promotion: char },
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
    pub empty_squares: u64,
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

    pub fn square(&self, position: u8) -> Option<char> {
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

    pub fn possible_hv(&self, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !self.empty_squares;

        let horizontal = (occupied.wrapping_sub(2u64.wrapping_mul(slider)))
            ^ (occupied
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits())))
            .reverse_bits();
        let vertical = ((occupied & FILES[position as usize % 8]).wrapping_sub(2u64.wrapping_mul(slider)))
            ^ ((occupied & FILES[position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2u64.wrapping_mul(slider.reverse_bits())))
            .reverse_bits();

        (horizontal & RANKS[(position / 8) as usize]) | (vertical & FILES[(position % 8) as usize])
    }

    pub fn possible_da(&self, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !self.empty_squares;

        let diagonal = (occupied & DIAGONALS[position as usize / 8 + position as usize % 8])
            .wrapping_sub(2 * slider)
            ^ (occupied & DIAGONALS[position as usize / 8 + position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2 * slider.reverse_bits())
                .reverse_bits();
        let anti_diagonal = (occupied
            & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
            .wrapping_sub(2 * slider)
            ^ (occupied & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
                .reverse_bits()
                .wrapping_sub(2 * slider.reverse_bits())
                .reverse_bits();

        (diagonal & DIAGONALS[position as usize / 8 + position as usize % 8])
            | (anti_diagonal & ANTI_DIAGONALS[position as usize / 8 + 7 - position as usize % 8])
    }

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

    pub fn possible_wp(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        // Pawn NE captures

        let mut pawn_moves =
            (self.white_pawns << 9) & !FILE_A & self.black_pieces & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i - 9, to: i });
            }
        }

        // Pawn NW captures

        pawn_moves = (self.white_pawns << 7) & !FILE_H & self.black_pieces & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i - 7, to: i });
            }
        }

        // Pawn forward one

        pawn_moves = (self.white_pawns << 8) & self.empty_squares & !RANK_1 & !RANK_8;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i - 8, to: i });
            }
        }

        // Pawn forward two

        pawn_moves = (pawn_moves << 8)
            & ((self.white_pawns & RANK_2) << 16 & self.empty_squares & !RANK_1 & !RANK_2);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal {
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

        // Pawn NE en passant

        pawn_moves = (self.white_pawns << 9) & !FILE_A & !RANK_1 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(EnPassant {
                from: self.en_passant - 9,
                to: self.en_passant,
                captured: self.en_passant - 8,
            });
        }

        // Pawn NW en passant

        pawn_moves = (self.white_pawns << 7) & !FILE_H & !RANK_1 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(EnPassant {
                from: self.en_passant - 7,
                to: self.en_passant,
                captured: self.en_passant - 8,
            });
        }

        moves
    }

    pub fn possible_wn(&self) -> Vec<Move> {
        use Move::*;

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
                        moves.push(Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    pub fn possible_wb(&self) -> Vec<Move> {
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

    pub fn possible_wr(&self) -> Vec<Move> {
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

    pub fn possible_wq(&self) -> Vec<Move> {    
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

    pub fn possible_wk(&self) -> Vec<Move> {
        use Move::*;

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
                        moves.push(Normal { from: i, to: j });
                    }
                }

                break;
            }
        }

        moves
    }

    pub fn possible_bp(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        // Pawn SW captures

        let mut pawn_moves =
            (self.black_pawns >> 9) & !FILE_H & self.white_pieces & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i + 9, to: i });
            }
        }

        // Pawn SE captures

        pawn_moves = (self.black_pawns >> 7) & !FILE_A & self.white_pieces & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i + 7, to: i });
            }
        }

        // Pawn forward one

        pawn_moves = (self.black_pawns >> 8) & self.empty_squares & !RANK_8 & !RANK_1;

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal { from: i + 8, to: i });
            }
        }

        // Pawn forward two

        pawn_moves = (pawn_moves >> 8)
            & ((self.black_pawns & RANK_7) >> 16 & self.empty_squares & !RANK_8 & !RANK_7);

        for i in 0..64 {
            if pawn_moves & (1u64 << i) != 0 {
                moves.push(Normal {
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
                    moves.push(Promotion {
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
                    moves.push(Promotion {
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
                    moves.push(Promotion {
                        from: i + 7,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        // Pawn SW en passant

        pawn_moves = (self.black_pawns >> 9) & !FILE_H & !RANK_8 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(EnPassant {
                from: self.en_passant + 9,
                to: self.en_passant,
                captured: self.en_passant + 8,
            });
        }

        // Pawn SE en passant

        pawn_moves = (self.black_pawns >> 7) & !FILE_A & !RANK_8 & (1u64 << self.en_passant);

        if pawn_moves != 0 && self.white_turn {
            moves.push(EnPassant {
                from: self.en_passant + 7,
                to: self.en_passant,
                captured: self.en_passant + 8,
            });
        }

        moves
    }

    pub fn possible_bn(&self) -> Vec<Move> {
        use Move::*;

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
                        moves.push(Normal { from: i, to: j });
                    }
                }
            }
        }

        moves
    }

    pub fn possible_bb(&self) -> Vec<Move> {
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

    pub fn possible_br(&self) -> Vec<Move> {
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

    pub fn possible_bq(&self) -> Vec<Move> {
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

    pub fn possible_bk(&self) -> Vec<Move> {
        use Move::*;

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
                        moves.push(Normal { from: i, to: j });
                    }
                }

                break;
            }
        }

        moves
    }

    pub fn possible_wc(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        if self.white_castle_kingside {
            if  self.unsafe_w() & (1u64 >> 4) == 0
                && self.unsafe_w() & (1u64 >> 5) == 0
                && self.unsafe_w() & (1u64 >> 6) == 0
            {
                moves.push(Castle { from: 4, to: 6, rook: 7 });
            }
        }

        if self.white_castle_queenside {
            if  self.unsafe_w() & (1u64 >> 2) == 0
                && self.unsafe_w() & (1u64 >> 3) == 0
                && self.unsafe_w() & (1u64 >> 4) == 0
            {
                moves.push(Castle { from: 4, to: 2, rook: 0 });
            }
        }

        moves
    }

    pub fn possible_bc(&self) -> Vec<Move> {
        use Move::*;

        let mut moves: Vec<Move> = Vec::new();

        if self.black_castle_kingside {
            if  self.unsafe_b() & (1u64 >> 60) == 0
                && self.unsafe_b() & (1u64 >> 61) == 0
                && self.unsafe_b() & (1u64 >> 62) == 0
            {
                moves.push(Castle { from: 60, to: 62, rook: 63 });
            }
        }

        if self.black_castle_queenside {
            if  self.unsafe_b() & (1u64 >> 58) == 0
                && self.unsafe_b() & (1u64 >> 59) == 0
                && self.unsafe_b() & (1u64 >> 60) == 0
            {
                moves.push(Castle { from: 60, to: 58, rook: 56 });
            }
        }

        moves
    }

    pub fn unsafe_w(&self) -> u64 {
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

    pub fn unsafe_b(&self) -> u64 {
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