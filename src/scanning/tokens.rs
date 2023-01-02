#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single-character tokens
    Plus,
    // Minus,
    // Slash,
    // Star,
    LeftParenthesis,
    RightParenthesis,

    // One or two character tokens
    Equal,

    // Literal tokens
    Number(f64),
    Identifier(String),

    // Mechanics token
    Eof,
    Eol,

    // Keywords
    Qty,
}
