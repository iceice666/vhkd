use crate::keymap::{KeyBinding, KeySequence, KeymapResult};

pub trait Daemon<'x> {
    fn register(&mut self, binding: KeyBinding, mode: Option<String>);
    fn unregister(&mut self, sequences: KeySequence, mode: Option<String>) -> KeymapResult<()>;
    fn reset<'b: 'x>(&'b mut self);
    fn mode_change<'b: 'x>(&'b mut self, mode: String) -> KeymapResult<()>;
    fn run_daemon_mode( self);
    fn run_observe_mode(self);
}
