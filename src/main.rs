use std::io::Write;

fn main() {
    let mut cells = [0 as u8; 1024];
    let mut bf = Brainfuck::new(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.++
+.------.--------.>>+.>++.",
        &mut cells,
    );
    bf.run(&bf.parse((0, 0)));
}

struct Brainfuck {
    ptr: usize,
    cells: Vec<u8>,
    text: String,
}

impl Brainfuck {
    fn new(text: &str, cells: &mut [u8]) -> Self {
        Brainfuck {
            ptr: 0,
            cells: cells.to_vec(),
            text: text.to_string(),
        }
    }

    fn run(&mut self, tokens: &Vec<Token>) {
        for token in tokens {
            match token {
                Token::Move(value) => {
                    self.ptr = (self.ptr as isize + value) as usize;
                }
                Token::Add(value) => {
                    if let Some(cell) = self.cells.get_mut(self.ptr) {
                        *cell = (*cell as isize + value) as u8
                    }
                }
                Token::Output => {
                    if let Some(cell) = self.cells.get(self.ptr) {
                        print!("{}", *cell as char);
                        std::io::stdout().flush().unwrap();
                    }
                }
                Token::Input => {
                    if let Some(cell) = self.cells.get_mut(self.ptr) {
                        let mut a = String::new();
                        if let Ok(_) = std::io::stdin().read_line(&mut a) {
                            if let Some(in_char) = a.bytes().next() {
                                *cell = in_char;
                            }
                        }
                    }
                }
                Token::Loop(value) => unsafe {
                    while self.cells.get_unchecked(self.ptr) > &0 {
                        self.run(value);
                    }
                },
            }
        }
    }

    fn parse(&self, mut char_location: (usize, usize)) -> Vec<Token> {
        let text = match char_location {
            (0, 0) => &self.text[0..],
            (_, 0) => &self.text[char_location.0..],
            _ => &self.text[char_location.0..char_location.1],
        };
        let mut temp_tokens: Vec<Token> = Vec::new();
        for ch in text.chars() {
            match ch {
                '>' => {
                    temp_tokens.push(Token::Move(1));
                }
                '<' => {
                    temp_tokens.push(Token::Move(-1));
                }
                '+' => {
                    temp_tokens.push(Token::Add(1));
                }
                '-' => {
                    temp_tokens.push(Token::Add(-1));
                }
                '.' => {
                    temp_tokens.push(Token::Output);
                }
                ',' => {
                    temp_tokens.push(Token::Input);
                }
                '[' => {
                    if let Some(location) = self.find_bracket_location(char_location.0 + 1) {
                        temp_tokens.push(Token::Loop(self.parse((char_location.0 + 1, location))));
                        temp_tokens.append(&mut self.parse((location, 0)));
                        break;
                    }
                }
                ']' => {
                    break;
                }
                _ => {}
            }
            char_location.0 = char_location.0 + 1;
        }
        return temp_tokens;
    }
    fn find_bracket_location(&self, mut char_location: usize) -> Option<usize> {
        let mut bracket = 0usize;
        let text = &self.text[char_location..];
        for char in text.chars() {
            match char {
                '[' => {
                    bracket = bracket + 1;
                }
                ']' => {
                    if bracket == 0 {
                        return Some(char_location + 1);
                    } else {
                        bracket = bracket - 1;
                    }
                }
                _ => {}
            }
            char_location = char_location + 1;
        }
        return None;
    }
}

enum Token {
    Move(isize),
    Add(isize),
    Output,
    Input,
    Loop(Vec<Token>),
}
