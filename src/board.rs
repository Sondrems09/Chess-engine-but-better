#[derive(Clone, Debug, PartialEq)]
pub struct ChessBoard {
    // Bitboards for each piece type
    pub bitboards: [[u64; 6]; 2],

    pub pawns: u8,
    pub knights: u8,
    pub bishops: u8,
    pub rooks: u8,
    pub queens: u8,
    pub king: u8,

    pub white: u8,
    pub black: u8,

    pub white_can_castle_queenside: bool,
    pub white_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,

    pub has_white_castled: bool,
    pub has_black_castled: bool,

    pub all_pieces: u64,
    pub white_pieces: u64,
    pub black_pieces: u64,

    pub en_passant_target_square: u8, // 255 if none

    pub white_king_pos: u8,
    pub black_king_pos: u8,

    pub white_attack_board: u64,
    pub black_attack_board: u64,

    pub white_to_move: bool,

    pub fifty_move_rule: f32,
    pub move_count: f32,
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            pawns: 0,
            knights: 1,
            bishops: 2,
            rooks: 3,
            queens: 4,
            king: 5,
            white: 0,
            black: 1,
            bitboards: [[0; 6]; 2],
            all_pieces: 0,
            black_pieces: 0,
            white_pieces: 0,
            white_can_castle_queenside: false,
            white_can_castle_kingside: false,
            black_can_castle_queenside: false,
            black_can_castle_kingside: false,
            has_white_castled: false,
            has_black_castled: false,
            en_passant_target_square: u8::MAX,
            white_king_pos: 4,
            black_king_pos: 60,
            white_attack_board: 0,
            black_attack_board: 0,
            white_to_move: true,
            fifty_move_rule: 0.0,
            move_count: 0.0,
        }
    }
}

pub fn set_bit(bitboard: u64, bit_position: usize, value: bool) -> u64 {
    if bit_position >= 64 {
        panic!("Bit position must be between 0 and 63.");
    }
    let mask = 1u64 << bit_position;
    if value {
        bitboard | mask
    } else {
        bitboard & !mask
    }
}

impl ChessBoard {
    pub fn fen_string_reader(&mut self) {
        println!("fen_string_reader does not do anything useful yet.");
        self.bitboards[self.pawns as usize][self.black as usize] = 0xffffffffffffffff;
    }
}

/*
impl ChessBoard {
    pub fn update_bitboards(&mut self) {
        self.white_pieces = self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_king;
        self.black_pieces = self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_king;
        self.all_pieces = self.white_pieces | self.black_pieces;
    }
}
*/
/*
impl ChessBoard {
    pub fn move_piece(&mut self, piece_type: u8, piece_color: bool) {
        // Going to change the bitboards to be in an array
        // Ignore this function for the time being
    }
}*/