fn main() {
    // Chess:
    let chess_start_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let starting_board = fen_to_board(chess_start_fen);
    for i in 0..starting_board.len() {
        println!("{:?}", starting_board[i]);
    }
}

enum Piece {
    Pawn = "p",
    Knight = "n",
    Bishop = "b",
    Rook = "r",
    Queen = "q",
    King = "k",
    Blank = "",
}

fn fen_to_board(locations: &str) -> Vec<((i32, i32), &str)> {
    let mut result = Vec::new();
    for (i, c) in locations.chars().enumerate() {
        let row = i / 8;
        let col = i % 8;
        if c != '/' {
            result.push(((row, col), &c.to_string()));
        }
    }
    return &result;
}