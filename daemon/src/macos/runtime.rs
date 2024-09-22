use std::ffi::c_void;

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{CGEvent, CGEventType};
use core_graphics::event::{
    CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
};

use crate::keymap::{KeyCode, KeyModifier, KeySpec};

/// Usually you will nee two threads:
/// 1. This mainloop thread
/// 2. The receiver thread which receive the data from channel
///
/// This function takes a **Fn** which cannot mutate **ANY** outer data.  
/// The solution is open a channel and send the data to the receiver thread.
pub fn mainloop<F>(callback: F)
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
pub fn observer_mode() {
    mainloop(|event, key| {
        println!("Key: {}", key);
        utils::consume_event(event)
    });
}

pub mod utils {

    use core_graphics::{
        event::{CGEvent, CGEventFlags, CGEventTapLocation, EventField},
        event_source::{CGEventSource, CGEventSourceStateID},
    };

    use crate::keymap::{KeyModifier, KeySpec};

    pub fn consume_event(event: &CGEvent) -> Option<CGEvent> {
        event.set_flags(CGEventFlags::CGEventFlagNull);
        event.set_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE, i64::MIN);
        None
    }

    pub fn grab_data(event: &CGEvent) -> (i64, CGEventFlags) {
        let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
        let flags = event.get_flags();
        (keycode, flags)
    }

    const MY_FAVORITE_NUMBER: u32 = 0x114514;
    pub fn post(key: KeySpec) -> Result<(), ()> {
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
        let event = CGEvent::new_keyboard_event(source, 0, true)?;
        let flags = KeyModifier::into_event_flag(key.0);
        let key = key.1 as u16;

        event.set_string(&key.to_string());
        event.set_flags(flags);
        event.set_integer_value_field(
            EventField::EVENT_SOURCE_USER_DATA,
            MY_FAVORITE_NUMBER.into(),
        );
        event.post(CGEventTapLocation::HID);

        Ok(())
    }
}
