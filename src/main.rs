pub mod board;

fn main() {
    println!("Hello, world!");

    let board = board::ChessBoard::new();

    print!("{} 0x{:x} 0x{:x}", board.black_king_pos, board.white_pawns, board.black_pawns);
}
