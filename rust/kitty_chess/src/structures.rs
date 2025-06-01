#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}
pub enum SymbolStyle {
    Unicode,
    Ascii,
}
impl Piece {
    /// Retorna o caractere no estilo dado:
    /// - Se `style == SymbolStyle::Unicode`, devolve “♔, ♕, …” ou “♚, ♛, …”.
    /// - Se `style == SymbolStyle::Ascii`, devolve “K, Q, …” ou “k, q, …”.
    pub fn symbol(&self, style: &SymbolStyle) -> char {
        match (&self.kind, &self.color, &style) {
            // Unicode
            (&PieceKind::King, &Color::White, &SymbolStyle::Unicode) => '♔',
            (&PieceKind::Queen, &Color::White, &SymbolStyle::Unicode) => '♕',
            (&PieceKind::Rook, &Color::White, &SymbolStyle::Unicode) => '♖',
            (&PieceKind::Bishop, &Color::White, &SymbolStyle::Unicode) => '♗',
            (&PieceKind::Knight, &Color::White, &SymbolStyle::Unicode) => '♘',
            (&PieceKind::Pawn, &Color::White, &SymbolStyle::Unicode) => '♙',

            (&PieceKind::King, &Color::Black, &SymbolStyle::Unicode) => '♚',
            (&PieceKind::Queen, &Color::Black, &SymbolStyle::Unicode) => '♛',
            (&PieceKind::Rook, &Color::Black, &SymbolStyle::Unicode) => '♜',
            (&PieceKind::Bishop, &Color::Black, &SymbolStyle::Unicode) => '♝',
            (&PieceKind::Knight, &Color::Black, &SymbolStyle::Unicode) => '♞',
            (&PieceKind::Pawn, &Color::Black, &SymbolStyle::Unicode) => '♟',

            // ASCII
            (&PieceKind::King, &Color::White, &SymbolStyle::Ascii) => 'K',
            (&PieceKind::Queen, &Color::White, &SymbolStyle::Ascii) => 'Q',
            (&PieceKind::Rook, &Color::White, &SymbolStyle::Ascii) => 'R',
            (&PieceKind::Bishop, &Color::White, &SymbolStyle::Ascii) => 'B',
            (&PieceKind::Knight, &Color::White, &SymbolStyle::Ascii) => 'N',
            (&PieceKind::Pawn, &Color::White, &SymbolStyle::Ascii) => 'P',

            (&PieceKind::King, &Color::Black, &SymbolStyle::Ascii) => 'k',
            (&PieceKind::Queen, &Color::Black, &SymbolStyle::Ascii) => 'q',
            (&PieceKind::Rook, &Color::Black, &SymbolStyle::Ascii) => 'r',
            (&PieceKind::Bishop, &Color::Black, &SymbolStyle::Ascii) => 'b',
            (&PieceKind::Knight, &Color::Black, &SymbolStyle::Ascii) => 'n',
            (&PieceKind::Pawn, &Color::Black, &SymbolStyle::Ascii) => 'p',
        }
    }
}
