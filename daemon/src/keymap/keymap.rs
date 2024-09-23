use super::*;
use std::collections::BTreeMap;

pub struct KeymapDaemon {
    current_mode: String,
    current_sequence: KeySequence,
    keymaps: BTreeMap<(String, KeySequence), KeyAction>,
}

impl KeymapDaemon {
    pub fn new() -> Self {
        Self {
            current_mode: String::new(),
            current_sequence: KeySequence::new(),
            keymaps: BTreeMap::new(),
        }
    }

    fn try_interpret(&mut self) -> Option<KeyAction> {
        self.keymaps
            .get(&(self.current_mode.clone(), self.current_sequence.clone()))
            .cloned()
    }
}

// public apis
impl KeymapDaemon {
    pub fn bind(&mut self, mode: Option<String>, binding: KeySequence, action: KeyAction) {
        let mode = mode.unwrap_or_else(|| "default".into());
        self.keymaps.insert((mode, binding), action);
    }

    pub fn unbind(&mut self, mode: Option<String>, binding: KeySequence) -> Option<KeyAction> {
        let mode = mode.unwrap_or_else(|| "default".into());
        self.keymaps.remove(&(mode, binding))
    }

    pub fn switch_mode(&mut self, mode: String) {
        self.current_mode = mode
    }

    pub fn reset(&mut self) {
        self.current_mode = "default".into();
        self.current_sequence.clear();
    }

    pub fn make_input(&mut self, key: KeySpec) -> Option<KeyAction> {
        if matches!(key, KeySpec(_, KeyCode::Null)) {
            return None;
        }
        self.current_sequence.push(key);
        self.try_interpret()
    }
}
