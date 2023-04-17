#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub enum Piece {
    Pawn(bool, Square),
    Knight(bool, Square),
    Bishop(bool, Square),
    Rook(bool, Square),
    Queen(bool, Square),
    King(bool, Square),
}

impl Piece {
    pub fn new(piece_type: char, white: bool, x: u8, y: u8) -> Piece {
        let piece_type = piece_type.to_uppercase().next().unwrap();
        match piece_type {
            'K' => Piece::King(white, Square{x, y}),
            'Q' => Piece::Queen(white, Square{x, y}),
            'R' => Piece::Rook(white, Square{x, y}),
            'B' => Piece::Bishop(white, Square{x, y}),
            'N' => Piece::Knight(white, Square{x, y}),
            'P' => Piece::Pawn(white, Square{x, y}),
            _ => panic!("Invalid piece type"),
        }
    }

    pub fn new_fen(piece_type: char, x: u8, y: u8) -> Piece {
        let piece_type = piece_type.to_uppercase().next().unwrap();
        match piece_type {
            'K' => Piece::King(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            'Q' => Piece::Queen(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            'R' => Piece::Rook(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            'B' => Piece::Bishop(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            'N' => Piece::Knight(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            'P' => Piece::Pawn(piece_type == piece_type.to_uppercase().next().unwrap(), Square{x, y}),
            _ => panic!("Invalid piece type"),
        }
    }

    pub fn new_san(san: &str) -> Piece {
        let piece_type = san.chars().nth(0).unwrap();
        let x = san.chars().nth(1).unwrap();
        let col = {
            match x {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                'f' => 6,
                'g' => 7,
                'h' => 8,
                _ => panic!("Invalid column"),
            }
        };
        let row = san.chars().nth(2).unwrap() as u8 - 49;

        Self::new(piece_type, piece_type == piece_type.to_uppercase().next().unwrap(), col, row)
    }

    fn get_fen(&self) -> String {
        match self {
            Piece::Pawn(white, Square{x, y}) => {
                if *white {
                    format!("P{}{}", x, y)
                } else {
                    format!("p{}{}", x, y)
                }
            },
            Piece::Knight(white, Square{x, y}) => {
                if *white {
                    format!("N{}{}", x, y)
                } else {
                    format!("n{}{}", x, y)
                }
            },
            Piece::Bishop(white, Square{x, y}) => {
                if *white {
                    format!("B{}{}", x, y)
                } else {
                    format!("b{}{}", x, y)
                }
            },
            Piece::Rook(white, Square{x, y}) => {
                if *white {
                    format!("R{}{}", x, y)
                } else {
                    format!("r{}{}", x, y)
                }
            },
            Piece::Queen(white, Square{x, y}) => {
                if *white {
                    format!("Q{}{}", x, y)
                } else {
                    format!("q{}{}", x, y)
                }
            },
            Piece::King(white, Square{x, y}) => {
                if *white {
                    format!("K{}{}", x, y)
                } else {
                    format!("k{}{}", x, y)
                }
            },
        }
    }

    fn get_square(&self) -> Square {
        match self {
            Piece::Pawn(_, square)
            | Piece::Knight(_, square)
            | Piece::Bishop(_, square)
            | Piece::Rook(_, square)
            | Piece::Queen(_, square)
            | Piece::King(_, square) => *square,
        }
    }

    fn set_square(&mut self, square: Square) {
        match self {
            Piece::Pawn(_, ref mut s)
            | Piece::Knight(_, ref mut s)
            | Piece::Bishop(_, ref mut s)
            | Piece::Rook(_, ref mut s)
            | Piece::Queen(_, ref mut s)
            | Piece::King(_, ref mut s) => *s = square,
        }
    }

}

impl std::fmt::Display for Piece{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::King(white, Square{x, y}) => write!(f, "{} King at {}, {}", if *white { "White" } else { "Black" }, x, y),
            Piece::Queen(white, Square{x, y}) => write!(f, "{} Queen at {}, {}", if *white { "White" } else { "Black" }, x, y),
            Piece::Rook(white, Square{x, y}) => write!(f, "{} Rook at {}, {}", if *white { "White" } else { "Black" }, x, y),
            Piece::Bishop(white, Square{x, y}) => write!(f, "{} Bishop at {}, {}", if *white { "White" } else { "Black" }, x, y),
            Piece::Knight(white, Square{x, y}) => write!(f, "{} Knight at {}, {}", if *white { "White" } else { "Black" }, x, y),
            Piece::Pawn(white, Square{x, y}) => write!(f, "{} Pawn at {}, {}", if *white { "White" } else { "Black" }, x, y),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pieces: Vec<Piece>,
    white_turn: bool,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    en_passant: Vec<Square>,
    halfmove: u16,
    fullmove: u16,
}

impl Board {
    pub fn starting_pos() -> Board {
        Board {
            pieces: vec![
                Piece::new('R', false, 0, 0),
                Piece::new('N', false, 1, 0),
                Piece::new('B', false, 2, 0),
                Piece::new('Q', false, 3, 0),
                Piece::new('K', false, 4, 0),
                Piece::new('B', false, 5, 0),
                Piece::new('N', false, 6, 0),
                Piece::new('R', false, 7, 0),
                Piece::new('P', false, 0, 1),
                Piece::new('P', false, 1, 1),
                Piece::new('P', false, 2, 1),
                Piece::new('P', false, 3, 1),
                Piece::new('P', false, 4, 1),
                Piece::new('P', false, 5, 1),
                Piece::new('P', false, 6, 1),
                Piece::new('P', false, 7, 1),
                Piece::new('P', true, 0, 6),
                Piece::new('P', true, 1, 6),
                Piece::new('P', true, 2, 6),
                Piece::new('P', true, 3, 6),
                Piece::new('P', true, 4, 6),
                Piece::new('P', true, 5, 6),
                Piece::new('P', true, 6, 6),
                Piece::new('P', true, 7, 6),
                Piece::new('R', true, 0, 7),
                Piece::new('N', true, 1, 7),
                Piece::new('B', true, 2, 7),
                Piece::new('Q', true, 3, 7),
                Piece::new('K', true, 4, 7),
                Piece::new('B', true, 5, 7),
                Piece::new('N', true, 6, 7),
                Piece::new('R', true, 7, 7),
            ],
            white_turn: true,
            white_castle_kingside: true,
            white_castle_queenside: true,
            black_castle_kingside: true,
            black_castle_queenside: true,
            en_passant: vec![],
            halfmove: 0,
            fullmove: 0,
        }
    }

    pub fn new_fen(input: &str) -> Board {

        let mut row = 8;
        let mut col = 1;

        let fen: Vec<&str> = input.split_whitespace().collect();

        let fen_pieces = fen.get(0);
        let fen_turn = fen.get(1);
        let fen_castling = fen.get(2);
        let fen_en_passant = fen.get(3);
        let fen_half_move = fen.get(4);
        let fen_full_move = fen.get(5);

        let mut pieces = vec![];
        let white_turn;
        let mut white_castle_kingside = false;
        let mut white_castle_queenside = false;
        let mut black_castle_kingside = false;
        let mut black_castle_queenside = false;
        let mut en_passant: Vec<Square> = vec![];
        let halfmove;
        let fullmove;

        match fen_pieces {
            Some(fen_pieces) => {
                for c in fen_pieces.chars() {
                    if (c as u8) >= 48 && (c as u8) <= 57 {
                        col += c as u8 - 48;
                    } else if c == '/' {
                        row -= 1;
                        col = 1;
                    } else {
                        pieces.push(Piece::new_fen(c, col, row));
                        col += 1;
                    }
                }
            },
            None => panic!("Invalid FEN string"),
        }

        match fen_turn {
            Some(fen_turn) => {
                match *fen_turn {
                    "w" => white_turn = true,
                    "b" => white_turn = false,
                    _ => panic!("Invalid FEN string"),
                }
            },
            None => panic!("Invalid FEN string"),
        }

        match fen_castling {
            Some(fen_castling) => {
                for c in fen_castling.chars() {
                    match c {
                        'K' => white_castle_kingside = true,
                        'Q' => white_castle_queenside = true,
                        'k' => black_castle_kingside = true,
                        'q' => black_castle_queenside = true,
                        '-' => (),
                        _ => panic!("Invalid FEN string"),
                    }
                }
            },
            None => panic!("Invalid FEN string"),
        }

        match fen_en_passant {
            Some(fen_en_passant) => {
                if *fen_en_passant != "-" {
                    let col = fen_en_passant.chars().nth(0).unwrap() as u8 - 97;
                    let row = fen_en_passant.chars().nth(1).unwrap() as u8 - 49;
                    en_passant.push(Square{x: col, y: row});
                }
            },
            None => panic!("Invalid FEN string"),
        }

        match fen_half_move {
            Some(fen_half_move) => {
                let half_move = fen_half_move.parse::<u16>();
                match half_move {
                    Ok(half_move) => halfmove = half_move,
                    Err(_) => panic!("Invalid FEN string"),
                }
            },
            None => panic!("Invalid FEN string"),
        }

        match fen_full_move {
            Some(fen_full_move) => {
                let full_move = fen_full_move.parse::<u16>();
                match full_move {
                    Ok(full_move) => fullmove = full_move,
                    Err(_) => panic!("Invalid FEN string"),
                }
            },
            None => panic!("Invalid FEN string"),
        }
        
        Board { pieces: pieces, white_turn, white_castle_kingside, white_castle_queenside, black_castle_kingside, black_castle_queenside, en_passant, halfmove, fullmove }
    }

    pub fn export_fen(&self) -> String {
        let mut fen = String::new();
        let mut empty = 0;

        for row in (1..9).rev() {
            for col in 1..9 {
                let square = Square{x: col, y: row};
                let piece = self.get_square(square);
                match piece {
                    Some(piece) => {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        fen.push(piece.get_fen().chars().next().unwrap());
                    },
                    None => {
                        empty += 1;
                    },
                }
            }
            if empty > 0 {
                fen.push_str(&empty.to_string());
                empty = 0;
            }
            if row > 1 {
                fen.push('/');
            }
        }

        fen.push(' ');

        if self.white_turn {
            fen.push('w');
        } else {
            fen.push('b');
        }

        fen.push(' ');

        if self.white_castle_kingside {
            fen.push('K');
        }
        if self.white_castle_queenside {
            fen.push('Q');
        }
        if self.black_castle_kingside {
            fen.push('k');
        }
        if self.black_castle_queenside {
            fen.push('q');
        }
        if !self.white_castle_kingside && !self.white_castle_queenside && !self.black_castle_kingside && !self.black_castle_queenside {
            fen.push('-');
        }

        fen.push(' ');

        if self.en_passant.len() > 0 {
            fen.push((self.en_passant[0].x + 97) as char);
            fen.push((self.en_passant[0].y + 49) as char);
        } else {
            fen.push('-');
        }

        fen.push(' ');

        fen.push_str(&self.halfmove.to_string());

        fen.push(' ');

        fen.push_str(&self.fullmove.to_string());

        fen
    }

    pub fn get_square(&self, square: Square) -> Option<&Piece>{
        for piece in &self.pieces {
            if piece.get_square() == square {
                return Some(piece);
            }
        }
        return None;
    }


}