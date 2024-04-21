use colored::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone)]
enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

#[derive(Copy, Clone)]
pub struct Square {
    piece: Option<Piece>,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.piece {
            Some(piece) => {
                let piece_char = match piece.piece_type {
                    PieceType::King => 'K',
                    PieceType::Queen => 'Q',
                    PieceType::Rook => 'R',
                    PieceType::Bishop => 'B',
                    PieceType::Knight => 'N',
                    PieceType::Pawn => 'P',
                };
                let piece_char = match piece.color {
                    PieceColor::White => piece_char.to_string().color("Yellow"),
                    PieceColor::Black => piece_char.to_ascii_lowercase().to_string().color("Blue"),
                };
                write!(f, "{}", piece_char)
            }
            None => write!(f, " "),
        }
    }
}

pub struct Board {
    squares: [[Square; 8]; 8],
    location_to_index: HashMap<String, (usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        let mut location_to_index = HashMap::new();
        for (i, row) in ('a'..='h').enumerate() {
            for (j, col) in (1..=8).enumerate() {
                location_to_index.insert(format!("{}{}", row, col), (i, j));
            }
        }

        Self {
            squares: create_squares(),
            location_to_index,
        }
    }

    pub fn get_index(&self, location: &str) -> Option<(usize, usize)> {
        self.location_to_index.get(location).copied()
    }
}

pub fn create_squares() -> [[Square; 8]; 8] {
    let mut squares = [[Square { piece: None }; 8]; 8];

    for i in 0..8 {
        squares[1][i].piece = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::Black,
        });

        squares[6][i].piece = Some(Piece {
            piece_type: PieceType::Pawn,
            color: PieceColor::White,
        });
    }

    let back_row = [
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Queen,
        PieceType::King,
        PieceType::Bishop,
        PieceType::Knight,
        PieceType::Rook,
    ];

    for i in 0..8 {
        squares[0][i].piece = Some(Piece {
            piece_type: back_row[i],
            color: PieceColor::Black,
        });
        squares[7][i].piece = Some(Piece {
            piece_type: back_row[i],
            color: PieceColor::White,
        });
    }

    squares
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.squares.iter().enumerate() {
            write!(f, "{} | ", 8 - i)?;
            for square in row {
                write!(f, "{} ", square)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "   {}", "-".repeat(16))?;
        write!(f, "   ")?;
        for i in 0..8 {
            write!(f, " {}", (b'A' + i) as char)?;
        }

        writeln!(f)
    }
}
