use std::{error::Error, fs, str::Chars};

fn main() -> Result<(), Box<dyn Error>> {
    let use_test = false;
    let file_name = if use_test {
        "inputs/day3-test"
    } else {
        "inputs/day3"
    };
    let input = fs::read_to_string(file_name)?;

    let tokenizer = Tokenizer::new(&input);
    let mut parser = Parser::new(tokenizer);

    let ParseResult { result } = parser.parse();
    println!("{result}");
    Ok(())
}

struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            chars: code.chars(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            if c == 'm' {
                let t = self.match_mul();
                if t.is_some() {
                    return t;
                }
            } else if c == 'd' {
                let t = self.match_conditional();
                if t.is_some() {
                    return t;
                }
            }
        }
        None
    }

    fn expect_number(&mut self) -> Option<u32> {
        let mut n = String::new();
        let it_clone = self.chars.clone();
        let rest_num = it_clone.take_while(|c| c.is_ascii_digit()).collect::<String>();
        n.push_str(&rest_num);

        if n.is_empty() {
            return None;
        }

        let len = n.len();
        for _ in 0..len {
            self.chars.next();
        }
        n.parse::<u32>().ok()
    }

    fn expect_string(&mut self, s: &str) -> Option<()> {
        let mut it_clone = self.chars.clone();
        let rest_chars = s.chars();
        for c in rest_chars {
            let it_next = it_clone.next();
            if let Some(it_next) = it_next {
                if it_next == c {
                    continue;
                }
            }
            return None;
        }
        self.chars = it_clone;
        Some(())
    }

    fn match_mul(&mut self) -> Option<Token> {
        self.expect_string("ul")?;
        self.expect_next('(')?;
        let a = self.expect_number()?;
        self.expect_next(',')?;
        let b = self.expect_number()?;
        self.expect_next(')')?;
        Some(Token::Mul(a, b))
    }

    fn match_conditional(&mut self) -> Option<Token> {
        if self.expect_string("on't()").is_some() {
            return Some(Token::Dont);
        } else if self.expect_string("o()").is_some() {
            return Some(Token::Do);
        }
        None
    }

    fn expect_next(&mut self, c: char) -> Option<()> {
        let mut it_clone = self.chars.clone();
        let next = it_clone.next()?;
        if next == c {
            self.chars = it_clone;
            return Some(());
        }
        None
    }
}

#[derive(Debug)]
enum Token {
    Mul(u32, u32),
    Do,
    Dont,
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    fn parse(&mut self) -> ParseResult {
        let mut result = 0;
        let mut active = true;
        while let Some(t) = self.tokenizer.next() {
            match t {
                Token::Mul(a, b) => {
                    if active {
                        result += a * b;
                    }
                }
                Token::Do => {
                    active = true;
                }
                Token::Dont => {
                    active = false;
                }
            }
        }
        ParseResult { result }
    }
}

struct ParseResult {
    result: u32,
}
