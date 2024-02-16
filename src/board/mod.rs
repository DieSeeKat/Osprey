pub use self::models::{ Board, Move };
pub use self::services::MoveService;

pub mod models;
pub mod services;

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

///
/// The type of a piece.
///
#[allow(dead_code)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}
