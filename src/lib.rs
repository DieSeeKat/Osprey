// Make the function available at the root of the crate
pub use tools::draw_bit_board;
pub use board::{Board, Move};
pub use engine::perft::Perft;

// Make module public
pub mod tools; 
pub mod board;
pub mod engine;
