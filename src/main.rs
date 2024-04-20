mod chess;

fn main() {
    let board = chess::create_board();
    println!("{}", board);
}
