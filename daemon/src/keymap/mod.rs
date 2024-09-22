pub use crate::keymap::error::*;
pub use crate::keymap::keycode::*;
pub use crate::keymap::keys::*;
pub use crate::keymap::node::*;

mod error;
mod keycode;
mod keys;
mod node;

pub trait KeymapDaemon<'x, 'program: 'x> {
    /// Register a new keybinding
    fn register(&mut self, binding: KeyBinding, mode: Option<String>);
    /// Remove a keybinding
    fn unregister(&mut self, sequences: KeySequence, mode: Option<String>) -> KeymapResult<()>;
    /// Reset the input sequence
    fn reset(&'program mut self);
    /// Switch to another mode
    fn mode_change(&'program mut self, mode: String) -> KeymapResult<()>;
}
