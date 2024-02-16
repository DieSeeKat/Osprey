use crate::board::Piece;

/// A move of a piece on the board.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
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
    Promotion { from: u8, to: u8, promotion: Piece },
}