use std::fmt::Debug;
use std::fmt::Display;

use super::KeyCode;

#[derive(PartialEq, Eq, Debug)]
pub enum KeyAction {
    Nop,
    ShellCmd(String),
    ModeChange(String),
}

#[derive(PartialEq, Eq, Debug)]
pub enum KeyModifier {
    Ctrl,
    Shift,
    Alt,
    Cmd,
    Fn,
}

impl Display for KeyModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyModifier::Ctrl => write!(f, "Ctrl"),
            KeyModifier::Shift => write!(f, "Shift"),
            KeyModifier::Alt => write!(f, "Alt"),
            KeyModifier::Cmd => write!(f, "Cmd"),
            KeyModifier::Fn => write!(f, "Fn"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct KeySpec(pub Vec<KeyModifier>, pub KeyCode);

impl Debug for KeySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for modifier in &self.0 {
            write!(f, "{}-", modifier)?;
        }
        write!(f, "{:?}", self.1)
    }
}

pub struct KeyBinding {
    pub sequences: Vec<KeySpec>,
    pub action: KeyAction,
}
