use super::{keys::KeyBinding, KeyAction, KeyCode, KeySpec};

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

    pub fn get(&self, key: &KeySpec) -> Option<&ActionNode> {
        self.children.iter().find(|node| &node.key == key)
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
    pub fn unbind(&mut self, mut sequences: Vec<KeySpec>) {
        let mut node = self;
        let target_key = sequences.pop().unwrap();

        for key in sequences {
            node = node
                .children
                .iter_mut()
                .find(|node| node.key == key)
                .unwrap();
        }
        node.children.retain(|node| node.key != target_key);
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
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
            sequences: vec![KeySpec(vec![], KeyCode::kVK_ANSI_B)],
            action: KeyAction::Nop,
        });

        root.bind(KeyBinding {
            sequences: vec![
                KeySpec(vec![], KeyCode::kVK_Space),
                KeySpec(vec![], KeyCode::kVK_ANSI_C),
            ],
            action: KeyAction::Nop,
        });

        println!("{:#?}", root);
    }
}
