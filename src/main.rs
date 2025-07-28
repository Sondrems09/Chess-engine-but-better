pub mod board;

fn main() {
    println!("Hello, world!");

    let mut board = board::ChessBoard::new();

    board.fen_string_reader("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    print!("{} 0x{:x} 0x{:x}", board.black_king_pos, board.bitboards[board.pawns as usize][board.white as usize], board.bitboards[board.pawns as usize][board.black as usize]);
}
