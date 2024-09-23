use std::collections::BTreeSet;

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{CGEvent, CGEventTapProxy, CGEventType};
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
};

use crate::keymap::{KeyCode, KeyModifier, KeySpec};
use crate::macos::utils::grab_key;

use super::utils;

/// Usually you will nee two threads:
/// 1. This mainloop thread
/// 2. The receiver thread which receive the data from channel
///
/// This function takes a **Fn** which cannot mutate **ANY** outer data.  
/// The solution is open a channel and send the data to the receiver thread.
pub(crate) fn mainloop<F>(callback: F)
where
    F: Fn(CGEventTapProxy, CGEventType, &CGEvent) -> Option<CGEvent>,
{
    let current = CFRunLoop::get_current();

    let tap = CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::KeyDown, CGEventType::FlagsChanged],
        callback,
    )
    .unwrap();

    unsafe {
        let loop_source = tap
            .mach_port
            .create_runloop_source(0)
            .expect("Somethings went bad ");
        current.add_source(&loop_source, kCFRunLoopCommonModes);
        tap.enable();
        CFRunLoop::run_current();
    }
}

fn ctrl_c_quit(key: &KeySpec) {
    if key
        == &KeySpec(
            BTreeSet::from_iter([KeyModifier::Ctrl]),
            KeyCode::kVK_ANSI_C,
        )
    {
        std::process::exit(0);
    }
}

/// Read the key event and print it out
pub fn observer_mode() {
    mainloop(|_, _, event| {
        let key = grab_key(event);
        println!("{}", key);
        ctrl_c_quit(&key);
        utils::consume_event(event)
    });
}

#[cfg(test)]
mod tests {

    use utils::{consume_event, post_key};

    use super::*;

    #[test]
    fn bind_f6_to_lock_screen() {
        mainloop(|_, _, event| {
            let key_spec = grab_key(event);
            ctrl_c_quit(&key_spec);

            if key_spec == KeySpec(BTreeSet::from_iter([KeyModifier::Fn]), KeyCode::kVK_F6) {
                let _ = post_key(KeySpec(
                    BTreeSet::from_iter([KeyModifier::Cmd, KeyModifier::Ctrl]),
                    KeyCode::kVK_ANSI_Q,
                ));
                consume_event(event)
            } else {
                Some(event.to_owned())
            }
        });
    }

    #[test]
    fn swap_esc_and_capslock() {
        mainloop(|_, _, event| {
            let key_spec = grab_key(event);
            ctrl_c_quit(&key_spec);

            if key_spec == KeySpec(BTreeSet::new(), KeyCode::kVK_Escape) {
                let _ = post_key(KeySpec(BTreeSet::new(), KeyCode::kVK_CapsLock));
                consume_event(event)
            } else if key_spec == KeySpec(BTreeSet::new(), KeyCode::kVK_CapsLock) {
                let _ = post_key(KeySpec(BTreeSet::new(), KeyCode::kVK_Escape));
                consume_event(event)
            } else {
                Some(event.to_owned())
            }
        });
    }
}
