# brainfuck_interpreter.rs
It is a simple Brainfuck interpreter. If you want you can use or change.

## Example
Sorry it's not a module for now, so you have to copy all of the source code.😋
```Rust
let mut cells = [0 as u8; 1024]; //u8 Array for cells (Except u8 not working for now)

let mut bf = Brainfuck::new(",[>+.<-]", &mut cells); //For create a new brainfuck object

let tokens = bf.parse((0,0)); //For tokenize the text in the created object
                              //bf.parse((5,0)) If the second variable is 0, the rest of the text is tokenized.
                              //bf.parse((0,5)) If the first variable is 0, the first part of text is tokenized.

bf.run(&tokens)//For running interpreter
```
