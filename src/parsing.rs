//! This module contains information about quantity's syntactic grammar
//! and how it is parsed into an Abstract Syntax Tree
//!
//! # Syntactic Grammar for Quantity
//! The Syntactic grammar for Quantity will be very unique. It is a functional language that does
//! allow the grouping of data in structures. The specialty aspect of it, though, is that it has
//! first-class support for connecting units to numeric values. This can prevent confusion when
//! working with measurements or other data forms that can be input in a variety of ways.
//!
//! For example, the following expression:
//! ```text
//! 40ft + 20m
//! ```
//! is valid in the quantity language, and the result will NOT be 60!
//!
//! ### What is a Syntactic Grammar
//! Similar to the lexical grammar, the syntactic grammar is a somewhat higher level set of rules
//! for how the language is structured. Now we are done with individual characters and move on to how
//! sentences are formed.
//!
//! For this document, the syntax of the grammar will be expressed in the form given by the author of
//! crafting interpreters, in
//! [section 5.1](https://craftinginterpreters.com/representing-code.html#context-free-grammars).
//!
//! ### New Language Rule Set
//! ```text
//! program        -> statement* EOF ;
//!
//! statement      -> exprStmt
//!                 | printStmt ;
//!
//! exprStmt       -> expression ;
//! printStmt      -> "print" expression ;
//!
//! expression     -> NUMBER
//!                 | IDENTIFIER ;
//! ```
//!
//! Currently this is an extremely simplistic look at the language, as it is my first attempt at creating one.
//!
//! ### Old Language Rule Set
//! ```text
//! statement      -> quantity
//!                 | unit
//!                 | declaration
//!                 | expression
//!
//! quantity       -> "qty" type composition?
//! composition    -> "(" quantity_list ( "/" quantity_list ) ")"
//! quantity_list  -> type ( "*" type )+
//! unit           -> IDENTIFIER ( "|" IDENTIFIER )+ ( "=" expression )?
//!
//! declaration    -> type IDENTIFIER = expression
//!
//! expression     -> closure
//!                 | literal
//!                 | IDENTIFIER
//!
//! closure        -> "|" arguments+ "|"
//!
//! arguments      ->
//! operation      -> IDENTIFIER operator
//! literal        -> NUMBER type
//! type           -> IDENTIFIER
//! ```

mod parser;
mod tree;

pub use parser::Parser;
pub use tree::TreeNode;
