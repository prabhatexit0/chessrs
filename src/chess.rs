use colored::*;

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

pub fn print_board(board: &Board) {
    board.squares.iter().enumerate().for_each(|(i, row)| {
        print!("{} | ", i + 1);
        for square in row {
            match &square.piece {
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
                        PieceColor::Black => {
                            piece_char.to_ascii_lowercase().to_string().color("Blue")
                        }
                    };
                    print!("{} ", piece_char);
                }
                None => print!(" "),
            }
        }
        println!();
    });

    println!("   {}", "-".repeat(16));
    print!("   ");
    for i in 0..8 {
        print!(" {}", (b'A' + i) as char);
    }

    println!();
}
