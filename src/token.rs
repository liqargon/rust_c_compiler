pub enum OperatorKind {
    TkAdd,
    TkSub,
    TkMul,
    TkDiv,
    TkPrSt,
    TkPrEd,
    TkBrSt,
    TkBrEd,
    TkEq,
    TkNEq,
    TkLt,
    TkGt,
    TkLe,
    TkGe,
    TkAssign,
    TkExprEnd,
}

pub enum KeywordKind {
    TkReturn,
    TkIf,
    TkElse,
    TkWhile,
    TkFor,
}

pub enum Token {
    Operator {
        kind: OperatorKind,
    },
    Number {
        val: i32,
    },
    Ident {
        name: String
    },
    Return,
    If,
    Else,
    While,
    For,
}


pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input = input;
    loop {
        if input.is_empty() {
            break;
        }
        tokenize_whitespace(&mut input);
        if let Some(kw) = tokenize_keyword(&mut input) {
            match kw {
                KeywordKind::TkReturn => tokens.push(Token::Return),
                KeywordKind::TkIf => tokens.push(Token::If),
                KeywordKind::TkElse => tokens.push(Token::Else),
                _ => {}
            }
            continue;
        }
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

fn tokenize_keyword(x: &mut String) -> Option<KeywordKind> {
    if x.starts_with("return ") {
        x.drain(0..7);
        return Some(KeywordKind::TkReturn);
    }
    if x.starts_with("if(") {
        x.drain(0..2);
        return Some(KeywordKind::TkIf);
    }
    if x.starts_with("else ") {
        x.drain(0..4);
        return Some(KeywordKind::TkElse);
    }
    None
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

fn tokenize_operator(x: &mut String) -> Option<OperatorKind> {
    if x.len() >= 2 {
        if x.starts_with("==") {
            x.drain(0..2);
            return Some(OperatorKind::TkEq);
        }
        if x.starts_with("!=") {
            x.drain(0..2);
            return Some(OperatorKind::TkNEq);
        }
        if x.starts_with("<=") {
            x.drain(0..2);
            return Some(OperatorKind::TkLe);
        }
        if x.starts_with(">=") {
            x.drain(0..2);
            return Some(OperatorKind::TkGe);
        }
    }

    match x.chars().next() {
        Some(c) => {
            match c {
                '+' => {
                    x.remove(0);
                    Some(OperatorKind::TkAdd)
                }
                '-' => {
                    x.remove(0);
                    Some(OperatorKind::TkSub)
                }
                '*' => {
                    x.remove(0);
                    Some(OperatorKind::TkMul)
                }
                '/' => {
                    x.remove(0);
                    Some(OperatorKind::TkDiv)
                }
                '(' => {
                    x.remove(0);
                    Some(OperatorKind::TkPrSt)
                }
                ')' => {
                    x.remove(0);
                    Some(OperatorKind::TkPrEd)
                }
                '{' => {
                    x.remove(0);
                    Some(OperatorKind::TkBrSt)
                }
                '}' => {
                    x.remove(0);
                    Some(OperatorKind::TkBrEd)
                }
                '<' => {
                    x.remove(0);
                    Some(OperatorKind::TkLt)
                }
                '>' => {
                    x.remove(0);
                    Some(OperatorKind::TkGt)
                }
                '=' => {
                    x.remove(0);
                    Some(OperatorKind::TkAssign)
                }
                ';' => {
                    x.remove(0);
                    Some(OperatorKind::TkExprEnd)
                }
                _ => None,
            }
        }
        None => {
            None
        }
    }
}

