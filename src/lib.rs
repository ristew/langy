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

fn putchar(c: i32) {
    print!("{}", c as u8 as char);
    io::stdout().flush().unwrap();
}


enum Token {
    Add(i32),
    Sub(i32),
    Gt(i32),
    Lt(i32),
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
    while i < program.len() {
        let c = program.chars().nth(i).unwrap();
        match c {
            '+' => {
                let mut pluses = 0;
                while program.chars().nth(i + pluses).unwrap() == '+' {
                    pluses += 1;
                }    
                i += pluses;
                i -= 1;
                toks.push(Token::Add(pluses as i32));
                
            }
            '-' => { 
                let mut minuses = 0;
                while program.chars().nth(i + minuses).unwrap() == '-' {
                    minuses += 1;
                }
                i += minuses;
                i -= 1;
                toks.push(Token::Sub(minuses as i32));
            },
            '>' => {
                let mut gts = 0;
                while program.chars().nth(i + gts).unwrap() == '>' {
                    gts += 1;
                }
                i += gts;
                i -= 1;
                toks.push(Token::Gt(gts as i32));
            },
            '<' => {
                let mut lts = 0;
                while program.chars().nth(i + lts).unwrap() == '<' {
                    lts += 1;
                }
                i += lts;
                i -= 1;
                toks.push(Token::Lt(lts as i32));
            },
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
        i += 1;
    }
    let mut ticks = 0;
    i = 0;
    while i < toks.len() {
        match toks[i] {
            Token::Add(n)  => {
                *stack.get_mut(pos).unwrap() += n;
            },
            Token::Sub(n) => {
                *stack.get_mut(pos).unwrap() -= n;
            },
            Token::Gt(n) => {
                pos += n as usize;
            },
            Token::Lt(n) => {
                pos -= n as usize;
            },
            Token::Out => {
                let c = *stack.get(pos).unwrap();
                putchar(c);
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
