//! Learn more about variables in the Quantity programming language
//! # Variables
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
//! The following grammar rules show how variables are declared and re-assigned
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
//! # Variables and Quantity's Laws
//! This section explains how the [Laws of Quantity](https://joeyame.github.io/quantity/quantity/docs/laws/index.html)
//! influenced the design and implementation of variables for this language.
//!
//! ### 🎁 Law of Amenity
//! Documentation comments added before or after a variable declaration will be
//! accessible throughout the code base and in the exported documentation.
//!
//! The type of a variable can easily be determined before runtime.
//!
//! ### 📝 Law of Brevity
//! Type of a variable can be implicitly determined by assigning it a value, saving
//! on the overall amount of code that must be written.
//!
//! Assignments return the assigned value, allowing for stacked and more concise code.
//!
//! ### 🪢 Law of Tensity
//! Declaring a new variable will result in 100% repeatable behavior, 100% of the
//! time. Even if you don't fully initialize a value, the default initial value will
//! always be the same.
//!
//! ### 🐇 Law of Rabbity
//! The Desert Cottontail can live up to two years. Your variables can live longer if
//! you play your cards right.
