use rand::{rngs::StdRng, Rng};

#[derive(Debug, Clone)]
pub struct NodeBinop {
    lhs: Box<NodeKind>,
    rhs: Box<NodeKind>,
}

enum NodeState {
    A,
    C,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    X,
    Y,
    Random(f32),
    Add(NodeBinop),
    Mult(NodeBinop),
    //NK_RULE,
    //NK_BOOLEAN,
    //NK_ADD,
    //NK_MULT,
    //NK_MOD,
    //NK_GT,
    //NK_IF,
}

pub fn eval(x: f32, y: f32, node: &NodeKind) -> f32 {
    match node {
        NodeKind::X => x,
        NodeKind::Y => y,
        NodeKind::Random(r) => *r,
        NodeKind::Add(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref()) + eval(x, y, node_binop.rhs.as_ref())
        }
        NodeKind::Mult(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref()) * eval(x, y, node_binop.rhs.as_ref())
        }
    }
}

pub fn generate_tree(depth: u32, rng: &mut StdRng) -> NodeKind {
    let state = match depth == 0 {
        true => NodeState::A,
        false => {
            if rng.gen_bool(1f64 / 4f64) {
                NodeState::A
            } else {
                NodeState::C
            }
        }
    };
    match state {
        NodeState::A => match rng.gen_range(1..=3) {
            1 => NodeKind::X,
            2 => NodeKind::Y,
            3 => NodeKind::Random(rng.gen_range(-1f32..=1f32)),
            _ => unreachable!(),
        },
        NodeState::C => match rng.gen_range(1..=2) {
            1 => NodeKind::Add(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            2 => NodeKind::Mult(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            _ => unreachable!(),
        },
    }
}
