#![crate_name = "osprey"]

use crate::board::Board;
use crate::perft::perft;
use clap::{Parser, Subcommand};
use std::thread::available_parallelism;

mod board;
mod perft;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Perft {
        #[clap(short, long)]
        load_file: Option<String>,
        #[clap(short, long)]
        fen_string: Option<String>,
        #[clap(short, long)]
        depth: u8,
        #[clap(short, long)]
        num_threads: Option<u8>,
        #[clap(short, long)]
        auto_threads: bool,
        #[clap(short, long)]
        benchmark: bool,
    },
}

fn main() {
    let args = App::parse();

    match args.command {
        Command::Perft {
            load_file,
            fen_string,
            depth,
            num_threads,
            auto_threads,
            benchmark,
        } => {
            let num_threads = match num_threads {
                Some(num_threads) => num_threads,
                None => {
                    let threads = if auto_threads {
                        match available_parallelism() {
                            Ok(num_threads) => num_threads.get() as u8 - 1,
                            Err(_) => 1,
                        }
                    } else {
                        1
                    };

                    println!("Using {} threads", threads);

                    threads
                }
            };

            // Construct the Board

            let mut fen: Option<String>;
            fen = match load_file {
                Some(file) => {
                    let file = std::fs::read_to_string(file);

                    match file {
                        Ok(file) => Some(file),
                        Err(_) => panic!("File not found"),
                    }
                }
                None => None,
            };

            fen = match fen_string {
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

            // Run the perft
            let start = std::time::Instant::now();
            let nodes = perft(&start_board, 0, depth, num_threads);
            let duration = start.elapsed();

            println!("====Perft Results===");
            println!("-----------------");
            println!("Nodes: {}", nodes);
            println!("-----------------");

            if benchmark {
                println!("Time: {:?}", duration);
                println!(
                    "Nodes per second: {}",
                    nodes as f64 / duration.as_secs_f64()
                );
                println!(
                    "Nodes per second per thread: {}",
                    nodes as f64 / duration.as_secs_f64() / num_threads as f64
                );
                println!("-----------------");
            }
        }
    }
}
