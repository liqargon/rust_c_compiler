use crate::token::*;

pub enum Node {
    Operator {
        kind: NodeKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Number {
        val: i32,
    },
}

pub enum NodeKind {
    NdAdd,
    NdSub,
    NdMul,
    NdDiv,
    NdEq,
    NdNEq,
    NdLt,
    NdGt,
    NdLe,
    NdGe,
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

pub fn expr(tokens: &mut Vec<Token>) -> Node {
    let node = equality(tokens);

    node
}

fn equality(tokens: &mut Vec<Token>) -> Node {
    let mut node = relational(tokens);

    loop {
        if tokens.len() == 0 {
            break;
        }
        match &tokens[0] {
            Token::Operator { kind: TokenKind::TkEq } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdEq,
                    Box::new(node),
                    Box::new(relational(tokens)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkNEq } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdNEq,
                    Box::new(node),
                    Box::new(relational(tokens)),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}


fn relational(tokens: &mut Vec<Token>) -> Node {
    let mut node = add(tokens);

    loop {
        if tokens.len() == 0 {
            break;
        }
        match &tokens[0] {
            Token::Operator { kind: TokenKind::TkLt } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdLt,
                    Box::new(node),
                    Box::new(add(tokens)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkLe } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdLe,
                    Box::new(node),
                    Box::new(add(tokens)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkGt } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdGt,
                    Box::new(add(tokens)),
                    Box::new(node),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkGe } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdGe,
                    Box::new(add(tokens)),
                    Box::new(node),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn add(tokens: &mut Vec<Token>) -> Node {
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

fn unary(tokens: &mut Vec<Token>) -> Option<Node> {
    match &tokens[0] {
        Token::Operator { kind: TokenKind::TkAdd } => {
            tokens.remove(0);
            primary(tokens)
        }
        Token::Operator { kind: TokenKind::TkSub } => {
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
