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

/*
Gonna recreate this:
public void LoadFenString()
        {
            // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
            string fenString = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            board = new();


            byte piecePosition = 0;
            fenString = fenString.Replace("/", "");

            // Find location of spaces
            List<int> spaceIndices = new List<int>();

            for (int j = 0; j < fenString.Length; j++)
            {
                if (fenString[j] == ' ')
                {
                    spaceIndices.Add(j);
                }
            }

            int[] spaceIndicesArray = spaceIndices.ToArray();

            /*for (int k = 0; k < spaceIndicesArray.Length; k++) {
                Debug.Log(spaceIndicesArray[k]);
            }*/

            board.ClearBoard();
            int i;

            // Piece positions
            for (i = 0; i < fenString.Length; i++)
            {
                if (fenString[i] == ' ')
                {
                    break;
                }

                // Check for black pieces
                if (fenString[i] == 'r')
                {
                    HelperFunctions.SetBit(ref board.BlackRooks, piecePosition, 1);
                }
                else if (fenString[i] == 'n')
                {
                    HelperFunctions.SetBit(ref board.BlackKnights, piecePosition, 1);
                }
                else if (fenString[i] == 'b')
                {
                    HelperFunctions.SetBit(ref board.BlackBishops, piecePosition, 1);
                }
                else if (fenString[i] == 'k')
                {
                    HelperFunctions.SetBit(ref board.BlackKing, piecePosition, 1);
                }
                else if (fenString[i] == 'q')
                {
                    HelperFunctions.SetBit(ref board.BlackQueens, piecePosition, 1);
                }
                else if (fenString[i] == 'p')
                {
                    HelperFunctions.SetBit(ref board.BlackPawns, piecePosition, 1);
                }

                // Check for white pieces
                if (fenString[i] == 'R')
                {
                    HelperFunctions.SetBit(ref board.WhiteRooks, piecePosition, 1);
                }
                else if (fenString[i] == 'N')
                {
                    HelperFunctions.SetBit(ref board.WhiteKnights, piecePosition, 1);
                }
                else if (fenString[i] == 'B')
                {
                    HelperFunctions.SetBit(ref board.WhiteBishops, piecePosition, 1);
                }
                else if (fenString[i] == 'K')
                {
                    HelperFunctions.SetBit(ref board.WhiteKing, piecePosition, 1);
                }
                else if (fenString[i] == 'Q')
                {
                    HelperFunctions.SetBit(ref board.WhiteQueens, piecePosition, 1);
                }
                else if (fenString[i] == 'P')
                {
                    HelperFunctions.SetBit(ref board.WhitePawns, piecePosition, 1);
                }

                // Make numbers in fenStrings work
                // NOTE: I'm not adding cases for the number 1 because it is already done by the for loop
                if (fenString[i] == '2')
                {
                    piecePosition++;
                }
                else if (fenString[i] == '3')
                {
                    piecePosition += 2;
                }
                else if (fenString[i] == '4')
                {
                    piecePosition += 3;
                }
                else if (fenString[i] == '5')
                {
                    piecePosition += 4;
                }
                else if (fenString[i] == '6')
                {
                    piecePosition += 5;
                }
                else if (fenString[i] == '7')
                {
                    piecePosition += 6;
                }
                else if (fenString[i] == '8')
                {
                    piecePosition += 7;
                }

                piecePosition++;
            }

            // Side to move
            if (fenString[spaceIndicesArray[0] + 1] == 'w')
            {
                board.WhiteToMove = true;
            }
            else
            {
                board.WhiteToMove = false;
            }

            // Castling
            FenStringCastling(fenString, spaceIndicesArray[1]);

            //finish setting up the board
            board.WhiteBishops = HelperFunctions.FlipBitboard(board.WhiteBishops);
            board.WhiteKing = HelperFunctions.FlipBitboard(board.WhiteKing);
            board.WhiteQueens = HelperFunctions.FlipBitboard(board.WhiteQueens);
            board.WhiteRooks = HelperFunctions.FlipBitboard(board.WhiteRooks);
            board.WhiteKnights = HelperFunctions.FlipBitboard(board.WhiteKnights);
            board.WhitePawns = HelperFunctions.FlipBitboard(board.WhitePawns);

            board.BlackBishops = HelperFunctions.FlipBitboard(board.BlackBishops);
            board.BlackKing = HelperFunctions.FlipBitboard(board.BlackKing);
            board.BlackQueens = HelperFunctions.FlipBitboard(board.BlackQueens);
            board.BlackRooks = HelperFunctions.FlipBitboard(board.BlackRooks);
            board.BlackKnights = HelperFunctions.FlipBitboard(board.BlackKnights);
            board.BlackPawns = HelperFunctions.FlipBitboard(board.BlackPawns);

            for (int j = 0; j < 64; j++)
            {
                if (HelperFunctions.GetBit(j, board.BlackKing) == 1) board.BlackKingPos = (byte)j;
                if (HelperFunctions.GetBit(j, board.WhiteKing) == 1) board.WhiteKingPos = (byte)j;
            }

            board.UpdateBitBoards();
        }

        private void FenStringCastling(string fenString, int fenStringPos)
        {
            Debug.Log($"Fenstring: {fenString}, Fenstringposition: {fenStringPos}");
            // First position
            if (fenString[fenStringPos + 1] == '-')
            {
                board.WhiteCanCastleKingside = false;
                board.WhiteCanCastleQueenside = false;
                board.BlackCanCastleKingside = false;
                board.BlackCanCastleQueenside = false;

                return;
            }
            else if (fenString[fenStringPos + 1] == 'K')
            {
                board.WhiteCanCastleKingside = true;
            }
            else if (fenString[fenStringPos + 1] == 'Q')
            {
                board.WhiteCanCastleQueenside = true;
            }
            else if (fenString[fenStringPos + 1] == 'k')
            {
                board.BlackCanCastleKingside = true;
            }
            else if (fenString[fenStringPos + 1] == 'q')
            {
                board.BlackCanCastleQueenside = true;

                return;
            }

            if (fenString[fenStringPos + 2] == ' ')
            {
                return;
            }
            else if (fenString[fenStringPos + 2] == 'Q')
            {
                board.WhiteCanCastleQueenside = true;
            }
            else if (fenString[fenStringPos + 2] == 'k')
            {
                board.BlackCanCastleKingside = true;
            }
            else if (fenString[fenStringPos + 2] == 'q')
            {
                board.BlackCanCastleQueenside = true;

                return;
            }

            if (fenString[fenStringPos + 3] == ' ')
            {
                return;
            }
            else if (fenString[fenStringPos + 3] == 'k')
            {
                board.BlackCanCastleKingside = true;
            }
            else if (fenString[fenStringPos + 3] == 'q')
            {
                board.BlackCanCastleQueenside = true;

                return;
            }

            if (fenString[fenStringPos + 4] == ' ')
            {
                return;
            }
            else if (fenString[fenStringPos + 4] == 'q')
            {
                board.BlackCanCastleQueenside = true;

                return;
            }


        }
*/
impl ChessBoard {
    pub fn fen_string_reader(&mut self, fen_string: String) {
        println!("fen_string_reader does not do anything useful yet.");
        println!("{fen_string}");
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
