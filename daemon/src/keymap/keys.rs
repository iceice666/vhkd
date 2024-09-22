use std::fmt::Debug;
use std::fmt::Display;

use core_graphics::event::CGEventFlags;

use super::KeyCode;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum KeyAction {
    Nop,
    ShellCmd(String),
    ModeChange(String),
    SendKey(KeySpec),
}

#[derive(Clone, PartialEq, Eq, Debug)]
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

impl KeyModifier {
    pub fn from(value: CGEventFlags) -> Vec<Self> {
        let mut modifiers = Vec::new();
        if value.contains(CGEventFlags::CGEventFlagControl) {
            modifiers.push(KeyModifier::Ctrl);
        }
        if value.contains(CGEventFlags::CGEventFlagShift) {
            modifiers.push(KeyModifier::Shift);
        }
        if value.contains(CGEventFlags::CGEventFlagAlternate) {
            modifiers.push(KeyModifier::Alt);
        }
        if value.contains(CGEventFlags::CGEventFlagCommand) {
            modifiers.push(KeyModifier::Cmd);
        }
        if value.contains(CGEventFlags::CGEventFlagSecondaryFn) {
            modifiers.push(KeyModifier::Fn);
        }
        modifiers
    }

    pub fn into_event_flag(modifiers: Vec<Self>) -> CGEventFlags {
        modifiers
            .iter()
            .fold(CGEventFlags::CGEventFlagNull, |acc, modifier| {
                acc | match modifier {
                    KeyModifier::Ctrl => CGEventFlags::CGEventFlagControl,
                    KeyModifier::Shift => CGEventFlags::CGEventFlagShift,
                    KeyModifier::Alt => CGEventFlags::CGEventFlagAlternate,
                    KeyModifier::Cmd => CGEventFlags::CGEventFlagCommand,
                    KeyModifier::Fn => CGEventFlags::CGEventFlagSecondaryFn,
                }
            })
    }
}

#[derive(PartialEq, Eq, Clone)]
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
