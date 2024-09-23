use super::{
    runtime,
    utils::{self, consume_event},
};
use crate::keymap::{KeyAction, KeymapDaemon};

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
        if let Ok(mut keymap) = keymap_mutex.lock() {
            let action = keymap.make_input(key);

            if let Some(action) = action {
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
    use super::run;
    use crate::keymap::*;

    #[test]
    fn test_run() {
        let mut keymap = KeymapDaemon::new();
        keymap.bind(
            None,
            KeySequence(vec![
                KeySpec(vec![], KeyCode::kVK_Space),
                KeySpec(vec![], KeyCode::kVK_ANSI_L),
            ]),
            KeyAction::ShellCmd("notify owo yee".into()),
        );

        run(keymap);
    }
}
