#[derive(Clone, Debug, PartialEq)]
pub struct ChessBoard {
    // Bitboards for each piece type
    pub white_pawns: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub bitboards: [[u64; u64; u64; u64; u64; u64], [u64; u64; u64; u64; u64; u64]],

    pub black_pawns: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

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
        let white_pawns = 0x000000000000FF00;
        let white_knights = 0x0000000000000042;
        let white_bishops = 0x0000000000000024;
        let white_rooks = 0x0000000000000081;
        let white_queens = 0x0000000000000008;
        let white_king = 0x0000000000000010;

        let black_pawns = 0x00FF000000000000;
        let black_knights = 0x4200000000000000;
        let black_bishops = 0x2400000000000000;
        let black_rooks = 0x8100000000000000;
        let black_queens = 0x0800000000000000;
        let black_king = 0x1000000000000000;

        let white_pieces =
            white_pawns | white_knights | white_bishops | white_rooks | white_queens | white_king;
        let black_pieces =
            black_pawns | black_knights | black_bishops | black_rooks | black_queens | black_king;
        let all_pieces = white_pieces | black_pieces;

        ChessBoard {
            white_pawns,
            white_knights,
            white_bishops,
            white_rooks,
            white_queens,
            white_king,
            black_pawns,
            black_knights,
            black_bishops,
            black_rooks,
            black_queens,
            black_king,
            white_can_castle_queenside: false,
            white_can_castle_kingside: false,
            black_can_castle_queenside: false,
            black_can_castle_kingside: false,
            has_white_castled: false,
            has_black_castled: false,
            all_pieces,
            white_pieces,
            black_pieces,
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


impl ChessBoard {
    pub fn move_piece(&mut self, piece_type: u8, piece_color: bool) {
        // Going to change the bitboards to be in an array
        // Ignore this function for the time being
    }
}