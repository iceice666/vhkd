use super::{runtime, utils::{self, consume_event}};
use crate::keymap::{KeyAction, KeymapDaemon};


pub fn run() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        runtime::mainloop(|event, key| {
            let tx = tx.clone();

            tx.send(key).unwrap();

            consume_event(event)
        });
    });

    let keymap_mutex = std::sync::Mutex::new(KeymapDaemon::new());

    while let Ok(key) = rx.recv() {
        if let Ok(mut keymap) = keymap_mutex.lock() {
            let action = keymap.make_input(key);

            if let Some(action) = action {
                match action {
                    KeyAction::Nop => {}
                    KeyAction::ShellCmd(cmd) => {
                        let _ = std::process::Command::new("sh")
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
