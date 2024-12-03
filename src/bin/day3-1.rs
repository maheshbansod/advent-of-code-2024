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

    let result = parser.parse();
    let result: u32 = result.operations.iter().map(|(a, b)| a * b).sum();
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
            }
        }
        None
    }

    fn expect_number(&mut self) -> Option<u32> {
        let mut n = String::new();
        let it_clone = self.chars.clone();
        let rest_num = it_clone.take_while(|c| c.is_digit(10)).collect::<String>();
        n.push_str(&rest_num);

        if n.len() == 0 {
            return None;
        }

        let len = n.len();
        for _ in 0..len {
            self.chars.next();
        }
        n.parse::<u32>().ok()
    }

    fn match_mul(&mut self) -> Option<Token> {
        let rest_code = "ul";
        let mut it_clone = self.chars.clone();
        let rest_chars = rest_code.chars();
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
        self.expect_next('(')?;
        let a = self.expect_number()?;
        self.expect_next(',')?;
        let b = self.expect_number()?;
        self.expect_next(')')?;
        return Some(Token::Mul(a, b));
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

enum Token {
    Mul(u32, u32),
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokenizer: Tokenizer<'a>) -> Self {
        Self { tokenizer }
    }

    fn parse(&mut self) -> ParseResult {
        let mut operations = vec![];
        while let Some(t) = self.tokenizer.next() {
            match t {
                Token::Mul(a, b) => {
                    operations.push((a, b));
                }
            }
        }
        ParseResult { operations }
    }
}

struct ParseResult {
    operations: Vec<(u32, u32)>,
}
