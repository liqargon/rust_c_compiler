use std::env;

enum TokenKind {
    TkAdd,
    TkSub,
    TkMul,
    TkDiv,
    TkPrSt,
    TkPrEd,
}

enum Token {
    Operator {
        kind: TokenKind,
    },
    Number {
        val: i32,
    },
}

enum Node {
    Operator {
        kind: NodeKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Number {
        val: i32,
    },
}

enum NodeKind {
    NdAdd,
    NdSub,
    NdMul,
    NdDiv,
}

impl Node {
    fn new(kind: NodeKind, lhs: Box<Node>, rhs: Box<Node>) -> Box<Node> {
        let node = Node::Operator {
            kind: kind,
            lhs: lhs,
            rhs: rhs,
        };
        let node = Box::new(node);
        node
    }

    fn new_node_num(val: i32) -> Box<Node> {
        let node = Node::Number { val: val };
        let node = Box::new(node);
        node
    }
}


fn tokenize(input: String) -> Vec<Token> {
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

fn tokenize_operator(x: &mut String) -> Option<TokenKind> {
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
                _ => None,
            }
        }
        None => {
            None
        }
    }
}

fn expr(tokens: &mut Vec<Token>) -> Node {
    let mut node = mul(tokens);

    loop {
        if tokens.len() == 0 {
            break;
        }
        match &tokens[0] {
            Token::Operator { kind: TokenKind::TkAdd } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdAdd,
                    Box::new(node),
                    Box::new(mul(tokens)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkSub } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdSub,
                    Box::new(node),
                    Box::new(mul(tokens)),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn mul(tokens: &mut Vec<Token>) -> Node {
    let mut node = unary(tokens).unwrap();

    loop {
        if tokens.len() == 0 {
            break;
        }
        match &tokens[0] {
            Token::Operator { kind: TokenKind::TkMul } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdMul,
                    Box::new(node),
                    Box::new(unary(tokens).unwrap()),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkDiv } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdDiv,
                    Box::new(node),
                    Box::new(unary(tokens).unwrap()),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn unary(tokens: &mut Vec<Token>) -> Option<Node>{
    match &tokens[0] {
        Token::Operator {kind: TokenKind::TkAdd} => {
            tokens.remove(0);
            primary(tokens)
        }
        Token::Operator {kind: TokenKind::TkSub} => {
            tokens.remove(0);
            Some(*Node::new(NodeKind::NdSub, Node::new_node_num(0), Box::new(primary(tokens).unwrap())))
        }
        _ => primary(tokens),
    }
}

fn primary(tokens: &mut Vec<Token>) -> Option<Node> {
    match &tokens[0] {
        Token::Operator { kind: TokenKind::TkPrSt } => {
            tokens.remove(0);
            let node = expr(tokens);
            if let Token::Operator { kind: TokenKind::TkPrEd } = &tokens[0] {
                tokens.remove(0);
            }
            Some(node)
        }
        _ => {
            if let Token::Number { val } = tokens[0] {
                tokens.remove(0);
                Some(*Node::new_node_num(val))
            } else {
                None
            }
        }
    }
}

fn gen(node: Node) {
    if let Node::Number { val } = node {
        println!("  push {}", val)
    }
    if let Node::Operator { kind, lhs, rhs } = node {
        gen(*lhs);
        gen(*rhs);
        println!("  pop rdi");
        println!("  pop rax");
        match kind {
            NodeKind::NdAdd => {
                println!("  add rax, rdi");
            }
            NodeKind::NdSub => {
                println!("  sub rax, rdi");
            }
            NodeKind::NdMul => {
                println!("  imul rax, rdi");
            }
            NodeKind::NdDiv => {
                println!("  cqo");
                println!("  idiv rdi");
            }
        }
        println!("  push rax");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    let s = args[1].clone();
//    let s = "-30*(12-11) -2 + 2".to_string();
    let mut v = tokenize(s);

    let nodes = expr(&mut v);
    gen(nodes);

    println!("  pop rax");
    println!("  ret");
}
