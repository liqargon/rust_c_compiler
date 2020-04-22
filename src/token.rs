pub enum TokenKind {
    TkAdd,
    TkSub,
    TkMul,
    TkDiv,
    TkPrSt,
    TkPrEd,
    TkEq,
    TkNEq,
    TkLt,
    TkGt,
    TkLe,
    TkGe,
    TkAssign,
    TkExprEnd,
}

pub enum Token {
    Operator {
        kind: TokenKind,
    },
    Number {
        val: i32,
    },
    Ident {
        name: String
    },
}


pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input = input;
    loop {
        if input.is_empty() {
            break;
        }
        tokenize_whitespace(&mut input);
        if let Some(i) = tokenize_number(&mut input) {
            tokens.push(Token::Number { val: i });
            continue;
        }
        if let Some(op) = tokenize_operator(&mut input) {
            tokens.push(Token::Operator { kind: op });
            continue;
        }
        if let Some(name) = tokenize_variant(&mut input) {
            tokens.push(Token::Ident { name });
            continue;
        }
    }
    tokens
}

fn tokenize_whitespace(x: &mut String) {
    loop {
        match x.chars().next() {
            Some(c) if c.is_whitespace() => {
                x.remove(0);
            }
            _ => {
                break;
            }
        }
    }
}

fn tokenize_number(x: &mut String) -> Option<i32> {
    let mut digits = "".to_string();
    loop {
        match x.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                digits += &c.to_string();
                x.remove(0);
            }
            _ => {
                break;
            }
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(digits.parse::<i32>().unwrap())
    }
}

fn tokenize_variant(x: &mut String) -> Option<String> {
    let mut s: String = "".to_string();
    loop {
        match x.chars().next() {
            Some(c) if 'a' <= c && c <= 'z' => {
                x.remove(0);
                s.push(c);
            }
            _ => {
                break;
            }
        }
    }
    if s.len() != 0 {
        Some(s)
    } else {
        None
    }
}

fn tokenize_operator(x: &mut String) -> Option<TokenKind> {
    if x.len() >= 2 {
        if x.starts_with("==") {
            x.drain(0..2);
            return Some(TokenKind::TkEq);
        }
        if x.starts_with("!=") {
            x.drain(0..2);
            return Some(TokenKind::TkNEq);
        }
        if x.starts_with("<=") {
            x.drain(0..2);
            return Some(TokenKind::TkLe);
        }
        if x.starts_with(">=") {
            x.drain(0..2);
            return Some(TokenKind::TkGe);
        }
    }

    match x.chars().next() {
        Some(c) => {
            match c {
                '+' => {
                    x.remove(0);
                    Some(TokenKind::TkAdd)
                }
                '-' => {
                    x.remove(0);
                    Some(TokenKind::TkSub)
                }
                '*' => {
                    x.remove(0);
                    Some(TokenKind::TkMul)
                }
                '/' => {
                    x.remove(0);
                    Some(TokenKind::TkDiv)
                }
                '(' => {
                    x.remove(0);
                    Some(TokenKind::TkPrSt)
                }
                ')' => {
                    x.remove(0);
                    Some(TokenKind::TkPrEd)
                }
                '<' => {
                    x.remove(0);
                    Some(TokenKind::TkLt)
                }
                '>' => {
                    x.remove(0);
                    Some(TokenKind::TkGt)
                }
                '=' => {
                    x.remove(0);
                    Some(TokenKind::TkAssign)
                }
                ';' => {
                    x.remove(0);
                    Some(TokenKind::TkExprEnd)
                }
                _ => None,
            }
        }
        None => {
            None
        }
    }
}

