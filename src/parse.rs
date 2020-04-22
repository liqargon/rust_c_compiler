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
    LVar {
        offset: i32,
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
    NdAssign,
    NdExprEnd,
}

struct LVar {
    offset: i32,
    name: String,
}

impl LVar {
    fn new(offset: i32, name: String) -> LVar {
        LVar { offset, name }
    }
}
fn find_duplication_lvar(name: String, locals: &mut Vec<LVar>) -> Option<i32> {
    for l in locals {
        if name == l.name {
            return Some(l.offset);
        }
    }
    None
}
fn find_lvar(name: String, locals: &mut Vec<LVar>) -> i32 {
    let dup = find_duplication_lvar(name.clone(), locals);
    match dup{
        Some(i) => {
            i
        }
        None => {
            let offset = (locals.len() as i32 + 1) * 8;
            let lvar = LVar::new(offset, name);
            let offset_lvar = lvar.offset;
            locals.push(lvar);
            offset_lvar
        }
    }
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
    fn new_node_lvar(val: i32) -> Box<Node> {
        let node = Node::LVar { offset: val };
        let node = Box::new(node);
        node
    }
}

pub fn program(tokens: &mut Vec<Token>) -> Vec<Node> {
    let mut lvars:Vec<LVar> = Vec::new();
    let mut code: Vec<Node> = Vec::new();
    loop {
        if tokens.len() == 0 {
            break;
        }
        code.push(stmt(tokens, &mut lvars));
    }
    code
}

fn stmt(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let node = expr(tokens, lvars);
    if let Token::Operator { kind: TokenKind::TkExprEnd } = &tokens[0] {
        tokens.remove(0);
    }

    node
}


fn expr(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let node = assign(tokens, lvars);

    node
}

fn assign(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let node = equality(tokens, lvars);
    match &tokens[0] {
        Token::Operator { kind: TokenKind::TkAssign } => {
            tokens.remove(0);
            *Node::new(NodeKind::NdAssign, Box::new(node), Box::new(assign(tokens, lvars)))
        }
        _ => node,
    }
}

fn equality(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let mut node = relational(tokens, lvars);

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
                    Box::new(relational(tokens, lvars)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkNEq } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdNEq,
                    Box::new(node),
                    Box::new(relational(tokens, lvars)),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}


fn relational(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let mut node = add(tokens, lvars);

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
                    Box::new(add(tokens, lvars)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkLe } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdLe,
                    Box::new(node),
                    Box::new(add(tokens, lvars)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkGt } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdGt,
                    Box::new(add(tokens, lvars)),
                    Box::new(node),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkGe } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdGe,
                    Box::new(add(tokens, lvars)),
                    Box::new(node),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn add(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let mut node = mul(tokens, lvars);

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
                    Box::new(mul(tokens, lvars)),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkSub } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdSub,
                    Box::new(node),
                    Box::new(mul(tokens, lvars)),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn mul(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Node {
    let mut node = unary(tokens, lvars).unwrap();

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
                    Box::new(unary(tokens, lvars).unwrap()),
                );
                node = node_i;
            }
            Token::Operator { kind: TokenKind::TkDiv } => {
                tokens.remove(0);
                let node_i = *Node::new(
                    NodeKind::NdDiv,
                    Box::new(node),
                    Box::new(unary(tokens, lvars).unwrap()),
                );
                node = node_i;
            }
            _ => break,
        }
    }
    node
}

fn unary(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Option<Node> {
    match &tokens[0] {
        Token::Operator { kind: TokenKind::TkAdd } => {
            tokens.remove(0);
            primary(tokens, lvars)
        }
        Token::Operator { kind: TokenKind::TkSub } => {
            tokens.remove(0);
            Some(*Node::new(NodeKind::NdSub, Node::new_node_num(0), Box::new(primary(tokens, lvars).unwrap())))
        }
        _ => primary(tokens, lvars),
    }
}

fn primary(tokens: &mut Vec<Token>, lvars: &mut Vec<LVar>) -> Option<Node> {
    let first_token = (&tokens).first().unwrap().clone();
    match first_token {
        Token::Operator { kind: TokenKind::TkPrSt } => {
            tokens.remove(0);
            let node = expr(tokens, lvars);
            if let Token::Operator { kind: TokenKind::TkPrEd } = &tokens[0] {
                tokens.remove(0);
            }
            Some(node)
        }
        Token::Ident { name } => {
            let offset = find_lvar(name.to_string(), lvars);
            tokens.remove(0);
            Some(*Node::new_node_lvar(offset))
        }
        Token::Number { val } => {
            let v = *val;
            tokens.remove(0);
            Some(*Node::new_node_num(v))
        }
        _ => {
            None
        }
    }
}
