//! Learn more about Operators in the Quantity programming language
//! # Operators
//! Quantity has a mix of operators for productivity and familiarity. They
//! are split into various categories of operators like pretty much any
//! other programming language. Furthermore, structure is in place to define
//! new operators using the Hump Expression.
//!
//! ### The Hump Expression
//! The Hump Expression allows you to overload existing operators to add support
//! for new types. In addition, it is even possible to add new operators using a
//! `hump` expression. The below example is the hump expression for the addition
//! operator.
//!
//! ```qty
//! _+_
//! ```
//!
//! Normally a special symbol like this cannot be used as an identifier because
//! it would allow for very unreadable expressions. The Hump Expression is your
//! way of telling `Quantity` to interpret the symbol as an identifier instead of
//! an operator. To use it, just surround a single special symbol in underscores.
//!
//! The following code block shows how the humps get tokenized by the scanner
//! ```qty
//! 4 + 3   // NumToken(4),Addition,NumToken(3)
//! 4 _+_ 3 // NumToken(4),Identifier("+"),NumToken(3)
//! ```
//!
//! ##### Notes
//! * Identifier("+") is identical to the Addition token
//! * Defining new behavior for an operator is done by generating a new function
//!   with the proper number of arguments. The proper number is defined per-operator.
//! * Read up on function call forms to learn about how the operator-as-function system
//!   works behind the scene.
//!
//! ### Standard Arithmetic Operations
//! These operators are used to perform arithmetic operations on variables and
//! literals.
//!
//! | Operator | Hump  | Arguments | Operation      |
//! | -------- | ----- | --------- | -------------- |
//! | `+`      | `_+_` | lhs, rhs  | Addition       |
//! | `-`      | `_-_` | lhs, rhs  | Subtraction    |
//! | `*`      | `_*_` | lhs, rhs  | Multiplication |
//! | `/`      | `_/_` | lhs, rhs  | Division       |
//! | `%`      | `_%_` | lhs, rhs  | Modulo         |
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
