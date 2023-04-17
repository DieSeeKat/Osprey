use crate::utils::{Board, Piece};
use clap::{Parser, Subcommand};

mod utils;

#[derive(Parser)]
struct App {
    #[clap(short, long)]
    load_file: Option<String>,
    #[clap(short, long)]
    fen_string: Option<String>,
}

fn main() {
    let args = App::parse();

    // Construct the Board

    let mut fen: Option<String>;
    fen = match args.load_file {
        Some(file) => Some(std::fs::read_to_string(file).unwrap()),
        None => None,
    };

    fen = match args.fen_string {
        Some(fen_string) => Some(fen_string),
        None => fen,
    };

    let fen = match fen {
        Some(fen) => fen,
        None => panic!("No FEN string provided"),
    };

    let start_board = Board::new_fen(&fen);
    println!("{:?}", start_board);
    println!("{}", start_board.export_fen());
}
