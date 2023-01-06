//! Learn more about the laws driving the Quantity programming language
//!
//! The following laws are the major influencing factors of all decisions made during the
//! development of `Quantity`. They offer insight as to why the language is the way
//! that it is.
//!
//! # 1. 🧪 Law of Reality
//! The Law of Reality states that
//! > "All values in `Quantity` represent reality"
//!
//! One of the major uses for programming languages is solving real problems and making
//! our lives easier. Therefore, it is a *real problem* that every other language
//! forces us to strip all meaning from the values in our code.
//!
//! > For example, if you have a variable to store a distance of 30 feet, you have no way of
//! > attaching feet to that numeric value. Instead, you will have to either remember that the
//! > value is in feet, you'll have to leave a comment there that might get missed later, or
//! > you'll have to include the unit of the variable in its name. Now, imagine that you would
//! > like the user to input a distance of their choosing. Do you force them to input feet too?
//! > What if they have their value in meters? Do you code a separate conversion function for
//! > every single possible unit? This very small example is quickly turning into a big problem.
//!
//! `Quantity` allows your code to represent reality in ways no other language has achieved so
//! far. It does so through its innovative Quantity and Unit system. Units are treated as
//! secondary types to the value, allowing you to write generic functions operating on quantities
//! themselves. These systems allow you to focus more on reality and less on implementation.
//!
//! # 2. 🎁 Law of Amenity
//! The Law of Amenity states that
//! > "Working with `Quantity` is a blessing, not a chore"
//!
//! The team managing this project is focused on providing the best tooling, writing, and overall
//! developer experience there is. Our inspiration for this is first and foremost Rust, which comes
//! with built-in code documentation, testing, package management, language server, and very helpful
//! error messages *(until you get deep into generic Hell)*.
//!
//! Our goal is to make `Quantity` at least as easy to work with as Rust. We will ideally provide
//! some features for academic professionals that will make this language especially useful to them.
//! One such idea that is still in the early phases of development is embedding `Quantity` code into
//! Markdown and LaTEX documents - then running the code almost as a preprocessor on itself, like PHP.
//!
//! Such a feature would allow you to create a simulation, write a paper, and present the results all
//! in one single document! No other language has ever come close to such a workflow, but this is
//! one of the driving goals of `Quantity`.
//!
//! # 3. 📝 Law of Brevity
//! The Law of Brevity states that
//! > "Every character in `Quantity` code is important and
//! productive"
//!
//! Many languages nowadays, like Rust, have mechanics in place to help minimize
//! the amount of boilerplate that you write. Macros provide an interface for the language to
//! write itself, allowing you to focus on the things that are actually important. This is a
//! good and inspiring thing to `Quantity`, but it does not go far enough.
//!
//! Every single unnecessary character is a waste of your time. This language is
//! designed to eliminate every character possible without negatively
//! affecting the clarity of your code. Every single decision in its design has been centered
//! around minimizing the amount of work you must do as a developer.
//!
//! # 4. 🪢 Law of Tensity
//! The Law of Tensity states that
//! > "All expressions of `Quantity` code are comprehensively defined"
//!
//! With `Quantity`, there is no undefined behavior that you need to be aware of
//! when targeting different systems. One of the great disadvantages of C and C++
//! are the wide range of almost standard-practice techniques that are not technically
//! defined uses of the language. Compilers on different platforms might handle
//! certain blocks of code differently. How can you have confidence in your work
//! with that fact looming on your mind?
//!
//! In this language - every behavior is fully defined behavior. This allows you to approach
//! problems with the confidence you need to support all platforms.
//!
//! # 5. ☀️ Law of Clarity
//! The Law of Clarity states that
//! > "Code written in `Quantity` explicitly describes itself"
//!
//! While Brevity is a major component of the design of `Quantity`, it is
//! worthless if the end result is not clear. Every aspect of this language is
//! designed to be self-explanatory and clear. Once you learn the basics, you
//! will never have to repeat a search again.
//!
//! # 6. ☯️ Law of Duality
//! The Law of Duality states that
//! > "A `Quantity` program is both compilable and interpretable"
//!
//! Many languages require a trade-off between convenience and runtime
//! performance. Higher-level languages like Julia and Python both support
//! JIT-compilation but that comes at the cost of constant re-compilations.
//! Lower-level languages like Rust have great performance that comes at the
//! cost of a longer development process.
//!
//! `Quantity` bridges the gap between the two groups of languages by natively
//! supporting interpretation and compilation - sometimes within the same process!
//!
//! This support is on a level that no other language has reached before. Rust gets
//! close with its extensive macro system that is pretty much its own language, but
//! imagine writing macros for your code in the same language as the code itself!
//! This is within the realm of possibility for this language.
//!
//! # 7. 🐇 Law of Rabbity
//! The Law of Rabbity states that
//! > "Code written in `Quantity` is effortlessly fast"
//!
//! Being a language whose main focus is computation and simulation, it will be
//! unreasonable for it to lack the performance of lower-level languages like C
//! or Fortran. `Quantity's` development is heavily inspired to deliver this
//! performance while keeping the language extremely easy to use.
//!
//! The final fulfillment of the Rabbity law is that the mascot for this language
//! is the Desert Cottontail - an adorable rabbit that is native to the place where
//! this language was made.
//!
