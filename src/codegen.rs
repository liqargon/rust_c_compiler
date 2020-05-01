use crate::parse::*;

pub fn gen(node: Node) {
    if let Node::Number { val } = node {
        println!("  push {}", val);
        return;
    }
    if let Node::Return(rhs) = node {
        gen(*rhs);
        println!("  pop rax");
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
        return;
    }
    if let Node::If { cond, i_st, e_st } = node {
        gen(*cond);
        println!("  pop rax");
        println!("  cmp rax, 0");
        if let Node::Number { val } = *e_st {
            println!("  je  .Lend001");
            gen(*i_st);
            println!(".Lend001:");
            return;
        }
        println!("  je  .Lelse001");
        gen(*i_st);
        println!("  jmp .Lend001");
        println!(".Lelse001:");
        gen(*e_st);
        println!(".Lend001:");
        return;
    }
    if let Node::LVar { offset: _ } = node {
        gen_lval(node);
        println!("  pop rax");
        println!("  mov rax, [rax]");
        println!("  push rax");
        return;
    }
    if let Node::Operator { kind: NodeKind::NdAssign, lhs, rhs } = node {
        gen_lval(*lhs);
        gen(*rhs);
        println!("  pop rdi");
        println!("  pop rax");
        println!("  mov [rax], rdi");
        println!("  push rdi");
        return;
    }
    if let Node::Block(stmts) = node {
        for stmt in stmts {
            gen(stmt);
            println!("  pop rax");
        }
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
    if let Node::LVar { offset } = node {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
    } else {
        return;
    }
}