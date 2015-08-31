use std::env;
use std::io;
use std::io::{Read, Write};

enum Token {
    ADD,
    SUB,
    GT,
    LT,
    OUT,
    IN,
    BEGIN,
    END,
}

fn main() {
    let mut arguments = env::args();
    arguments.next();
    match arguments.next() {
        Some(fname) => interpret(read_file(fname)),
        None => println!("no file"),
    }
}

fn interpret(program: String) {
    let mut stack: Vec<i32> = vec![0; 1000];
    let mut scopes: Vec<usize> = Vec::new();
    let mut pos = 0;
    let mut toks: Vec<Token> = Vec::new();
    let mut i = 0;
    for c in program.chars() {
        match c {
            '+' => toks.push(Token::ADD),
            '-' => toks.push(Token::SUB),
            '>' => toks.push(Token::GT),
            '<' => toks.push(Token::LT),
            '.' => toks.push(Token::OUT),
            ',' => toks.push(Token::IN),
            '[' => {
                toks.push(Token::BEGIN);
            },
            ']' => {
                toks.push(Token::END);
            },
            _ => {},
        }
    }
    while i < toks.len() {
        match toks[i] {
            Token::ADD  => {
                *stack.get_mut(pos).unwrap() += 1;
            },
            Token::SUB => {
                *stack.get_mut(pos).unwrap() -= 1;
            },
            Token::GT => {
                pos += 1;
            },
            Token::LT => {
                pos -= 1;
            },
            Token::OUT => {
                let c = *stack.get(pos).unwrap();
                print!("{}", c as u8 as char);
                io::stdout().flush().unwrap();
            },
            Token::IN => {
                let inp = io::stdin().bytes().next();
                match inp {
                    Some(n) => *stack.get_mut(pos).unwrap() = n.unwrap() as u8 as i32,
                    None => println!("none"),
                }
                // *stack.get_mut(pos).unwrap() = io::stdio::stdin_raw().unwrap().read_u8().ok().unwrap_or(0);
            },
            Token::BEGIN => {
                if *stack.get(pos).unwrap() != 0 {
                    scopes.push(i);
                } else {
                    let mut cnt = 0;
                    let mut done = false;
                    loop {
                        match toks[i] {
                            Token::BEGIN => cnt += 1,
                            Token::END   => {
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
            Token::END => {
                i = scopes.pop().unwrap() - 1;
            },
        }
        //println!("{}: stack at {}, value {}", i, pos, stack.get(pos).unwrap());
        i += 1;
    }
}

fn read_file(fname: String) -> String {
    use std::io::prelude::*;
    use std::fs::File;
    let mut f = File::open(fname).unwrap();
    let mut res = String::new();
    f.read_to_string(&mut res).unwrap();
    res
}
