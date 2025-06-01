use kitty_chess::structures::{Color, Piece, PieceKind};
use kitty_chess::terminal_printer::print_chess_board;
use kitty_chess::validator::{is_a_move, pedantic_guard};
use std::io;
fn move_translator(command: &str) -> [usize; 4] {
    let b = command.as_bytes();
    let col_o = (b[0] - b'a') as usize;
    let row_o = 7 - ((b[1] - b'1') as usize); // ‘1’ → índice 7, ‘8’ → índice 0
    let col_d = (b[2] - b'a') as usize;
    let row_d = 7 - ((b[3] - b'1') as usize);
    [col_o, row_o, col_d, row_d]
}
fn move_piece(board: &mut [[Option<Piece>; 8]; 8], command: &[usize; 4]) {
    let (col_o, row_o, col_d, row_d) = (command[0], command[1], command[2], command[3]);
    print!(
        "casa de origem tem um {:?} e casa final {:?}",
        board[row_o][col_o], board[row_d][col_d]
    );

    // 1) Retira o Option<Piece> da célula de origem
    let moving_piece = board[row_o][col_o].take(); // take() → deixa None, retorna Option<Piece>

    // 2) Opcional: se moving_piece é None, quer dizer “sem peça na origem”
    if moving_piece.is_none() {
        println!("Não há peça na casa de origem.");
        return;
    }

    // 3) Se quiser ver que peça foi (por exemplo, para debug)
    // if let Some(piece) = &moving_piece {
    //     println!("Movendo peça do tipo {:?} de {:?}", piece.kind, piece.color);
    // }

    // 4) Coloca a peça no destino (substitui qualquer coisa que lá estivesse):
    board[row_d][col_d] = moving_piece;
    // Isso move o Some(Piece) para o destino. Se lá já tinha outra Some(_), ela é simplesmente apagada
    // (captura) e substituída.
}
const BACK_RANK: [PieceKind; 8] = [
    PieceKind::Rook,
    PieceKind::Knight,
    PieceKind::Bishop,
    PieceKind::Queen,
    PieceKind::King,
    PieceKind::Bishop,
    PieceKind::Knight,
    PieceKind::Rook,
];

fn main() {
    let mut chess_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];
    let back_ranks: &[(usize, Color, usize)] = &[
        (0, Color::Black, 1), // na linha 0, back‐rank de peças pretas
        (7, Color::White, 6), // na linha 7, back‐rank de peças brancas
    ];

    // 3) Para cada (linha, cor) no back_ranks, preenche com os 8 PieceKind de BACK_RANK
    for &(row, color, pawn_row) in back_ranks.iter() {
        for (col, &kind) in BACK_RANK.iter().enumerate() {
            chess_board[row][col] = Some(Piece { kind, color });
            chess_board[pawn_row][col] = Some(Piece {
                kind: PieceKind::Pawn,
                color,
            })
        }
    }
    /*
    let mut chess_board: [[char; 8]; 8] = [
        ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'], // Linha 0 - peças pretas (♛ na coluna 3, ♚ na coluna 4)
        ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'], // Linha 1 - peões pretos
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Linha 2
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Linha 3
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Linha 4
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // Linha 5
        ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'], // Linha 6 - peões brancos
        ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'], // Linha 7 - peças brancas (♕ na coluna 3, ♔ na coluna 4)
    ];*/

    let mut entry = String::new();
    loop {
        entry.clear();
        //clean the terminal
        //print!("\x1B[2J\x1B[H");
        print_chess_board(&chess_board);
        prin
        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading the entry");
        entry = entry
            .trim() // remove espaços no início e no final
            .chars() // itera sobre os caracteres
            .filter(|c| !c.is_whitespace()) // filtra qualquer caractere que seja espaço, tabulação, etc.
            .collect::<String>()
            .to_lowercase();
        if !is_a_move(&entry) {
            print!("Invalid move,try something like e2e4 or g1f3");
            continue;
        }

        println!("{}", entry);

        let movement: [usize; 4] = move_translator(&entry);
        pedantic_guard(&chess_board, &movement);
        //println!("{:?}",chess_board[movement[2]][movement[3]].take());

        //println!("{:?}",chess_board[movement[0]][movement[1]].take());
        if pedantic_guard(&chess_board, &movement) {
            move_piece(&mut chess_board, &movement);
        }
    }
}
