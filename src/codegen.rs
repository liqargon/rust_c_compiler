use crate::parse::*;

pub fn gen(node: Node) {
    if let Node::Number { val } = node {
        println!("  push {}", val);
        return;
    }
    if let Node::LVar {offset} = node {
        gen_lval(node);
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        return;
    }
    if let Node::Operator {kind : NodeKind::NdAssign, lhs,rhs} = node {
        gen_lval(*lhs);
        gen(*rhs);
        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        return;
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
            NodeKind::NdEq => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
            }
            NodeKind::NdNEq => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
            }
            NodeKind::NdLe | NodeKind::NdGe => {
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
            }
            NodeKind::NdLt | NodeKind::NdGt => {
                println!("  cmp rax, rdi");
                println!("  satl al");
                println!("  movzb rax, al");
            }
            _ => (),
        }
        println!("  push rax");
    }
}

fn gen_lval(node: Node) {
    if let Node::LVar {offset } = node {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");

    }else{
        return;

    }

}