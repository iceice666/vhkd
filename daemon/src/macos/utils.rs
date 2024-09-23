use core_graphics::{
    event::{CGEvent, CGEventFlags, CGEventTapLocation, EventField},
    event_source::{CGEventSource, CGEventSourceStateID},
};

use crate::keymap::{KeyModifier, KeySpec};

use super::error::MacOsRuntimeError;

const MY_FAVORITE_NUMBER: u32 = 0x114514;

pub(crate) fn consume_event(event: &CGEvent) -> Option<CGEvent> {
    event.set_flags(CGEventFlags::CGEventFlagNull);
    event.set_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE, i64::MIN);
    None
}

pub(crate) fn grab_data(event: &CGEvent) -> (i64, CGEventFlags) {
    let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
    let flags = event.get_flags();
    (keycode, flags)
}

pub(crate) fn post_key(key: KeySpec) -> Result<(), MacOsRuntimeError> {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_keyboard_event(source, 0, true)
        .map_err(|_| MacOsRuntimeError::CGEventError)?;
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
