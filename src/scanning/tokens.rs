#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // LeftParenthesis,
    // RightParenthesis,
    // LeftBrace,
    // RightBrace,
    // Comma,
    // Dot,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens
    // Equal,

    // Literal tokens
    Number(f64),
    Identifier(String),

    // Control Flow Tokens
    Eol,
    Eof,

    // Keywords
    Let,
    Assign,
    // Qty,
}
