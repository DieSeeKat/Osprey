pub fn draw_bit_board(bitboard: u64) {
    let mut bitboard = bitboard;
    let output = String::new();
    for i in 0..8 {
        for j in 0..8 {
            if bitboard & 1u64 << ((7 - i) * 8 + j) != 0 {
                print!("x ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}