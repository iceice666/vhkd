use std::os::raw::c_void;

use core_graphics::{
    event::{CGEvent, CGEventFlags, CGEventTapCallBackFn, CGEventTapLocation, CGEventTapProxy, CGEventType, EventField},
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

pub(crate) fn grab_key(event : &CGEvent)  -> KeySpec  {
    let flags = event.get_flags();
    let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);
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

     KeySpec(modifiers, keycode)
}
