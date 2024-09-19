use std::ffi::c_void;

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement,
    CGEventType, EventField,
};

fn callback(_: *const c_void, _et: CGEventType, event: &CGEvent) -> Option<CGEvent> {
    let flags = event.get_flags();
    let keycode = event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE);

    if CGEventFlags::CGEventFlagControl == flags && keycode == 0x3B {
        // Quit the app
        panic!("Keyboard interrupt")
    }
    println!("\rflag: {:?} keycode: 0x{:04X}", flags, keycode);
    None
}

pub fn mainloop() {
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
