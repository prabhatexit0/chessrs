use colored::*;
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
struct Square {
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
}

pub fn create_board() -> Board {
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

    Board { squares }
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
