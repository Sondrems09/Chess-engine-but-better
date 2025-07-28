pub mod board;

fn main() {
    println!("Hello, world!");

    let mut board = board::ChessBoard::new();

    board.fen_string_reader();

    print!("{} 0x{:x} 0x{:x}", board.black_king_pos, board.bitboards[board.pawns as usize][board.white as usize], board.bitboards[board.pawns as usize][board.black as usize]);
}
