mod chess;

fn main() {
    let board = chess::create_board();
    chess::print_board(&board);
}
