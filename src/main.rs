pub mod board;
pub mod constants;

fn main() {
    use constants::*;

    println!("Hello, world!");

    let mut board = board::ChessBoard::new();

    board.fen_string_reader("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    print!("{} 0x{:x} 0x{:x}", board.black_king_pos, board.bitboards[constants::PAWNS as usize][WHITE as usize], board.bitboards[PAWNS as usize][BLACK as usize]);
}
