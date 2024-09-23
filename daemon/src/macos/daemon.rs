use std::collections::BTreeSet;

use super::{
    runtime,
    utils::{self, consume_event},
};
use crate::keymap::{KeyAction, KeyCode, KeyModifier, KeySpec, KeymapDaemon};

pub fn run(keymap: KeymapDaemon) {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        runtime::mainloop(|event, key| {
            let tx = tx.clone();

            tx.send(key).unwrap();

            consume_event(event)
        });
    });

    let keymap_mutex = std::sync::Mutex::new(keymap);

    while let Ok(key) = rx.recv() {
        println!("Received key: {}", key);
        if KeySpec(
            BTreeSet::from_iter([
                KeyModifier::Fn,
                KeyModifier::Alt,
                KeyModifier::Cmd,
                KeyModifier::Ctrl,
            ]),
            KeyCode::kVK_F5,
        ) == key
        {
            break;
        }
        if let Ok(mut keymap) = keymap_mutex.lock() {
            if let Some(action) = keymap.make_input(key) {
                match action {
                    KeyAction::Nop => {}
                    KeyAction::ShellCmd(cmd) => {
                        let _ = std::process::Command::new("$SHELL")
                            .arg("-c")
                            .arg(cmd)
                            .output();
                    }
                    KeyAction::SendKey(key) => {
                        // TODO: report error
                        utils::post_key(key);
                    }
                    KeyAction::ModeChange(mode) => {
                        keymap.switch_mode(mode);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::run;
    use crate::keymap::*;

    #[test]
    fn test_run() {
        let mut keymap = KeymapDaemon::new();
        keymap.bind(
            None,
            KeySequence(vec![
                KeySpec(BTreeSet::new(), KeyCode::kVK_Space),
                KeySpec(BTreeSet::new(), KeyCode::kVK_ANSI_L),
            ]),
            KeyAction::ShellCmd("notify owo yee".into()),
        );

        run(keymap);
    }
}
