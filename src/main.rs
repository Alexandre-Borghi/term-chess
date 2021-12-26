use core::fmt::Display;

const BOARD_SIZE: usize = 64;

#[derive(Clone, Copy)]
enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    Blank,
}

impl Default for Piece {
    fn default() -> Self {
        Self::Blank
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_str = match self {
            Piece::WhitePawn => "♙",
            Piece::WhiteRook => "♖",
            Piece::WhiteKnight => "♘",
            Piece::WhiteBishop => "♗",
            Piece::WhiteQueen => "♕",
            Piece::WhiteKing => "♔",
            Piece::BlackPawn => "♟",
            Piece::BlackRook => "♜",
            Piece::BlackKnight => "♞",
            Piece::BlackBishop => "♝",
            Piece::BlackQueen => "♛",
            Piece::BlackKing => "♚",
            Piece::Blank => " ",
        };

        f.write_str(piece_str)?;

        Ok(())
    }
}

fn main() {
    let board = board_new();
    println!("{}", board_to_str(board));
}

fn board_new() -> [Piece; BOARD_SIZE] {
    // 00 01 02 03 04 05 06 07
    // 08 09 10 11 12 13 14 15
    // 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31
    // 32 33 34 35 36 37 38 39
    // 40 41 42 43 44 45 46 47
    // 48 49 50 51 52 53 54 55
    // 56 57 58 59 60 61 62 63

    let mut board: [Piece; BOARD_SIZE] = [Piece::Blank; BOARD_SIZE];
    board[0] = Piece::BlackRook;
    board[1] = Piece::BlackKnight;
    board[2] = Piece::BlackBishop;
    board[3] = Piece::BlackQueen;
    board[4] = Piece::BlackKing;
    board[5] = Piece::BlackBishop;
    board[6] = Piece::BlackKnight;
    board[7] = Piece::BlackRook;
    for i in 8..=15 {
        board[i] = Piece::BlackPawn;
    }
    for i in 48..=55 {
        board[i] = Piece::WhitePawn;
    }
    board[56 + 0] = Piece::WhiteRook;
    board[56 + 1] = Piece::WhiteKnight;
    board[56 + 2] = Piece::WhiteBishop;
    board[56 + 3] = Piece::WhiteQueen;
    board[56 + 4] = Piece::WhiteKing;
    board[56 + 5] = Piece::WhiteBishop;
    board[56 + 6] = Piece::WhiteKnight;
    board[56 + 7] = Piece::WhiteRook;
    board
}

fn board_to_str(board: [Piece; BOARD_SIZE]) -> String {
    let mut repr = String::new();
    for i in 0..BOARD_SIZE {
        if i != 0 && i % 8 == 0 {
            repr.push('\n');
        }

        repr.push_str(format!("{}", board[i]).as_str());
    }
    repr
}
