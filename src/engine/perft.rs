use crate::board::{Board, Move};
use std::thread;

pub struct Perft {}

impl Perft {
    pub fn perft(board: &Board, depth: u8, max_depth: u8, num_threads: u8) -> u64 {
        if depth == max_depth {
            return 1;
        }

        let mut nodes = 0;

        let moves = if board.white_turn {
            board.possible_white()
        } else {
            board.possible_black()
        };

        let chunks = moves.chunks(moves.len() / num_threads as usize);
        let mut handles: Vec<_> = Vec::new();

        for chunk in chunks {
            let my_board = board.clone();
            let my_chunk: Vec<Move> = chunk.iter().map(|m| (*m).clone()).collect();

            let handle = thread::spawn(move || {
                let mut nodes = 0;
                for m in my_chunk {
                    match my_board.make_move(&m) {
                        Ok(new_board) => {
                            let num_nodes = Perft::perft_branch(&new_board, depth + 1, max_depth);
                            nodes += num_nodes;
                        }
                        Err(_) => {}
                    }
                }

                return nodes;
            });

            handles.push(handle);
        }

        for handle in handles {
            nodes += handle.join().unwrap();
        }

        nodes
    }

    fn perft_branch(board: &Board, depth: u8, max_depth: u8) -> u64 {
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
                    let num_nodes = Perft::perft_branch(&new_board, depth + 1, max_depth);
                    nodes += num_nodes;
                }
                Err(_) => {}
            }
        }

        nodes
    }
}
