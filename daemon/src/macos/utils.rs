use core_graphics::{
    event::{CGEvent, CGEventFlags, CGEventTapLocation, EventField},
    event_source::{CGEventSource, CGEventSourceStateID},
};

use crate::keymap::{KeyModifier, KeySpec};

use super::{error::MacOsRuntimeError, KeyCode};

const MY_FAVORITE_NUMBER: i64 = 0x114514;

pub(crate) fn consume_event(event: &CGEvent) -> Option<CGEvent> {
    event.set_flags(CGEventFlags::CGEventFlagNull);
    event.set_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE, i64::MAX);
    None
}

pub(crate) fn grab_data(event: &CGEvent) -> (i64, CGEventFlags) {
    let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
    let flags = event.get_flags();
    println!("keycode: {}, flags: {:?}", keycode, flags);
    (keycode, flags)
}

pub(crate) fn post_key(key: KeySpec) -> Result<(), MacOsRuntimeError> {
    let flags = KeyModifier::into_event_flag(key.0);
    if key.1 == KeyCode::Null {
        return Ok(());
    }
    let key = key.1.into();

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_keyboard_event(source, key, true)
        .map_err(|_| MacOsRuntimeError::CGEventError)?;

    // event.set_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE, key as i64);
    event.set_flags(flags);
    event.set_integer_value_field(EventField::EVENT_SOURCE_USER_DATA, MY_FAVORITE_NUMBER);
    event.post(CGEventTapLocation::HID);

    Ok(())
}
