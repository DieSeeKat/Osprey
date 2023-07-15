// Make module public
pub mod utils; 
pub mod board;
pub mod perft;

// Make the function available at the root of the crate
pub use utils::*;
pub use board::*;
pub use perft::*;