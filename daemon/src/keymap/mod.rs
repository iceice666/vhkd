use std::collections::HashMap;

pub use crate::keymap::error::*;
pub use crate::keymap::keycode::*;
pub use crate::keymap::keys::*;
pub use crate::keymap::node::*;

mod error;
mod keycode;
mod keys;
mod node;

enum State<'x> {
    AtRoot {
        node: &'x ActionNode,
        cached: KeySequence,
    },
    AtLeaf(&'x ActionNode),
    Init,
}

struct KeymapStateMachine<'x> {
    state: State<'x>,
    keymap: HashMap<String, ActionNode>,
}

impl<'x, 'program: 'x> KeymapStateMachine<'x> {
    pub(crate) fn new() -> Self {
        let mut keymap = HashMap::new();
        keymap.insert("default".into(), ActionNode::default());

        KeymapStateMachine {
            state: State::Init,
            keymap,
        }
    }

    fn get_mode_mut(&mut self, mode: Option<String>) -> KeymapResult<&mut ActionNode> {
        let mode = mode.unwrap_or_else(|| "default".into());
        self.keymap
            .get_mut(&mode)
            .ok_or(KeymapError::NoSuchMode(mode))
    }

    // =================Public API=================

    /// Register a new key binding.
    pub fn register(&mut self, binding: KeyBinding, mode: Option<String>) -> KeymapResult<()> {
        self.get_mode_mut(mode)?.bind(binding);
        Ok(())
    }

    /// Remove a key binding.
    pub fn unregister(&mut self, sequences: KeySequence, mode: Option<String>) -> KeymapResult<()> {
        self.get_mode_mut(mode)?.unbind(sequences)
    }

    /// Reset current input progress and back to default mode.
    pub fn reset(&'program mut self) -> Option<KeySequence> {
        let node = self.keymap.get("default").unwrap();
        let ret = if let State::AtRoot { cached, .. } = &mut self.state {
            Some(std::mem::take(cached))
        } else {
            None
        };

        self.state = State::AtRoot {
            node,
            cached: KeySequence::new(),
        };
        ret
    }

    /// Switch to another mode.
    pub fn switch_mode(&'program mut self, mode: &str) -> KeymapResult<()> {
        self.keymap.get(mode).map_or_else(
            || Err(KeymapError::NoSuchMode(mode.into())),
            |node| {
                self.state = State::AtRoot {
                    node,
                    cached: KeySequence::new(),
                };
                Ok(())
            },
        )
    }

    /// Make an input and return the action if set.
    pub fn make_input(&'program mut self, key: KeySpec) -> KeymapResult<Option<KeyAction>> {
        if let State::AtRoot { node, cached } = &mut self.state {
            let next = node.get(&key)?;

            if next.is_leaf() {
                self.state = State::AtLeaf(node);
                Ok(Some(next.action.clone()))
            } else {
                cached.push(key);
                self.state = State::AtRoot {
                    node,
                    cached: cached.clone(),
                };
                Ok(None)
            }
        } else {
            unreachable!()
        }
    }
}