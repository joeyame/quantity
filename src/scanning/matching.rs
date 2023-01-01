//! The matching module contains various utility functions to make the pattern matching
//! part of the scanner less complex

use super::tokens::Token;
use super::tokens::Token::*;

/// Skips the current line by searching for the nearest newline character and returning its index
/// If the index is too high, it will instead return the maximum index of the source string
pub fn skip_line(text: &[char], cursor: usize) -> (Option<Token>, usize) {
    let end_index = text.iter().position(|&c| c == '\n').unwrap_or(text.len());
    (None, end_index + cursor)
}

/// Parses a number out of a series of numerical characters
pub fn match_number(text: &[char], cursor: &usize) -> (Option<Token>, usize) {
    let mut found_decimal = false;
    let number_end_index = text
        .iter()
        .position(|&c| match c {
            '0'..='9' => false,

            // Only in the number if we haven't seen a decimal yet
            '.' => {
                found_decimal = !found_decimal;
                !found_decimal
            }

            // No longer in the number
            _ => true,
        })
        .unwrap_or(text.len());

    // Now create the number token
    let digits: String = text[..number_end_index].iter().collect();
    let num: f64 = digits.parse().expect("Failed to parse number!");
    (Number(num).into(), cursor + digits.len())
}

/// Parses an identifier out of a series of characters
pub fn match_identifier(text: &[char], cursor: &usize) -> (Option<Token>, usize) {
    let end_index = text
        .iter()
        .position(|&c| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(text.len());

    let name: String = text[..end_index].iter().collect();

    (
        match name.as_str() {
            "qty" => Qty,
            _ => Identifier(name),
        }
        .into(),
        cursor + end_index,
    )
}

#[cfg(test)]
mod tests {
    use super::match_identifier;
    use super::Identifier;
    use super::Token;

    fn tokenize_identifier(name: impl Into<String>, start_index: usize) -> (Option<Token>, usize) {
        let name: String = name.into();
        let name_chars: Vec<char> = name.chars().collect();
        let name_slice = &name_chars[..];
        match_identifier(&name_slice, &start_index)
    }

    #[test]
    fn test_identifiers() {
        // Test that just one word is accounted for, and the space is
        // skipped
        assert_eq!(
            tokenize_identifier("Hi there", 0),
            (Some(Identifier("Hi".into())), 2)
        );

        // Test for all valid characters
        assert_eq!(
            tokenize_identifier(
                "abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ_01234567890.hi",
                0
            )
            .0
            .unwrap(),
            Identifier("abcdefghijklmnopqrstuvwxyz_ABCDEFGHIJKLMNOPQRSTUVWXYZ_01234567890".into())
        );

        // Start with underscore
        assert_eq!(
            tokenize_identifier("_hi", 0).0.unwrap(),
            Identifier("_hi".into())
        );
    }
}
