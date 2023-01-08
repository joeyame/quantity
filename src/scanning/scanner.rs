use super::tokens::{Token, Token::*};
// use super::tokens::TokenType::*;
use super::matching::*;

pub fn scan_source(source: impl Into<String>) -> Vec<Token> {
    let source: String = source.into(); // Turn source into a string
    let source: Vec<char> = source.chars().collect(); // Consume that string into a vector of bytes
    let mut cursor = 0;

    // Use Rust's powerful pattern matching to scan for tokens
    let mut token_list = Vec::<Token>::new();
    while cursor < source.len() {
        let (token, new_cursor) = match &source[cursor..] {
            // Comments
            ['/', '/', rest @ ..] => skip_line(rest, cursor + 2),

            // Literals (number, identifier, hump)
            [rest @ ..] if ('0'..='9').contains(&rest[0]) => match_number(rest, &cursor),
            [rest @ ..] if valid_identifier_char(rest[0], true) => match_identifier(rest, &cursor),
            // ['_', rest @ .., '_', ..] if !contains_whitespace(rest) => (
            //     Identifier(rest.iter().collect::<String>()).into(),
            //     rest.len() + 2,
            // ),

            // Skip whitespace
            ['\n', ..] => (Eol.into(), cursor + 1),
            [' ' | '\t' | '\r', ..] => (None, cursor + 1),

            // Operators
            ['+', ..] => (Plus.into(), cursor + 1),
            ['-', ..] => (Minus.into(), cursor + 1),
            ['*', ..] => (Star.into(), cursor + 1),
            ['/', ..] => (Slash.into(), cursor + 1),
            ['=', ..] => (Assign.into(), cursor + 1),

            // Bracketing
            // ['(', ..] => (LeftParenthesis.into(), cursor + 1),
            // [')', ..] => (RightParenthesis.into(), cursor + 1),

            // Error
            _ => panic!("Scanner not yet implemented!"),
        };

        // Move cursor to new spot and add token if not None
        cursor = new_cursor;
        if let Some(tok) = token {
            token_list.push(tok);
        }
    }
    token_list.push(Eof);
    token_list
}

// pub fn scan_tokens(source: impl Into<String>) -> Vec<Token> {
//     let source: String = source.into();
//     let source_bytes = source.into_bytes();
//     let mut token_list = Vec::<Token>::new();

//     let mut cursor: usize = 0;

//     while cursor < source_bytes.len() {
//         println!("Currently scanning {}", source_bytes[cursor] as char);
//         // Scan literals here
//         if let Some(literal_token) = scan_literals(&source_bytes, &mut cursor) {
//             token_list.push(literal_token);
//             continue;
//         }
//         // Scan two-char tokens here
//         if let Some(double_char_token) = scan_two_char_token(&source_bytes, &mut cursor) {
//             token_list.push(double_char_token);
//             continue;
//         }
//         // Scan single char tokens here
//         if let Some(single_char_token) = scan_single_char_token(&source_bytes, &mut cursor) {
//             token_list.push(single_char_token);
//             continue;
//         }
//     }

//     // Add end-of-file token
//     token_list.push(Eof);

//     // Return the token list
//     token_list
// }

// fn scan_literals(source: &Vec<u8>, current_index: &mut usize) -> Option<Token> {
//     if *current_index == source.len() {
//         return None;
//     }
//     match source[*current_index] {
//         // Number literal
//         b'0'..=b'9' => {
//             let mut end_index = *current_index;
//             while end_index < source.len() && b'0' <= source[end_index] && source[end_index] <= b'9'
//             {
//                 end_index += 1;
//             }
//             if end_index < source.len() && source[end_index] == b'.' {
//                 end_index += 1;

//                 while end_index < source.len()
//                     && b'0' <= source[end_index]
//                     && source[end_index] <= b'9'
//                 {
//                     end_index += 1;
//                 }
//             };

//             // Now create the number token
//             let digits = &source[*current_index..end_index];
//             let num_str = std::str::from_utf8(digits).expect("Failed to turn number into string");
//             let num = num_str.parse().expect("Failed to parse number!");
//             *current_index = end_index;
//             Number(num).into()
//         }

//         // Identifier literal
//         b'a'..=b'z' => {
//             // search for end of identifier
//             let mut end_index = *current_index;
//             while end_index < source.len()
//                 && match source[end_index] {
//                     b'a'..=b'z' => true,
//                     b'_' => true,
//                     _ => false,
//                 }
//             {
//                 end_index += 1;
//             }

//             let name = String::from_utf8(source[*current_index..end_index].into())
//                 .expect("Could not read identifier!");

//             *current_index = end_index;
//             Identifier(name).into()
//         }
//         _ => None,
//     }
// }

// fn scan_two_char_token(source: &Vec<u8>, current_index: &mut usize) -> Option<Token> {
//     // Don't execute this function if there is only one character remaining!
//     if *current_index + 1 == source.len() {
//         return None;
//     }

//     let new_token = match source[*current_index..*current_index + 2] {
//         [b'/', b'/'] => {
//             // This is a comment. Continue searching until the end of the line
//             if let Some(new_index) = source[*current_index..].iter().position(|&e| e == b'\n') {
//                 *current_index = new_index + 1;
//             } else {
//                 *current_index = source.len();
//             };
//             None
//         }
//         _ => None,
//     };

//     if new_token.is_some() {
//         *current_index += 2;
//     };

//     new_token
// }

// fn scan_single_char_token(source: &Vec<u8>, current_index: &mut usize) -> Option<Token> {
//     let new_token = match source[*current_index] {
//         b'+' => Plus.into(),
//         b'(' => LeftParenthesis.into(),
//         b')' => RightParenthesis.into(),

//         // Skip over whitespace
//         b' ' | b'\n' | b'\r' | b'\t' => {
//             *current_index += 1;
//             None
//         }

//         unknown => panic!("Unknown token: {}", unknown as char),
//     };

//     if new_token.is_some() {
//         *current_index += 1;
//     };

//     new_token
// }

// #[cfg(test)]
// mod tests {
//     use super::scan_literals;
//     use super::scan_tokens;
//     use super::Token::*;

//     #[test]
//     fn tokenize_numbers() {
//         // Test on decimal numbers
//         let mut num = 0.1;
//         while num < 100000000.0 {
//             println!("Testing tokenization of {}", num);
//             let num_str = num.to_string();
//             let code: Vec<u8> = num_str.as_bytes().into();
//             let mut cursor = 0;
//             let tok = scan_literals(&code, &mut cursor);
//             assert!(tok.is_some());
//             match tok.unwrap() {
//                 Number(received_num) => assert_eq!(received_num, num),
//                 _ => assert!(false, "Received a non-numeric token!"),
//             }

//             num *= 5.1;
//         }

//         // Test on integer numbers
//         let mut num = 0;
//         while num < 100000000 {
//             println!("Testing tokenization of {}", num);
//             let num_str = num.to_string();
//             let code: Vec<u8> = num_str.as_bytes().into();
//             let mut cursor = 0;
//             let tok = scan_literals(&code, &mut cursor);
//             assert!(tok.is_some());
//             match tok.unwrap() {
//                 Number(received_num) => assert_eq!(received_num, num as f64),
//                 _ => assert!(false, "Received a non-numeric token!"),
//             }

//             num += 10000000;
//         }
//     }

//     #[test]
//     fn tokenize_names() {
//         let mut name = String::new();
//         for char in 'a'..='z' {
//             // Add to name
//             name += &char.to_string();
//             println!("Tokenizing {}", name);

//             // Actually tokenize the name
//             let code: Vec<u8> = name.clone().into_bytes();
//             let mut cursor = 0;
//             let tok = scan_literals(&code, &mut cursor);
//             assert!(tok.is_some());
//             match tok.unwrap() {
//                 Identifier(tok_name) => assert_eq!(tok_name, name),
//                 _ => assert!(false, "Received a non-identifier token!"),
//             }
//         }

//         // underscore in name
//         name = name + "_test";
//         println!("Tokenizing {}", name);
//         let code: Vec<u8> = name.clone().into_bytes();
//         let mut cursor = 0;
//         let tok = scan_literals(&code, &mut cursor);
//         assert!(tok.is_some());
//         match tok.unwrap() {
//             Identifier(tok_name) => assert_eq!(tok_name, name),
//             _ => assert!(false, "Received a non-identifier token!"),
//         }
//     }

//     #[test]
//     fn tokenize_quantity_declaration() {
//         let raw = "qty length picometer(pm)";
//         // let code = raw.into_bytes();
//         // let mut cursor = 0;
//         let tok = scan_tokens(raw);
//         println!("{:#?}", tok);
//     }
// }
