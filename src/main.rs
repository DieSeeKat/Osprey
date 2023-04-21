use crate::board::Board;
use clap::Parser;

mod board;

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
        Some(file) => {
            let file = std::fs::read_to_string(file);

            match file {
                Ok(file) => Some(file),
                Err(_) => panic!("File not found"),
            }
        }
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

    let start_board = Board::new(&fen);
    println!("====FEN String===");
    println!("-----------------");
    println!("{}", start_board.export_fen());
    println!("-----------------");
    println!("==Board Display==");
    println!("-----------------");
    println!("{}", start_board);
    println!("-----------------");
}
