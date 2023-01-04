//! # The Quantity Programming Language
//! A type-driven, mid-level programming language meant for scientific computation and simulation
//!
//! The main design points driving the creation of this programming language are:
//!
//! | Concept    | Description                                                      |
//! | ---------- | ---------------------------------------------------------------- |
//! | Explicit   | Quantity code shall have no undefined behavior                   |
//! | Beautiful  | Quantity code shall be easy to read and understand               |
//! | Functional | Quantity code shall be purely functional yet organized           |
//! | Expressive | Quantity code shall require minimal boilerplate to be productive |
//!
//! ## Table of Contents
//! The following table will link to the official documentation for this project
//!
//! | Page                                                                     | Description                                                   |
//! | ------------------------------------------------------------------------ | ------------------------------------------------------------- |
//! | [Docs](https://joeyame.github.io/quantity/quantity/docs)                 | Documentation related to the language itself                  |
//! | [Interface](https://joeyame.github.io/quantity/quantity/interface)       | Documentation about how the interpreter can be run            |
//! | [Interpreting](https://joeyame.github.io/quantity/quantity/interpreting) | Documentation about the interpreter itself                    |
//! | [Parsing](https://joeyame.github.io/quantity/quantity/parsing)           | Documentation about how the AST is generated                  |
//! | [Scanning](https://joeyame.github.io/quantity/quantity/scanning)         | Documentation about how the source code is parsed into tokens |
//!
// End of readme documentation
// !
// ! ## Quantity Code Examples
// ! The following snippets of code show what is (or will be) possible using the quantity programming language
// !
// ! ### Quantities
// ! Quantities are the backbone of this language, so it only makes sense to start this guide by making some quantities.
// !
// ! ```qty
// ! // Declaring new quantities for the program
// ! qty length
// ! qty area (length*length)
// ! qty volume (length*area)
// ! qty time
// ! qty rate (length/time)
// ! ```
// !
// ! As you can see, a quantity is basically a system of measurement. They are almost like different realms of numbers. You can add and subtract two `lengths` together, but you cannot add a length to an area or volume. We can, however, move between quantities using multiplication and division. For example, two lengths multiplied together produce an area quantity.
// !
// ! In simple terms, you can think of the quantities as generic data types that can be satisfied by any of their member units.
// !
// ! ### Units
// ! Units and quantities go hand-in-hand. Think of quantities as groupings of units. The length quantity contains meters, feet, lightyears, etc. These can freely be added, subtracted, or even converted between each other.
// !
// ! ```qty
// ! // Declaring new units for the program
// ! length meter|meters|m
// ! length foot|feet|ft = 0.3048m
// ! ```
// !
// ! The first example line above is the declaration of a base unit. It does not need any conversions. Afterwards, the feet unit is declared in relation to the meter unit.
// !
// ! ### Number Literals
// ! Now that we have quantities and units defined, we can start to create number literals with them.
// !
// ! For example, if `34 meters` ever shows up in the code, it will generate a value of 34 and attach it to the meter 'type'.
// !
// ! The following code block will show the end value of various expressions:
// ! ```
// ! 45 meters           // 45 meters
// ! 3.281ft             // 1 meter
// ! 1meter * 3.281 feet // 1 meter*meter
// ! ```
// !
// ! ### Name Literals
// ! Naming conventions are much more lax in Quantity than perhaps any other programming language. A name can contain any of the following characters:
// !
// ! | Category   | Acceptable Characters | Start of Name |
// ! | ---------- | --------------------- | ------------- |
// ! | Alphabetic | `a-z` `A-Z`           | Yes           |
// ! | Numeric    | `0-9`                 | No            |
// ! | Symbolic   | `- + * / etc`         | Yes           |
// ! | Emojis     | all                   | Yes           |
// ! *The Start of Name column states whether or not the name can begin with that category of text.*
// !
// !
// ! ### Functions
// ! Since Quantity is a functional language, it has first-class support for functions, which end up being extremely similar to closures in Rust. These functions also include rich support for generic types based on quantities.
// !
// ! ```Rust
// ! area rect_area = |length len, length wid| len * wid
// ! area tri_area = |length base, length height| base * height
// ! ```
// !
// ! There are three main ways to call functions:
// ! 1. Traditional Syntax<br>
// !     The traditional approach is the same across virtually all programming languages. It involves using the function name and passing in every argument.
// !     ```Rust
// !     rect_area(45ft, 10m) // Returns an area value which carries the ft*m flag
// !     ```
// ! 2. Method Syntax<br>
// !     While `Quantity` is not object oriented, we can still benefit from the easier organization that object oriented languages sometimes provide. To use this syntax, the first argument is treated as an object, the function name is treated as a member, and the other arguments are passed into the function as normal.
// !     ```Rust
// !     45ft.rect_area(10m) // Returns an area value which carries the ft*m flag
// !     ```
// ! 3. Operator Syntax<br>
// !     Any function that takes two arguments can be used as an operator. This mostly makes sense with functions that perform actual operations, but this language feature can be used by any dual-argument function.
// !     ```Rust
// !     45ft rect_area 10m // Returns an area value which carries the ft*m flag
// !     ```

use anyhow::Result;
use clap::Parser;
use interface::*;
use std::path::PathBuf;

// Load in core modules
// #[doc(hidden)]
mod docs;
#[doc(hidden)]
mod interface;
#[doc(hidden)]
mod interpreting;
#[doc(hidden)]
mod parsing;
#[doc(hidden)]
mod scanning;

#[doc(hidden)]
fn main() -> Result<()> {
    // Get command line arguments
    let cli = Cli::parse();

    match cli.filename {
        // If filename is provided, read and execute the file
        Some(name) => run_file(PathBuf::from(name))?,

        // Otherwise run the interactive REPL
        None => run_prompt()?,
    }

    // Exit with a success
    Ok(())
}

#[doc(hidden)]
pub fn run(code: String) {
    println!("Scanning the following line:");
    println!("\"{}\"", code);

    // Turn source code into tokens
    let tokens = crate::scanning::scan_source(code);
    println!("Token list: {:#?}", tokens);

    // Turn tokens into an abstract syntax tree
    let tree = crate::parsing::Parser::parse_tokens(tokens);
    println!("AST {:#?}", tree);

    // Interpret the syntax tree
    crate::interpreting::interpret(tree).expect("Interpreter failed");
}
