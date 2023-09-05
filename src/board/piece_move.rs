use super::Square;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveFlag {
    Quiet,
    DoublePawn,
    Capture,
    EnPassant{capture: Square},
    Promotion{piece: PieceType, capture: bool},
    Castle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    from: Square,
    to: Square,
    flag: MoveFlag,
}

impl Move {
    #[inline(always)]
    pub fn make_quiet(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::Quiet,
        }
    }

    #[inline(always)]
    pub fn make_double_pawn(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::DoublePawn,
        }
    }

    #[inline(always)]
    pub fn make_capture(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::Capture,
        }
    }

    #[inline(always)]
    pub fn make_en_passant(from: Square, to: Square, capture: Square) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::EnPassant{capture},
        }
    }

    #[inline(always)]
    pub fn make_promotion(from: Square, to: Square, piece: PieceType, capture: bool) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::Promotion{piece, capture},
        }
    }

    #[inline(always)]
    pub fn make_castle(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            flag: MoveFlag::Castle,
        }
    }

    #[inline(always)]
    pub fn make(from: Square, to: Square, flag: MoveFlag) -> Move {
        Move { from, to, flag }
    }
}