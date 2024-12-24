use std::fmt::{write, Display};

use rand::{rngs::StdRng, Rng};

use crate::state::RenderState;

#[derive(Debug, Clone)]
pub struct NodeBinop {
    lhs: Box<NodeKind>,
    rhs: Box<NodeKind>,
}

#[derive(Debug, Clone)]
pub struct NodeUnop {
    value: Box<NodeKind>,
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
    Sqrt(NodeUnop),
    Abs(NodeUnop),
    Sin(NodeUnop),
    Mod(NodeBinop),
    Gt(NodeBinop),
    Time,
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = generate_shader_code(self);

        write!(f, "{}", string)
    }
}

pub fn generate_shader_code(node: &NodeKind) -> String {
    match node {
        NodeKind::X => "mesh.uv.x * 2.0 - 1.0".to_string(),
        NodeKind::Y => "mesh.uv.y * 2.0 - 1.0".to_string(),
        NodeKind::Random(r) => format!("f32({})", *r),
        NodeKind::Add(node_binop) => {
            format!(
                "({}) + ({})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Mult(node_binop) => {
            format!(
                "({}) * ({})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Sqrt(node_unop) => {
            format!(
                "sqrt(abs({}))",
                generate_shader_code(node_unop.value.as_ref())
            )
        }
        NodeKind::Abs(node_unop) => {
            format!("abs({})", generate_shader_code(node_unop.value.as_ref()))
        }
        NodeKind::Sin(node_unop) => {
            format!("sin({})", generate_shader_code(node_unop.value.as_ref()))
        }
        NodeKind::Mod(node_binop) => {
            format!(
                "({}) % ({})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Gt(node_binop) => {
            format!(
                "f32(({}) > ({}))",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Time => format!("sin(globals.time)"),
    }
}

pub fn eval(x: f32, y: f32, node: &NodeKind, time: f32) -> f32 {
    match node {
        NodeKind::X => x,
        NodeKind::Y => y,
        NodeKind::Random(r) => *r,
        NodeKind::Add(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref(), time) + eval(x, y, node_binop.rhs.as_ref(), time)
        }
        NodeKind::Mult(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref(), time) * eval(x, y, node_binop.rhs.as_ref(), time)
        }
        NodeKind::Sqrt(node_unop) => eval(x, y, node_unop.value.as_ref(), time).abs().sqrt(),
        NodeKind::Abs(node_unop) => eval(x, y, node_unop.value.as_ref(), time).abs(),
        NodeKind::Sin(node_unop) => eval(x, y, node_unop.value.as_ref(), time).sin(),
        NodeKind::Mod(node_binop) => {
            eval(x, y, node_binop.lhs.as_ref(), time) % eval(x, y, node_binop.rhs.as_ref(), time)
        }
        NodeKind::Gt(node_binop) => {
            (eval(x, y, node_binop.lhs.as_ref(), time) > eval(x, y, node_binop.rhs.as_ref(), time))
                as i32 as f32
        }
        NodeKind::Time => time.sin(),
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
        NodeState::A => match rng.gen_range(1..=4) {
            1 => NodeKind::X,
            2 => NodeKind::Y,
            3 => NodeKind::Random(rng.gen_range(-1f32..=1f32)),
            4 => NodeKind::Time,
            _ => unreachable!(),
        },
        NodeState::C => match rng.gen_range(1..=7) {
            1 => NodeKind::Add(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            2 => NodeKind::Mult(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            3 => NodeKind::Sqrt(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            }),
            4 => NodeKind::Abs(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            }),
            5 => NodeKind::Sin(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            }),
            6 => NodeKind::Mod(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            7 => NodeKind::Gt(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            }),
            _ => unreachable!(),
        },
    }
}
