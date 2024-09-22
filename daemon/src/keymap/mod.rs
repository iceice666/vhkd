pub use crate::keymap::keycode::KeyCode;
pub use crate::keymap::keys::{KeyAction, KeyModifier, KeySpec};
pub use crate::keymap::node::ActionNode;
pub use crate::keymap::keys::KeyBinding;

mod keycode;
mod keys;
mod node;

pub fn new_keymap() -> ActionNode {
    ActionNode::new(KeySpec(Vec::new(), KeyCode::Null), KeyAction::Nop)
}
