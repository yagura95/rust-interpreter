use std::env;
use std::io::Read;
use std::process;
use std::fs::File;
use std::collections::HashMap;

#[derive(std::fmt::Debug, std::clone::Clone)]
enum TokenType {
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    
    Identifier,
    String,
    Number,
    
    And,
    Class,
    Else,
    For,
    If,
    Nil,
    Or, 
    Print,
    Return,
    Super,
    This,
    True,
    False,
    Var,
    While,

    Eof
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u64,
}

impl Token {
    fn new(token_type: TokenType, lexeme: &str, literal: &str, line: u64) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal: String::from(literal),
            line,
        }
    }
} 

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "Token type: {:?}\nLexeme: {}\nLiteral: {}\nLine: {}", self.token_type, self.lexeme, self.literal, self.line) 
        write!(f, "Token type: {:?}, {}, {}, {}", self.token_type, self.lexeme, self.literal, self.line) 
    }
}

fn args_validation(args: env::Args) -> String {
    if args.len() != 2 {
        println!("Usage: cargo run <file_name>");
        process::exit(1);
    }

    let mut a: Vec<String> = vec![];
    for arg in args {
        a.push(arg);
    }

    a[1].clone()
}

fn main() {
    let file = args_validation(env::args());

    println!("Opening file: {}", file);

    let fd = File::open(file);

    let mut fd = match fd {
        Ok(file) => file,
        Err(err) => {
            println!("Failed opening file: {}", err);
            process::exit(1);
        }
    };

    let mut content = String::new();
    let char_count = fd.read_to_string(&mut content);
    
    println!("File is {} chars", char_count.unwrap());

    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("and",    TokenType::And), 
        ("class",  TokenType::Class), 
        ("else",   TokenType::Else), 
        ("false",  TokenType::False),
        ("for",    TokenType::For),
        ("if",     TokenType::If),
        ("nil",    TokenType::Nil),
        ("or",     TokenType::Or),
        ("print",  TokenType::Print),
        ("return", TokenType::Return),
        ("super",  TokenType::Super),
        ("this",   TokenType::This),
        ("true",   TokenType::True),
        ("var",    TokenType::Var),
        ("while",  TokenType::While),
        ]);    

    let mut tokens: Vec<Token> = vec![];
    let mut errors: Vec<String> = vec![];

    let mut chars = content.chars().peekable();

    let mut line = 1;
 
    while let Some(c) = chars.next() {
        match c.to_string().as_str() {
            "\n" => {
                line = line + 1;
            },
            "\t" | "\r" | " " => {},
            "{" => {
                tokens.push(Token::new(TokenType::LeftBrace, "{", "", line))           
            },
            "}" => {
                tokens.push(Token::new(TokenType::RightBrace, "}", "", line))           
            },
            "(" => {
                tokens.push(Token::new(TokenType::LeftParen, "(", "", line))           
            },
            ")" => {
                tokens.push(Token::new(TokenType::RightParen, ")", "", line))           
            },
            "," => {
                tokens.push(Token::new(TokenType::Comma, ",", "", line))           
            },
            "." => {
                tokens.push(Token::new(TokenType::Dot, ".", "", line))           
            },
            "-" => {
                tokens.push(Token::new(TokenType::Minus, "-", "", line))           
            },
            "+" => {
                tokens.push(Token::new(TokenType::Plus, "+", "", line))           
            },
            ";" => {
                tokens.push(Token::new(TokenType::Semicolon, ";", "", line))           
            },
            "*" => {
                tokens.push(Token::new(TokenType::Star, "*", "", line))           
            },
            "!" => {
                let next = chars.peek().unwrap();

                if next.to_string().as_str() == "=" {
                    chars.next();
                    tokens.push(Token::new(TokenType::BangEqual, "!=", "", line));
                } else {
                    tokens.push(Token::new(TokenType::Bang, "!", "", line));
                } 
            },
            "=" => {
                let next = chars.peek().unwrap();

                if next.to_string().as_str() == "=" {
                    tokens.push(Token::new(TokenType::EqualEqual, "==", "", line));
                    chars.next();
                } else {
                    tokens.push(Token::new(TokenType::Equal, "=", "", line))           
                } 
            },
            "<" => {
                let next = chars.peek().unwrap();

                if next.to_string().as_str() == "=" {
                    tokens.push(Token::new(TokenType::LessEqual, "<=", "", line));
                    chars.next();
                } else {
                    tokens.push(Token::new(TokenType::Less, "<", "", line))           
                }             
            },
            ">" => {
                let next = chars.peek().unwrap();

                if next.to_string().as_str() == "=" {
                    chars.next();
                    tokens.push(Token::new(TokenType::GreaterEqual, ">=", "", line));
                } else {
                    tokens.push(Token::new(TokenType::Greater, ">", "", line))           
                }      
            },
            "/" => {
                let next = chars.peek().unwrap();

                if next.to_string().as_str() == "/" {
                    // TODO: It is a comment 
                    //       ignore until the end of the line 
                    //       consume chars until "\n" is found 
                    chars.next();
                    while let Some(i) = chars.next() {
                        match i.to_string().as_str() {
                            "\n" => {
                                line = line + 1;
                                break;
                            },
                            _ => {},
                        }
                    }
                } else if next.to_string().as_str() == "*" {
                    chars.next();
                    while let Some(i) = chars.next() {
                        match i.to_string().as_str() { 
                            "\n" => {
                                line = line + 1;
                            },
                            "*" => {
                                let a = chars.peek().unwrap();
                                let a = a.clone().to_string();
                                let a = a.as_str();

                                if a == "/" {
                                    chars.next();
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                } else {
                    tokens.push(Token::new(TokenType::Slash, "/", "", line))           
                }             
            },
            "\"" => {
                let mut s = String::new();
                while let Some(t) = chars.next() {
                    // NOTE: does not handle " chars inside string literals yet
                    match t.to_string().as_str() {
                        "\"" => {
                            tokens.push(Token::new(TokenType::String, "", s.as_str(), line));
                            break;
                        },
                        _ => {
                            s.push_str(t.to_string().as_str());
                        }
                    }
                }
                
            },
            _ => {
                // NOTE: identifiers must start with alphabetic chars
                if c.is_ascii_digit() {
                    let mut floating = false;
                    let mut n = String::new();

                    n.push_str(c.to_string().as_str());

                    while let Some(d) = chars.next() {
                        if d.to_string().as_str() == "." && floating {
                            errors.push(format!("Multiple '.' chars in number"));
                        } else if d.to_string().as_str() == "." && n.len() < 1 {
                            errors.push(format!("Number must with a digit"));
                        } else if d.to_string().as_str() == "." {
                            n.push_str(d.to_string().as_str());
                            floating = true;
                        } else if d.is_ascii_digit() {
                            n.push_str(d.to_string().as_str());
                        } else {
                            break;
                        }
                    }

                    tokens.push(Token::new(TokenType::Number, "", n.as_str(), line));
                         
                } else if c.is_ascii_alphabetic() {
                    let mut s = String::new();

                    s.push_str(c.to_string().as_str());

                    while let Some(c) = chars.next() {
                        if c.is_alphanumeric() {
                            s.push_str(c.to_string().as_str());
                        } else {
                            break
                        }
                    }

                    let k = keywords.get(s.as_str());
                    
                    match k {
                        None => {
                            tokens.push(Token::new(TokenType::Identifier, "", s.as_str(), line));
                        },
                        Some(t) => {
                            tokens.push(Token::new(t.clone(), "", s.as_str(), line));

                        }
                    }

                } else {
                    errors.push(format!("Unexpected char '{}' at line {}", c, line))    
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "EOF", "", line));

    println!("File is {} lines", line - 1);

    if errors.len() > 0 {
        for e in errors {
            println!("{}", e);
        }
        process::exit(1)
    }

    for t in tokens {
        println!("{}", t);
    }


}
