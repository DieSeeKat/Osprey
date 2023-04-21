// Make module public
pub mod utils; 
pub mod board;

// Make the function available at the root of the crate
pub use utils::*;
pub use board::*;