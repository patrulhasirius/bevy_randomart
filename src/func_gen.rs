use rand::{rngs::StdRng, Rng};

pub struct NodeBinop {
    lhs: Box<Node>,
    rhs: Box<Node>,
}

pub enum Node {
    X,
    Y,
    RANDOM,
    ADD(NodeBinop),
    MULT(NodeBinop),
}

pub fn eval(x: f32, y: f32, node: &Node, rng: &mut StdRng) -> f32 {
    match node {
        Node::X => x,
        Node::Y => y,
        Node::RANDOM => rng.gen_range(-1f32..1f32),
        Node::ADD(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref(), rng) + eval(x, y, node_binop.rhs.as_ref(), rng)
        }
        Node::MULT(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref(), rng) * eval(x, y, node_binop.rhs.as_ref(), rng)
        }
    }
}

pub fn generate_tree(depth: u32, rng: &mut StdRng) -> Node {
    let end = if depth == 0 { 3 } else { 5 };
    match rng.gen_range(1..=end) {
        1 => Node::X,
        2 => Node::Y,
        3 => Node::RANDOM,
        4 => Node::ADD(NodeBinop {
            lhs: Box::new(generate_tree(depth - 1, rng)),
            rhs: Box::new(generate_tree(depth - 1, rng)),
        }),
        5 => Node::MULT(NodeBinop {
            lhs: Box::new(generate_tree(depth - 1, rng)),
            rhs: Box::new(generate_tree(depth - 1, rng)),
        }),
        _ => panic!("at the disco"),
    }
}
