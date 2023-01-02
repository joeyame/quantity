# Lexical Grammar for Quantity
Quantity has a simple lexical grammar that is similar to C-style languages.

### What is a lexical grammar?
The lexical grammar of a language includes the lowest-level components that make up a sentence. For example:
* Characters (a, b, c, d, e, f, g, ...)
* Numbers (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
* Symbols ( , [ ] ) . ! ...

### Lexical Contents of the Quantity language
As quantity is still a very basic language, it has a limited set of lexical features. This set includes
* Operators `+` `=`
* Parenthesis `(` `)`
* Keywords
    - `qty`
* Identifiers that start with a letter and can contain `_` or numbers.
* Numbers that can contain a decimal

### Future Plans
I am still working on the language and determining how it should function and look. Various changes, some breaking ones, should be expected as the language is implemented and matured.

I am toying with the idea of executing the code from a markdown file. This would give code files rich commenting functionality, and might make writing documentation much easier.