# Syntactic Grammar for Quantity
The Syntactic grammar for Quantity will be very unique. It is a functional language that does allow the grouping of data in structures. The specialty aspect of it, though, is that it has first-class support for connecting units to numeric values. This can prevent confusion when working with measurements or other data forms that can be input in a variety of ways.

For example, the following expression:
```
40ft + 20m
```
is valid in the quantity language, and the result will NOT be 60!

### What is a Syntactic Grammar
Similar to the lexical grammar, the syntactic grammar is a somewhat higher level set of rules for how the language is formed. Now we are done with individual characters and move on to how sentences are formed. 

For this document, the syntax of the grammar will be expressed in the form given by the author of crafting interpreters, in [section 5.1](https://craftinginterpreters.com/representing-code.html#context-free-grammars).

### Language Rule Set
```
statement      -> quantity
                | unit
                | declaration
                | expression

quantity       -> "qty" type composition?
composition    -> "(" quantity_list ( "/" quantity_list ) ")"
quantity_list  -> type ( "*" type )+
unit           -> IDENTIFIER ( "|" IDENTIFIER )+ ( "=" expression )?

declaration    -> type IDENTIFIER = expression

expression     -> closure
                | literal
                | IDENTIFIER

closure        -> "|" arguments+ "|"

arguments      -> 
operation      -> IDENTIFIER operator 
literal        -> NUMBER type
type           -> IDENTIFIER
```

### Example Snippets
The following code snippets are all valid forms of Quantity code.

#### Quantity Declaration
To use quantities within a program, you must first declare the given quantity.
```
qty length picometer(pm)
qty weight picogram(pg)
```

The general form is
```
qty <quantity_name> <base_unit>
```

#### Unit Declaration
Once a quantity is declared, additional unit systems can be assigned to it.

```
unit meter(m) = 1000000000000pm
unit feet(ft) = 3281000000000pm
```