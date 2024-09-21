pub use crate::keymap::keycode::KeyCode;
pub use crate::keymap::keys::{KeyAction, KeyModifier, KeySpec};

mod keycode;
mod keys;
mod node;

pub fn new_keymap() -> node::ActionNode {
    node::ActionNode::new(KeySpec(Vec::new(), KeyCode::Null), KeyAction::Nop)
}
