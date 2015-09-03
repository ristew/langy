use std::io;
use std::io::{Read, Bytes, Write};

struct CharBuffer {
    chars: Bytes<io::Stdin>,
}

impl CharBuffer {
    pub fn new() -> CharBuffer {
        CharBuffer {
            chars: io::stdin().bytes(),
        }
    }

    pub fn getchar(&mut self) -> u8 {
        self.chars.next().unwrap().unwrap()
    }
}


enum Token {
    Add,
    Sub,
    Gt,
    Lt,
    Out,
    In,
    Begin,
    End,
}

pub fn interpret(program: String) {
    let debug: bool = false;
    let mut stack: Vec<i32> = vec![0; 1000];
    let mut scopes: Vec<usize> = Vec::new();
    let mut pos = 0;
    let mut toks: Vec<Token> = Vec::new();
    let mut charbuf = CharBuffer::new();
    let mut i = 0;
    for c in program.chars() {
        match c {
            '+' => toks.push(Token::Add),
            '-' => toks.push(Token::Sub),
            '>' => toks.push(Token::Gt),
            '<' => toks.push(Token::Lt),
            '.' => toks.push(Token::Out),
            ',' => toks.push(Token::In),
            '[' => {
                toks.push(Token::Begin);
            },
            ']' => {
                toks.push(Token::End);
            },
            _ => {},
        }
    }
    let mut ticks = 0;
    while i < toks.len() {
        match toks[i] {
            Token::Add  => {
                *stack.get_mut(pos).unwrap() += 1;
            },
            Token::Sub => {
                *stack.get_mut(pos).unwrap() -= 1;
            },
            Token::Gt => {
                pos += 1;
            },
            Token::Lt => {
                pos -= 1;
            },
            Token::Out => {
                let c = *stack.get(pos).unwrap();
                print!("{}", c as u8 as char);
                io::stdout().flush().unwrap();
            },
            Token::In => {
                *stack.get_mut(pos).unwrap() = charbuf.getchar() as i32;
            },
            Token::Begin => {
                if *stack.get(pos).unwrap() != 0 {
                    scopes.push(i);
                } else {
                    let mut cnt = 0;
                    let mut done = false;
                    loop {
                        match toks[i] {
                            Token::Begin => cnt += 1,
                            Token::End   => {
                                cnt -= 1;
                                if cnt == 0 {
                                    done = true;
                                }
                            },
                            _ => {},
                        }
                        if done {
                            break;
                        }
                        i += 1;
                    }
                }
            },
            Token::End => {
                i = scopes.pop().unwrap() - 1;
            },
        }
        // println!("{}: stack at {}, value {}", i, pos, stack.get(pos).unwrap());
        i += 1;
        ticks += 1;
    }
    if debug {
        println!("took {} iterations", ticks);
    }
}

#[test]
fn test_hello() {
    let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    interpret(program);
}
