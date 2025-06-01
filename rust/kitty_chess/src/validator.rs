use crate::structures::{Color, Piece, PieceKind};
pub fn is_a_move(entry: &str) -> bool {
    let bytes = entry.as_bytes();

    bytes.len() == 4
        && (b'a'..=b'h').contains(&bytes[0])
        && (b'1'..=b'8').contains(&bytes[1])
        && (b'a'..=b'h').contains(&bytes[2])
        && (b'1'..=b'8').contains(&bytes[3])
}
pub fn pedantic_guard(chess_board: &[[Option<Piece>; 8]; 8], movement: &[usize; 4]) -> bool {
    let [col_o, row_o, col_d, row_d] = *movement;
    let origin = chess_board[row_o][col_o];
    let destination = chess_board[row_d][col_d];
    if let Some(piece) = origin {
        // Agora podemos casar apenas no `piece.kind`
        match piece.kind {
            PieceKind::Pawn => {
                println!("É um peão!");
                print!("{} {}", row_o, row_d);
                if row_o == 6 && row_d == 4 {
                    return true;
                }
                if row_o == row_d + 1 {
                    return true;
                }
            }
            PieceKind::Knight => {
                println!("É um cavalo!");
            }
            PieceKind::Bishop => {
                println!("É um bispo!");
            }
            PieceKind::Rook => {
                println!("É uma torre!");
                if row_o == row_d || col_o == col_d {
                    return true;
                }
            }
            PieceKind::Queen => {
                println!("É uma rainha!");
            }
            PieceKind::King => {
                println!("É um rei!");
            }
        }
    } else {
        // Se origin == None: não há peça na casa de origem
        println!("Casa vazia.");
    }
    false
}
