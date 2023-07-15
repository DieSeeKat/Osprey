use crate::board::Board;

pub fn perft(board: &Board, depth: u8, max_depth: u8) -> u64 {
    if depth == max_depth {
        return 1;
    }

    let mut nodes = 0;

    let moves = if board.white_turn {
        board.possible_white()
    } else {
        board.possible_black()
    };

    for m in moves {
        match board.make_move(&m) {
            Ok(new_board) => {
                let num_nodes = perft(&new_board, depth + 1, max_depth);
                nodes += num_nodes;
            }
            Err(_) => {}
        }
    }

    nodes
}