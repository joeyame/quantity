//! Learn more about the axioms driving the design of the Quantity programming language
//! # Behavior is Defined
//! It has been a constant pain point in my professional career that C and C++ leave so
//! many issues up to interpretation by the compiler. Many of the tricks developers use
//! to improve their code are not guaranteed to work on every deployment.
//!
//! Quantity attempts to fully define all behavior, and will forbid code that results
//! in undefined behavior.
//!
//! This gives you the confidence to write code without having to consider every edge
//! case
//!
//! # Boilerplate is Wrong
//! If you are a fan of boilerplate, this language is not for you. I highly recommend
//! [this language instead.](https://en.wikipedia.org/wiki/Java_(programming_language))
//!
//! Many languages nowadays - like Rust - have mechanics in place to help minimize the
//! boilerplate you write. This is a good and inspiring thing for Quantity. However, it
//! does not go far enough. Every single unnecessary character is a waste of your time.
//!
//! Quantity aims to eliminate every unnecessary character possible without negatively
//! affecting the readability of your code.
//! 
//! # Limitations are Few
//! Unlike many other languages, Quantity will not get in your way. It empowers you to
//! write code that you are proud of. Much of the syntax allows for options and different
//! forms 
//!
//! Similar to most languages, variables must be declared before use. Unlike
//! C-style languages, an empty declaration does initialize a variable to the
//! type's default value. In the following block of code, `velocity` will
//! always start out as `0.0`. This helps prevent issues that might come up in
//! C-style code if variables are not explicitly initialized to a default value.
//!
//! ```qty
//! f32 velocity
//! ```
//!
//! Because a value isn't explicitly passed into `velocity`, it is necessary
//! to define its type within the declaration line.
//!
//! If we instead define a variable called `flag` and give it the initial value
//! of `true`, the language can implicitly determine what its type is, so we need
//! not explicitly state it.
//!
//! ```qty
//! let flag = true
//! ```
//!
//! # Grammar
//! The following  show how variables are declared and re-assigned
//! in `Quantity`.
//! ```text
//! declaration   -> ( type | "let" ) IDENTIFIER ( "=" expression )?
//! reassignment  -> IDENTIFIER "=" expression
//! ```
//!
//! **Note** that variable `declarations` and `reassignments` themselves are
//! expressions that will return a value. That means the following code block
//! is valid code in Quantity
//!
//! ```qty
//! let first_flag = let second_flag = false
//! ```
//!
//! # Variables and Quantity's Axioms
//! This section explains how the behavior of variables fits the axioms for
//! this language.
