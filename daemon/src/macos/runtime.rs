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
            let key = KeySpec(KeyModifier::from(flags), KeyCode::from(keycode));
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


/// Read the key event and print it out
pub(crate) fn observer_mode() {
    mainloop(|event, key| {
        println!("Key: {}", key);
        utils::consume_event(event)
    });
}


