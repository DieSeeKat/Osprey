<div align="center">

  # Osprey
  
  ![Static Badge](https://img.shields.io/badge/Lichess_Rating-0-blue)
  ![Static Badge](https://img.shields.io/badge/Version-0.0.4_Development-orange)
  ![GitHub Release Date - Published_At](https://img.shields.io/github/release-date/DieSeeKat/Osprey)
  
  ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/DieSeeKat/Osprey/rust.yml)
  ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/DieSeeKat/Osprey)
  ![GitHub language count](https://img.shields.io/github/languages/count/DieSeeKat/Osprey)

</div>

The Osprey chess bot is a free and open source chess engine written in Rust.

Osprey does not include a Graphical User Interface (GUI) and currently only lives in the console.

## Files

This repository contains the following files and folders:
- [README.md](https://github.com/DieSeeKat/Osprey/blob/master/README.md) - This file.
- [src](https://github.com/DieSeeKat/Osprey/tree/master/src) - The source folder containing the Osprey engine.
  - [board.rs](https://github.com/DieSeeKat/Osprey/blob/master/src/board.rs) - The struct and implementation of the chess board, containing all logic for move generation.
  - [engine.rs](https://github.com/DieSeeKat/Osprey/blob/master/src/engine.rs) - The Osprey engine.
  - [utils.rs](https://github.com/DieSeeKat/Osprey/blob/master/src/utils.rs) - Small utils file containing functionality useful in testing and development.
- [tests](https://github.com/DieSeeKat/Osprey/tree/master/tests) - The folder containing all tests.

## Insallation and Setup

The Osprey engine compiles using Rust, therefore some [Rust compiler](https://www.rust-lang.org/tools/install) is necessary for compilation.

After a Rust compiler is installed, compilation of the engine is easy:
- Clone the repository ```git clone https://github.com/DieSeeKat/Osprey```
- Move into the root directory ```cd Osprey```
- You can either run the project directly or build and use the bin file
  - Build the project ```cargo build --release```
    Now you can find the binaries in the ```target/release/osprey.bin```
    Add the file to your PATH variable and run ```osprey //inline arguments///```
  - Run the project ```cargo run -- //inline arguments//```
 
## Future Plans

1. Our first mission is to get Osprey finished and building the entire chess engine. Osprey is currently in development and a working engine is on the way.
2. Integrate Osprey with the Lichess Bot api.
3. Add a GUI.
4. General optimisations.

## Contributing

Osprey is an in-development engine, being developed by a small team (me). Any contribution bt pull request is welcome on the dev branch.
