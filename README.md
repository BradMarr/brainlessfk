# Brainlessfk
 Script in a easy to use basic style language then compile to brainfk code.

## Commands
| Command | Description | Argument 1 | Argument 2 | Argument 3 |
| - | - | - | - | - |
| def | defines variable | type | name | value (or length) |
| print | prints value | type | value |
### Ex:
```py
def input first_name 6  # defines a string named first_name with a value found in input of length 6 (if name is shorter, _ the remaining time and if it is longer then paraphrase)
def char return 13  # defines a character named return with a value of 13 (ascii of carriage return)

def str money "$504.27"  # defines a string named money with a value of "$504.27"

print str "Hello "  # prints a string with the value of "Hello "
print var first_name  # prints a variable named name
print str ","  # prints a string with the value of ","
print var return  # prints a variable named name

print str "you currently have "  # prints a string with the value of "you currently have "
print var money  # prints a variable named money
print str " in your bank account."  # prints a stromg with the value of " in your bank account."
```
compiles to:
```
>,>,>,>,>,>,>+++++++++++++>++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++<<<<<<<<<<<<<<++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.-------------------------------------------------------------------------------.>.>.>.>.>.>.<<<<<<++++++++++++.>>>>>>>.<<<<<<<+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.----------.++++++.-------------------------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++.---..-------------.+++++++++.++++++.--------.+++++++++++++.-----------------------------------------------------------------------------------------.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-------.+++++++++++++++++++++.-----------------.---------------------------------------------------------------------.>>>>>>>>.>.>.>.>.>.>.<<<<<<<<<<<<<<.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++.------------------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.----------.++++++.---.----------------------------------------------------------------------------------.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-.+++++++++++++.---.---------------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++..++++++++++++.++++++.-------.++++++.----------------------------------------------------------------------.
```
while then outputs:
```
Hello <first_name>,
you currently have $504.27 in your bank account.
```
## BrainFK Logic:
BrainFK is a minimalist esoteric programming language designed to challenge programmers and showcase the fundamentals of computing. Created in 1993 by Urban Müller, it consists of only eight commands: `>`, `<`, `+`, `-`, `[`, `]`, `.`, and `,`. These commands manipulate a simple array of memory cells and provide basic looping and input/output functionality. BrainFK programs are often written in a terse, cryptic style, making them both challenging to write and decipher. Despite its simplicity, BrainFK is Turing-complete, meaning it can compute anything that can be computed algorithmically, albeit with considerable effort. It's primarily used for educational purposes, code golfing, and as a mental exercise for programmers.

![array shift visualization](./README_SRC/array_shift.gif)
In this visualization, the squares act as memory cells within the array. 
