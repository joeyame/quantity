//! This module contains documentation related to the language itself
//!
//! # Core Concepts
//! * Quantity attempts to fit the market for scientific computing in ways that
//! no other language has done before. It accepts a blend of syntax between
//! higher level languages like Python and lower level languages like C. This
//! empowers you to write code as you like!
//!
//! * Due to the multitude of problems that come up in larger python projects,
//! Quantity does not support dynamic typing like Python.
//!
//! # Quick Start
//! To use this language, clone the github repository and build the project
//! using the `cargo build` command. Create a .qty script file and give it
//! the following contents:
//!
//! ```qty
//! main = ||:
//!     print "Greetings, World!"
//! ```
//!
//! As you can see, `main` is the entrypoint of a Quantity program.
//!
//! # Common Programming Concepts
//! This section provides a brief overview of concepts that appear in almost
//! every programming language and how they work in Quantity. Each subsection
//! links to its own documentation that explains them in even more detail.
//!
//! ### Variables
//! Variables in Quantity are statically typed. While this does add some
//! complexity to the code produced - it provides the following benefits
//! - You will always know what the type of a variable is because it is
//! explicitly stated in the code. *No more errors caused by incorrect types
//! being passed into functions!*
//! - Data can be processed faster because type checking is not necessary at
//! runtime.
//!
//! [More about variables...](variables)

// Bring in the other documentation sections
mod axioms;
mod functions;
mod variables;
