use core::fmt::Display;

const BOARD_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq)]
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

struct Board {
    pieces: [Piece; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        // 00 01 02 03 04 05 06 07
        // 08 09 10 11 12 13 14 15
        // 16 17 18 19 20 21 22 23
        // 24 25 26 27 28 29 30 31
        // 32 33 34 35 36 37 38 39
        // 40 41 42 43 44 45 46 47
        // 48 49 50 51 52 53 54 55
        // 56 57 58 59 60 61 62 63

        let mut pieces: [Piece; BOARD_SIZE] = [Piece::Blank; BOARD_SIZE];
        pieces[0] = Piece::BlackRook;
        pieces[1] = Piece::BlackKnight;
        pieces[2] = Piece::BlackBishop;
        pieces[3] = Piece::BlackQueen;
        pieces[4] = Piece::BlackKing;
        pieces[5] = Piece::BlackBishop;
        pieces[6] = Piece::BlackKnight;
        pieces[7] = Piece::BlackRook;
        for i in 8..=15 {
            pieces[i] = Piece::BlackPawn;
        }
        for i in 48..=55 {
            pieces[i] = Piece::WhitePawn;
        }
        pieces[56 + 0] = Piece::WhiteRook;
        pieces[56 + 1] = Piece::WhiteKnight;
        pieces[56 + 2] = Piece::WhiteBishop;
        pieces[56 + 3] = Piece::WhiteQueen;
        pieces[56 + 4] = Piece::WhiteKing;
        pieces[56 + 5] = Piece::WhiteBishop;
        pieces[56 + 6] = Piece::WhiteKnight;
        pieces[56 + 7] = Piece::WhiteRook;

        Self { pieces }
    }

    fn move_src_to_dst(&mut self, src: &str, dst: &str) {}

    fn pos_to_index(pos: &str) -> usize {
        0usize
    }

    fn move_i_to_j(&mut self, i: usize, j: usize) {
        self.pieces[j] = std::mem::take(&mut self.pieces[i]);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::new();
        repr.push_str("    a   b   c   d   e   f   g   h\n");
        for i in 0..8 {
            repr.push_str("  +---+---+---+---+---+---+---+---+\n");
            repr.push_str(format!("{} ", 8 - i).as_str());
            for j in 0..8 {
                repr.push_str(format!("| {} ", self.pieces[i * 8 + j]).as_str());
            }
            repr.push_str(format!("| {}\n", 8 - i).as_str());
        }
        repr.push_str("  +---+---+---+---+---+---+---+---+\n");
        repr.push_str("    a   b   c   d   e   f   g   h\n");
        f.write_str(repr.as_str())?;
        Ok(())
    }
}

fn main() {
    let board = Board::new();
    println!("{}", board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_i_to_j() {
        let mut b = Board::new();
        b.move_i_to_j(8, 16);
        assert_eq!(b.pieces[8], Piece::Blank);
        assert_eq!(b.pieces[16], Piece::BlackPawn);
    }
}
