use std::collections::HashMap;

use super::{
    keys::KeyBinding, KeyAction, KeyCode, KeySequence, KeySpec, KeymapError, KeymapResult,
};

#[derive(Debug)]
pub struct ActionNode {
    pub key: KeySpec,
    pub action: KeyAction,
    pub(crate) children: Vec<ActionNode>,
}

impl Default for ActionNode {
    fn default() -> Self {
        Self {
            key: KeySpec(Vec::new(), KeyCode::Null),
            action: KeyAction::Nop,
            children: Vec::new(),
        }
    }
}

impl ActionNode {
    pub const fn new(key: KeySpec, action: KeyAction) -> Self {
        Self {
            key,
            action,
            children: Vec::new(),
        }
    }

    pub fn get(&self, key: &KeySpec) -> KeymapResult<&ActionNode> {
        self.children
            .iter()
            .find(|node| &node.key == key)
            .ok_or(KeymapError::KeyNotFound(
                KeySequence::default(),
                key.to_string(),
            ))
    }

    fn get_mut(&mut self, key: &KeySpec) -> KeymapResult<&mut ActionNode> {
        self.children
            .iter_mut()
            .find(|node| &node.key == key)
            .ok_or(KeymapError::KeyNotFound(
                KeySequence::default(),
                key.to_string(),
            ))
    }

    fn get_or_insert(&mut self, key: KeySpec, action: KeyAction) -> &mut ActionNode {
        // Here comes a unsafe workaround
        let this = self as *const ActionNode;
        let this = unsafe { &mut *(this as *mut ActionNode) };

        this.children
            .iter_mut()
            .find(|node| node.key == key)
            .unwrap_or_else(|| {
                let node = ActionNode::new(key, action);
                self.children.push(node);
                self.children.last_mut().unwrap()
            })
    }

    pub fn bind(&mut self, binding: KeyBinding) {
        let KeyBinding {
            mut sequences,
            action,
        } = binding;
        let mut node = self;
        let target_key = sequences.pop().unwrap();

        for key in sequences {
            node = node.get_or_insert(key, KeyAction::Nop);
        }
        node.children.push(ActionNode::new(target_key, action));
    }

    // Remove a binding from the keymap
    pub fn unbind(&mut self, mut sequences: KeySequence) -> KeymapResult<()> {
        let target_key = sequences.pop().unwrap();
        let iter = sequences.clone().into_iter();

        let node = iter.fold(Ok(self), |node, key| {
            node?
                .get_mut(&key)
                .map_err(|_| KeymapError::KeyNotFound(sequences.clone(), key.to_string()))
        })?;

        node.children.retain(|node| node.key != target_key);
        Ok(())
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

// Mode, KeyMap
#[derive(Debug, Default)]
pub struct KeyMap(HashMap<String, ActionNode>);

impl KeyMap {
    pub fn bind(&mut self, binding: KeyBinding, mode: String) {
        let target_keymap = self.0.entry(mode).or_default();
        target_keymap.bind(binding);
    }

    pub fn unbind(&mut self, sequences: KeySequence, mode: String) -> KeymapResult<()> {
        let target_keymap = self.0.get_mut(&mode).ok_or(KeymapError::NoSuchMode(mode))?;
        target_keymap.unbind(sequences);
        Ok(())
    }

    pub fn get_mode(&self, mode: &str) -> KeymapResult<&ActionNode> {
        self.0
            .get(mode)
            .ok_or(KeymapError::NoSuchMode(mode.to_string()))
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    use crate::keymap::{keycode::KeyCode, keys::KeyBinding};

    #[test]
    fn test() {
        let mut root = ActionNode::new(KeySpec(vec![], KeyCode::kVK_ANSI_A), KeyAction::Nop);

        root.bind(KeyBinding {
            sequences: KeySequence(vec![KeySpec(vec![], KeyCode::kVK_ANSI_B)]),
            action: KeyAction::Nop,
        });

        root.bind(KeyBinding {
            sequences: KeySequence(vec![
                KeySpec(vec![], KeyCode::kVK_Space),
                KeySpec(vec![], KeyCode::kVK_ANSI_C),
            ]),
            action: KeyAction::Nop,
        });

        println!("{:#?}", root);
    }
}
