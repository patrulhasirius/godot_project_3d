use rand::seq::IteratorRandom;
use rand::SeedableRng;
use rand::{Rng, rngs::StdRng};
use strum::{EnumIter, IntoEnumIterator};
use std::fmt::Display;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::classes::MeshInstance3D;

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct Mesh3DRandomShader {
    base: Base<MeshInstance3D>,
}

use godot::classes::IMeshInstance3D;

#[godot_api]
impl IMeshInstance3D for Mesh3DRandomShader {
    fn init(base: Base<MeshInstance3D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { base }
    }
}

#[godot_api]
impl Mesh3DRandomShader {
    #[func]
    fn new_shader_code(seed: u64, depth: u32) -> String {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let r_tree = generate_tree(depth, &mut rng);
        let g_tree = generate_tree(depth, &mut rng);
        let b_tree = generate_tree(depth, &mut rng);

        format!(
            "
        shader_type spatial;
		render_mode unshaded;

        float fix_fmod(float x, float y) {{
	        return sign(x) * (abs(x) - y * floor(abs(x) / y));
        }}

		void fragment()
		{{
			ALBEDO = vec3({}, {}, {});
		}}
        ",
            r_tree, g_tree, b_tree
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct NodeBinop {
    lhs: Box<NodeKind>,
    rhs: Box<NodeKind>,
}

#[derive(Debug, Clone, Default)]
pub struct NodeUnop {
    value: Box<NodeKind>,
}

enum NodeState {
    A,
    C,
}

#[derive(Debug, Clone)]
enum NodeKind {
    NodeTerminal(NodeTerminal),
    NodeNonTerminal(NodeNonTerminal),
}

impl Default for NodeKind {
    fn default() -> Self { NodeKind::NodeTerminal(NodeTerminal::X) }
}

#[derive(Debug, EnumIter, Copy, Clone)]
enum NodeTerminal {
    X,
    Y,
    Random(f32),
    Time,
}

#[derive(Debug, EnumIter, Clone)]
enum NodeNonTerminal {
    Add(NodeBinop),
    Mult(NodeBinop),
    Sqrt(NodeUnop),
    Abs(NodeUnop),
    Sin(NodeUnop),
    Mod(NodeBinop),
    Gt(NodeBinop),
}
impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = generate_shader_code(self);

        write!(f, "{}", string)
    }
}

fn generate_shader_code(node: &NodeKind) -> String {
    match node {
        NodeKind::NodeTerminal(nodetype) => match nodetype {
            NodeTerminal::X => "UV.x * 2.0 - 1.0".to_string(),
            NodeTerminal::Y => "UV.y * 2.0 - 1.0".to_string(),
            NodeTerminal::Random(r) => format!("float({})", *r),
            NodeTerminal::Time => "sin(TIME)".to_string(),
            
        }
        NodeKind::NodeNonTerminal(nodetype) => match nodetype {
        NodeNonTerminal::Add(node_binop) => {
            format!(
                "({}) + ({})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeNonTerminal::Mult(node_binop) => {
            format!(
                "({}) * ({})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeNonTerminal::Sqrt(node_unop) => {
            format!(
                "sqrt(abs({}))",
                generate_shader_code(node_unop.value.as_ref())
            )
        }
        NodeNonTerminal::Abs(node_unop) => {
            format!("abs({})", generate_shader_code(node_unop.value.as_ref()))
        }
        NodeNonTerminal::Sin(node_unop) => {
            format!("sin({})", generate_shader_code(node_unop.value.as_ref()))
        }
        NodeNonTerminal::Mod(node_binop) => {
            format!(
                "fix_fmod({}, {})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeNonTerminal::Gt(node_binop) => {
            format!(
                "float(({}) > ({}))",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }}
    }
}

fn generate_tree(depth: u32, rng: &mut StdRng) -> NodeKind {
    let state = match depth == 0 {
        true => NodeState::A,
        false => {
            if rng.random_bool(1f64 / 4f64) {
                NodeState::A
            } else {
                NodeState::C
            }
        }
    };

    match state {
        NodeState::A => match NodeTerminal::iter().choose(rng).unwrap() {
            NodeTerminal::X => NodeKind::NodeTerminal(NodeTerminal::X),
            NodeTerminal::Y => NodeKind::NodeTerminal(NodeTerminal::Y),
            NodeTerminal::Time => NodeKind::NodeTerminal(NodeTerminal::Time),
            NodeTerminal::Random(_) => NodeKind::NodeTerminal(NodeTerminal::Random(rng.random_range(-1f32..=1f32))),
        },
        NodeState::C => match NodeNonTerminal::iter().choose(rng).unwrap() {
            NodeNonTerminal::Add(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Add(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Mult(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Mult(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Mod(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Mod(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Gt(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Gt(NodeBinop {
                lhs: Box::new(generate_tree(depth - 1, rng)),
                rhs: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Sqrt(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Sqrt(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Abs(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Abs(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            })),
            NodeNonTerminal::Sin(_) => NodeKind::NodeNonTerminal(NodeNonTerminal::Sin(NodeUnop {
                value: Box::new(generate_tree(depth - 1, rng)),
            })),
        },
    }
}
