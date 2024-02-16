use crate::board::{Board, Move, Piece};

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

const NOT_RANK_1_8: u64 = !(RANK_1 | RANK_8);
const NOT_RANK_1_2: u64 = !(RANK_1 | RANK_2);
const NOT_RANK_7_8: u64 = !(RANK_7 | RANK_8);

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

pub struct MoveService {}

impl MoveService {
    
    ///
    /// Get all pseudo-legal moves (without worrying about check) white can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal moves white can make.
    ///
    #[inline]
    pub fn possible_white(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.append(&mut MoveService::possible_wp(board));
        moves.append(&mut MoveService::possible_wk(board));
        moves.append(&mut MoveService::possible_wq(board));
        moves.append(&mut MoveService::possible_wr(board));
        moves.append(&mut MoveService::possible_wb(board));
        moves.append(&mut MoveService::possible_wn(board));
        moves.append(&mut MoveService::possible_wc(board));

        moves
    }

    ///
    /// Get all pseudo-legal moves (without worrying about check) black can make.
    ///
    /// # Returns
    ///
    /// A vector of all pseudo-legal moves black can make.
    ///
    #[inline]
    pub fn possible_black(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        moves.append(&mut MoveService::possible_bp(board));
        moves.append(&mut MoveService::possible_bk(board));
        moves.append(&mut MoveService::possible_bq(board));
        moves.append(&mut MoveService::possible_br(board));
        moves.append(&mut MoveService::possible_bb(board));
        moves.append(&mut MoveService::possible_bn(board));
        moves.append(&mut MoveService::possible_bc(board));

        moves
    }

    ///
    /// Calculates the positions possibly moved to by a horizontal or vertical slider.
    ///
    /// # Arguments
    ///
    /// * `board` - The board.
    /// * `position` - The position of the slider as a number between 0 and 63 (both included).
    ///
    /// # Returns
    ///
    /// A bitboard representing the possible positions.
    ///
    #[inline]
    const fn possible_hv(board: &Board, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !board.empty_squares;

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
    /// * `board` - The board.
    /// * `position` - The position of the slider as a number between 0 and 63 (both included).
    ///
    /// # Returns
    ///
    /// A bitboard representing the possible positions.
    ///
    #[inline]
    const fn possible_da(board: &Board, position: u8) -> u64 {
        let slider = 1u64 << position;
        let occupied = !board.empty_squares;

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
    /// Get all pseudo-legal pawn moves white can make.
    ///
    /// # Arguments
    /// 
    /// * `board` - The board.
    /// 
    /// # Returns
    ///
    /// A vector of all pseudo-legal pawn moves white can make.
    ///
    #[inline]
    fn possible_wp(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pawn_ne_captures = (board.white_pawns << 9) & !FILE_A & board.black_pieces & NOT_RANK_1_8;
        let pawn_nw_captures = (board.white_pawns << 7) & !FILE_H & board.black_pieces & NOT_RANK_1_8;
        let pawn_forward_one = (board.white_pawns << 8) & board.empty_squares & NOT_RANK_1_8;
        let pawn_forward_two = (pawn_forward_one << 8)
            & ((board.white_pawns & RANK_2) << 16 & board.empty_squares & NOT_RANK_1_2);
        let pawn_promotion = (board.white_pawns << 8) & board.empty_squares & RANK_8;
        let pawn_promotion_ne_captures =
            (board.white_pawns << 9) & !FILE_A & board.black_pieces & RANK_8;
        let pawn_promotion_nw_captures =
            (board.white_pawns << 7) & !FILE_H & board.black_pieces & RANK_8;

        for i in 0..64 {
            let this = 1u64 << i;
            if pawn_ne_captures & this != 0 {
                moves.push(Move::Normal { from: i - 9, to: i });
            }
            if pawn_nw_captures & this != 0 {
                moves.push(Move::Normal { from: i - 7, to: i });
            }
            if pawn_forward_one & this != 0 {
                moves.push(Move::Normal { from: i - 8, to: i });
            }
            if pawn_forward_two & this != 0 {
                moves.push(Move::Normal {
                    from: i - 16,
                    to: i,
                });
            }
            if pawn_promotion & this != 0 {
                for promotion in [
                    Piece::WhiteBishop,
                    Piece::WhiteKnight,
                    Piece::WhiteRook,
                    Piece::WhiteQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i - 8,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
            if pawn_promotion_ne_captures & this != 0 {
                for promotion in [
                    Piece::WhiteBishop,
                    Piece::WhiteKnight,
                    Piece::WhiteRook,
                    Piece::WhiteQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i - 9,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
            if pawn_promotion_nw_captures & this != 0 {
                for promotion in [
                    Piece::WhiteBishop,
                    Piece::WhiteKnight,
                    Piece::WhiteRook,
                    Piece::WhiteQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i - 7,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        match board.en_passant {
            Some(en_passant) => {
                // Pawn NE en passant

                let mut pawn_moves =
                    (board.white_pawns << 9) & !FILE_A & !RANK_1 & (1u64 << en_passant);

                if pawn_moves != 0 && board.white_turn {
                    moves.push(Move::EnPassant {
                        from: en_passant - 9,
                        to: en_passant,
                        captured: en_passant - 8,
                    });
                }

                // Pawn NW en passant

                pawn_moves = (board.white_pawns << 7) & !FILE_H & !RANK_1 & (1u64 << en_passant);

                if pawn_moves != 0 && board.white_turn {
                    moves.push(Move::EnPassant {
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
    #[inline]
    fn possible_wn(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.white_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.white_pieces & !board.white_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.white_pieces & !board.white_king;
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
    #[inline]
    fn possible_wb(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.white_bishops & (1u64 << i) != 0 {
                let bishop_moves = MoveService::possible_da(board, i) & !board.white_pieces & !board.white_king;

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
    #[inline]
    fn possible_wr(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.white_rooks & (1u64 << i) != 0 {
                let rook_moves = MoveService::possible_hv(board, i) & !board.white_pieces & !board.white_king;

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
    #[inline]
    fn possible_wq(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.white_queens & (1u64 << i) != 0 {
                let queen_moves = (MoveService::possible_hv(board, i) | MoveService::possible_da(board, i))
                    & !board.white_pieces
                    & !board.white_king;

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
    #[inline]
    fn possible_wk(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.white_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.white_pieces;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.white_pieces;
                }

                let safe_white = !MoveService::unsafe_w(board);

                for j in 0..64 {
                    if possibility & 1u64 << j & safe_white != 0 {
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
    #[inline]
    fn possible_bp(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pawn_se_captures = (board.black_pawns >> 7) & !FILE_A & board.white_pieces & NOT_RANK_1_8;
        let pawn_sw_captures = (board.black_pawns >> 9) & !FILE_H & board.white_pieces & NOT_RANK_1_8;
        let pawn_forward_one = (board.black_pawns >> 8) & board.empty_squares & NOT_RANK_1_8;
        let pawn_forward_two = (pawn_forward_one >> 8)
            & ((board.black_pawns & RANK_7) >> 16 & board.empty_squares & NOT_RANK_7_8);
        let pawn_promotion = (board.black_pawns >> 8) & board.empty_squares & RANK_1;
        let pawn_promotion_se_captures =
            (board.black_pawns >> 7) & !FILE_A & board.white_pieces & RANK_1;
        let pawn_promotion_sw_captures =
            (board.black_pawns >> 9) & !FILE_H & board.white_pieces & RANK_1;

        for i in 0..64 {
            let this = 1u64 << i;
            if pawn_se_captures & this != 0 {
                moves.push(Move::Normal { from: i + 7, to: i });
            }
            if pawn_sw_captures & this != 0 {
                moves.push(Move::Normal { from: i + 9, to: i });
            }
            if pawn_forward_one & this != 0 {
                moves.push(Move::Normal { from: i + 8, to: i });
            }
            if pawn_forward_two & this != 0 {
                moves.push(Move::Normal {
                    from: i + 16,
                    to: i,
                });
            }
            if pawn_promotion & this != 0 {
                for promotion in [
                    Piece::BlackBishop,
                    Piece::BlackKnight,
                    Piece::BlackRook,
                    Piece::BlackQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i + 8,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
            if pawn_promotion_se_captures & this != 0 {
                for promotion in [
                    Piece::BlackBishop,
                    Piece::BlackKnight,
                    Piece::BlackRook,
                    Piece::BlackQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i + 7,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
            if pawn_promotion_sw_captures & this != 0 {
                for promotion in [
                    Piece::BlackBishop,
                    Piece::BlackKnight,
                    Piece::BlackRook,
                    Piece::BlackQueen,
                ]
                .iter()
                {
                    moves.push(Move::Promotion {
                        from: i + 9,
                        to: i,
                        promotion: *promotion,
                    });
                }
            }
        }

        match board.en_passant {
            Some(en_passant) => {
                // Pawn SW en passant

                let mut pawn_moves =
                    (board.black_pawns >> 9) & !FILE_H & !RANK_8 & (1u64 << en_passant);

                if pawn_moves != 0 && !board.white_turn {
                    moves.push(Move::EnPassant {
                        from: en_passant + 9,
                        to: en_passant,
                        captured: en_passant + 8,
                    });
                }

                // Pawn SE en passant

                pawn_moves = (board.black_pawns >> 7) & !FILE_A & !RANK_8 & (1u64 << en_passant);

                if pawn_moves != 0 && !board.white_turn {
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
    #[inline]
    fn possible_bn(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.black_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.black_pieces & !board.black_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.black_pieces & !board.black_king;
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
    #[inline]
    fn possible_bb(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.black_bishops & (1u64 << i) != 0 {
                let bishop_moves = MoveService::possible_da(board, i) & !board.black_pieces & !board.black_king;

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
    #[inline]
    fn possible_br(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.black_rooks & (1u64 << i) != 0 {
                let rook_moves = MoveService::possible_hv(board, i) & !board.black_pieces & !board.black_king;

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
    #[inline]
    fn possible_bq(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.black_queens & (1u64 << i) != 0 {
                let queen_moves = (MoveService::possible_hv(board, i) | MoveService::possible_da(board, i))
                    & !board.black_pieces
                    & !board.black_king;

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
    #[inline]
    fn possible_bk(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for i in 0..64 {
            if board.black_king & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 9 {
                    possibility = KING_SPAN << (i - 9);
                } else {
                    possibility = KING_SPAN >> (9 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.black_pieces;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.black_pieces;
                }

                let safe_black = !MoveService::unsafe_b(board);

                for j in 0..64 {
                    if possibility & 1u64 << j & safe_black != 0 {
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
    #[inline]
    fn possible_wc(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // King side castle
        if board.white_castle_kingside {
            let unsafe_w = MoveService::unsafe_w(board);

            // Positions
            // 4 : King's position. Should not be in check.
            // 5 : Position between king and rook. Should be empty and safe.
            // 6 : Position between king and rook. Should be empty and safe.
            // 7 : Rook's position. Should contain a rook.
            if unsafe_w & (1u64 << 4) == 0
                && (unsafe_w | !board.empty_squares) & 1u64 << 5 == 0
                && (unsafe_w | !board.empty_squares) & 1u64 << 6 == 0
                && board.white_rooks & 1u64 << 7 != 0
            {
                moves.push(Move::Castle {
                    from: 4,
                    to: 6,
                    rook: 7,
                });
            }
        }

        // Queen side castle
        if board.white_castle_queenside {
            let unsafe_w = MoveService::unsafe_w(board);

            // Positions
            // 4 : King's position. Should not be in check.
            // 3 : Position between king and rook. Should be empty and safe.
            // 2 : Position between king and rook. Should be empty and safe.
            // 1 : Position between king and rook. Should be empty.
            // 0 : Rook's position. Should contain a rook.
            if !board.empty_squares & (1u64 << 1) == 0
                && (unsafe_w | !board.empty_squares) & (1u64 << 2) == 0
                && (unsafe_w | !board.empty_squares) & (1u64 << 3) == 0
                && unsafe_w & (1u64 << 4) == 0
            {
                moves.push(Move::Castle {
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
    #[inline]
    fn possible_bc(board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // King side castle
        if board.black_castle_kingside {
            let unsafe_b = MoveService::unsafe_b(board);

            // Positions
            // 60 : King's position. Should not be in check.
            // 61 : Position between king and rook. Should be empty and safe.
            // 62 : Position between king and rook. Should be empty and safe.
            // 63 : Rook's position. Should contain a rook.
            if unsafe_b & (1u64 << 60) == 0
                && (unsafe_b | !board.empty_squares) & (1u64 << 61) == 0
                && (unsafe_b | !board.empty_squares) & (1u64 << 62) == 0
                && board.black_rooks & (1u64 << 63) != 0
            {
                moves.push(Move::Castle {
                    from: 60,
                    to: 62,
                    rook: 63,
                });
            }
        }

        // Queen side castle
        if board.black_castle_queenside {
            let unsafe_b = MoveService::unsafe_b(board);

            // Positions
            // 60 : King's position. Should not be in check.
            // 59 : Position between king and rook. Should be empty and safe.
            // 58 : Position between king and rook. Should be empty and safe.
            // 57 : Position between king and rook. Should be empty.
            // 56 : Rook's position. Should contain a rook.
            if board.black_rooks & (1u64 << 56) != 0
                && !board.empty_squares & (1u64 << 57) == 0
                && (unsafe_b | !board.empty_squares) & (1u64 << 58) == 0
                && (unsafe_b | !board.empty_squares) & (1u64 << 59) == 0
                && unsafe_b & (1u64 << 60) == 0
            {
                moves.push(Move::Castle {
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
    #[inline]
    pub fn unsafe_w(board: &Board) -> u64 {
        let mut unsafe_squares: u64 = 0;

        // pawn
        unsafe_squares |= (board.black_pawns >> 7) & !FILE_A;
        unsafe_squares |= (board.black_pawns >> 9) & !FILE_H;

        // knight

        for i in 0..64 {
            if board.black_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.black_pieces & !board.black_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.black_pieces & !board.black_king;
                };

                unsafe_squares |= possibility;
            }
        }

        // bishop | queen

        let bishop_queen = board.black_bishops | board.black_queens;

        for i in 0..64 {
            if bishop_queen & 1u64 << i != 0 {
                unsafe_squares |= MoveService::possible_da(board, i);
            }
        }

        // rook | queen

        let rook_queen = board.black_rooks | board.black_queens;

        for i in 0..64 {
            if rook_queen & 1u64 << i != 0 {
                unsafe_squares |= MoveService::possible_hv(board, i);
            }
        }

        // king

        for i in 0..64 {
            if board.black_king & 1u64 << i != 0 {
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
    #[inline]
    pub fn unsafe_b(board: &Board) -> u64 {
        let mut unsafe_squares: u64 = 0;

        // pawn
        unsafe_squares |= (board.white_pawns << 7) & !FILE_H;
        unsafe_squares |= (board.white_pawns << 9) & !FILE_A;

        // knight

        for i in 0..64 {
            if board.white_knights & 1u64 << i != 0 {
                let mut possibility: u64;

                if i > 18 {
                    possibility = KNIGHT_SPAN << (i - 18);
                } else {
                    possibility = KNIGHT_SPAN >> (18 - i);
                }

                if i % 8 < 4 {
                    possibility &= !(FILE_G | FILE_H) & !board.white_pieces & !board.white_king;
                } else {
                    possibility &= !(FILE_A | FILE_B) & !board.white_pieces & !board.white_king;
                };

                unsafe_squares |= possibility;
            }
        }

        // bishop | queen

        let bishop_queen = board.white_bishops | board.white_queens;

        for i in 0..64 {
            if bishop_queen & 1u64 << i != 0 {
                unsafe_squares |= MoveService::possible_da(board, i);
            }
        }

        // rook | queen

        let rook_queen = board.white_rooks | board.white_queens;

        for i in 0..64 {
            if rook_queen & 1u64 << i != 0 {
                unsafe_squares |= MoveService::possible_hv(board, i);
            }
        }

        // king

        for i in 0..64 {
            if board.white_king & 1u64 << i != 0 {
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

#[test]
fn pawn_capture_nw() {
    let board = Board::new("8/8/8/p5pp/P6P/8/8/8 w - - 0 1");
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 31, to: 38 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_ne() {
    let board = Board::new("8/8/8/pp5p/P6P/8/8/8 w - - 0 1");
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 24, to: 33 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_n() {
    let board = Board::new("8/8/2p5/4p3/2P1P3/8/8/8 w - - 0 1");
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 26, to: 34 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_nn() {
    let board = Board::new("8/8/6p1/2p1p3/p7/4P1P1/P1P5/8 w - - 0 1");
    let moves = MoveService::possible_wp(&board);
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
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: Piece::WhiteRook,
        },
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: Piece::WhiteBishop,
        },
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: Piece::WhiteKnight,
        },
        Move::Promotion {
            from: 51,
            to: 59,
            promotion: Piece::WhiteQueen,
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
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: Piece::WhiteRook,
        },
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: Piece::WhiteBishop,
        },
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: Piece::WhiteKnight,
        },
        Move::Promotion {
            from: 51,
            to: 60,
            promotion: Piece::WhiteQueen,
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
    let moves = MoveService::possible_wp(&board);
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
    let moves = MoveService::possible_wp(&board);
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
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 39, to: 47 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn en_passant_border_nw() {
    let board = Board::new("8/8/8/P6p/8/8/8/8 w - h6 0 1");
    let moves = MoveService::possible_wp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 32, to: 40 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_sw() {
    let board = Board::new("8/8/8/p6p/P5PP/8/8/8 w - - 0 1");
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 39, to: 30 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_capture_se() {
    let board = Board::new("8/8/8/p6p/PP5P/8/8/8 w - - 0 1");
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 32, to: 25 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_s() {
    let board = Board::new("8/8/8/2p1p3/4P3/2P5/8/8 w - - 0 1");
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 34, to: 26 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn pawn_move_ss() {
    let board = Board::new("8/p1p5/4p1p1/P7/2P1P3/6P1/8/8 w - - 0 1");
    let moves = MoveService::possible_bp(&board);
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
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: Piece::BlackRook,
        },
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: Piece::BlackBishop,
        },
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: Piece::BlackKnight,
        },
        Move::Promotion {
            from: 11,
            to: 3,
            promotion: Piece::BlackQueen,
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
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: Piece::BlackRook,
        },
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: Piece::BlackBishop,
        },
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: Piece::BlackKnight,
        },
        Move::Promotion {
            from: 11,
            to: 4,
            promotion: Piece::BlackQueen,
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
    let moves = MoveService::possible_bp(&board);
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
    let moves = MoveService::possible_bp(&board);
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
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 31, to: 23 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn en_passant_border_sw() {
    let board = Board::new("8/8/8/8/p6P/8/8/8 b - a6 0 1");
    let moves = MoveService::possible_bp(&board);
    let correct_moves: Vec<Move> = vec![Move::Normal { from: 24, to: 16 }];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_rook_move_border() {
    let board = Board::new("8/8/8/8/3R4/8/8/8 w - - 0 1");
    let moves = MoveService::possible_wr(&board);
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
    let moves = MoveService::possible_wr(&board);
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
    let moves = MoveService::possible_wr(&board);
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
    let moves = MoveService::possible_br(&board);
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
    let moves = MoveService::possible_br(&board);
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
    let moves = MoveService::possible_br(&board);
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
    let moves = MoveService::possible_wb(&board);

    assert_eq!(moves.len(), 7);
}

#[test]
fn w_bishop_move_border() {
    let board = Board::new("8/8/8/8/3B4/8/8/8 w - - 0 1");
    let moves = MoveService::possible_wb(&board);
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
    let moves = MoveService::possible_wb(&board);
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
    let moves = MoveService::possible_wb(&board);
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
    let moves = MoveService::possible_bb(&board);
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
    let moves = MoveService::possible_bb(&board);
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
    let moves = MoveService::possible_bb(&board);
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
    let moves = MoveService::possible_wq(&board);
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
    let moves = MoveService::possible_wq(&board);
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
    let moves = MoveService::possible_wq(&board);
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
    let moves = MoveService::possible_bq(&board);
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
    let moves = MoveService::possible_bq(&board);
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
    let moves = MoveService::possible_bq(&board);
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
    let moves = MoveService::possible_wn(&board);
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
    let moves = MoveService::possible_wn(&board);
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
    let moves = MoveService::possible_wn(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn w_knight_border() {
    let board = Board::new("N6N/8/8/8/8/8/8/N6N w - - 0 1");
    let moves = MoveService::possible_wn(&board);
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
    let moves = MoveService::possible_bn(&board);
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
    let moves = MoveService::possible_bn(&board);
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
    let moves = MoveService::possible_bn(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn b_knight_border() {
    let board = Board::new("n6n/8/8/8/8/8/8/n6n w - - 0 1");
    let moves = MoveService::possible_bn(&board);
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
    let moves = MoveService::possible_wk(&board);
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
    let moves = MoveService::possible_wk(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn b_king_move_capture() {
    let board = Board::new("8/8/8/2P1P3/3k4/2P1P3/8/8 w - - 0 1");
    let moves = MoveService::possible_bk(&board);
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
    let moves = MoveService::possible_bk(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
    for m in moves {
        assert!(correct_moves.contains(&m));
    }
}

#[test]
fn castling_w_kq() {
    let board = Board::new("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let moves = MoveService::possible_wc(&board);
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
    let moves = MoveService::possible_wc(&board);
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
    let moves = MoveService::possible_wc(&board);
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

    let moves = MoveService::possible_wc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_w_checked() {
    let board = Board::new("4r3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    let moves = MoveService::possible_wc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_w_blocked() {
    let board = Board::new("8/8/8/8/8/8/8/R2PKP1R w KQ - 0 1");
    let moves = MoveService::possible_wc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_kq() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/8 w kq - 0 1");
    let moves = MoveService::possible_bc(&board);
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
    let moves = MoveService::possible_bc(&board);
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
    let moves = MoveService::possible_bc(&board);
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
    let moves = MoveService::possible_bc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_checked() {
    let board = Board::new("r3k2r/8/8/8/8/8/8/4R3 w q - 0 1");
    let moves = MoveService::possible_bc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn castling_b_blocked() {
    let board = Board::new("r2pkp1r/8/8/8/8/8/8/8 w q - 0 1");
    let moves = MoveService::possible_bc(&board);
    let correct_moves: Vec<Move> = vec![];
    assert_eq!(moves.len(), correct_moves.len());
}

#[test]
fn w_unsafe_squares() {
    let board = Board::new("8/4r3/3n2b1/3p2n1/4K3/8/6q1/8 w - - 0 1");
    let unsafe_squares: u64 = MoveService::unsafe_w(&board);
    let correct_unsafe_squares: u64 = 1508443033184550880;
    assert_eq!(unsafe_squares, correct_unsafe_squares);
}

#[test]
fn b_unsafe_squares() {
    let board = Board::new("8/4R3/3N2B1/6N1/4k3/3P4/6Q1/8 w - - 0 1");
    let unsafe_squares: u64 = MoveService::unsafe_b(&board);
    let correct_unsafe_squares: u64 = 1508443033184550880;
    assert_eq!(unsafe_squares, correct_unsafe_squares);
}
