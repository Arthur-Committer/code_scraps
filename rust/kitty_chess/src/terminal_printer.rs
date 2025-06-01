use crate::structures::{Piece, SymbolStyle};

const TOP_BORDER: &str = "  ┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓\n";
const MID_BORDER: &str = "  ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫\n";
const BOTTOM_BORDER: &str = "  ┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛\n";
const FILE_LETTERS: &str = "    a   b   c   d   e   f   g   h\n";
pub fn print_chess_board(chess_board: &[[Option<Piece>; 8]; 8]) {
    let style = SymbolStyle::Unicode;
    let mut buffer = String::new();

    buffer.push_str(TOP_BORDER);
    for (i, row) in chess_board.iter().enumerate() {
        buffer.push_str(&format!("{} ┃", 8 - i));
        for cell in row.iter() {
            let ch = match cell {
                Some(piece) => piece.symbol(&style),
                None => ' ',
            };
            buffer.push_str(&format!(" {} ┃", ch));
        }
        buffer.push('\n');
        if i < 7 {
            buffer.push_str(MID_BORDER);
        }
    }
    buffer.push_str(BOTTOM_BORDER);
    buffer.push_str(FILE_LETTERS);

    print!("{}", buffer);
}
