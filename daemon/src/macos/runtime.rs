use std::collections::BTreeSet;
use std::ffi::c_void;

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{CGEvent, CGEventType};
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
};

use crate::keymap::{KeyCode, KeyModifier, KeySpec};

use super::utils;

/// Usually you will nee two threads:
/// 1. This mainloop thread
/// 2. The receiver thread which receive the data from channel
///
/// This function takes a **Fn** which cannot mutate **ANY** outer data.  
/// The solution is open a channel and send the data to the receiver thread.
pub(crate) fn mainloop<F>(callback: F)
where
    F: Fn(&CGEvent, KeySpec) -> Option<CGEvent>,
{
    let current = CFRunLoop::get_current();

    let tap = CGEventTap::new(
        CGEventTapLocation::Session,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::KeyDown, CGEventType::FlagsChanged],
        move |_: *const c_void, _et: CGEventType, event: &CGEvent| -> Option<CGEvent> {
            let (keycode, flags) = utils::grab_data(event);
            let mut keycode = KeyCode::from(keycode as u16);

            // Ignore the modifier keys
            if matches!(
                keycode,
                KeyCode::kVK_Option
                    | KeyCode::kVK_RightOption
                    | KeyCode::kVK_Shift
                    | KeyCode::kVK_RightShift
                    | KeyCode::kVK_Command
                    | KeyCode::kVK_RightCommand
                    | KeyCode::kVK_Control
                    | KeyCode::kVK_RightControl
                    | KeyCode::kVK_Function
            ) {
                keycode = KeyCode::Null
            }

            let modifiers = KeyModifier::from(flags);

            let key = KeySpec(modifiers, keycode);
            callback(event, key)
        },
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
    mainloop(|event, key| {
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
        mainloop(|event, key_spec| {
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
}
