use std::fmt::Display;
use rand::SeedableRng;
use rand::{rngs::StdRng, Rng};

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::classes::MeshInstance3D;

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct Mesh3DRandomShader {
    base: Base<MeshInstance3D>
}

use godot::classes::IMeshInstance3D;

#[godot_api]
impl IMeshInstance3D for Mesh3DRandomShader {
    fn init(base: Base<MeshInstance3D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            base,
        }
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
        
        format!("
        shader_type spatial;
		render_mode unshaded;

        float fix_fmod(float x, float y) {{
	        return sign(x) * (abs(x) - y * floor(abs(x) / y));
        }}

		void fragment()
		{{
			ALBEDO = vec3({}, {}, {});
		}}
        ", r_tree, g_tree, b_tree
            )
    }

}


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

fn generate_shader_code(node: &NodeKind) -> String {
    match node {
        NodeKind::X => "UV.x * 2.0 - 1.0".to_string(),
        NodeKind::Y => "UV.y * 2.0 - 1.0".to_string(),
        NodeKind::Random(r) => format!("float({})", *r),
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
                "fix_fmod({}, {})",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Gt(node_binop) => {
            format!(
                "float(({}) > ({}))",
                generate_shader_code(node_binop.lhs.as_ref()),
                generate_shader_code(node_binop.rhs.as_ref())
            )
        }
        NodeKind::Time => format!("sin(TIME)"),
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
        NodeState::A => match rng.random_range(1..=4) {
            1 => NodeKind::X,
            2 => NodeKind::Y,
            3 => NodeKind::Random(rng.random_range(-1f32..=1f32)),
            4 => NodeKind::Time,
            _ => unreachable!(),
        },
        NodeState::C => match rng.random_range(1..=7) {
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
