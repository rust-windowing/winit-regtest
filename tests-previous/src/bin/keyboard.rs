use std::{io::Write, thread::spawn, time::Duration};

use common::UserEvent;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopProxy,
};

use enigo::{Enigo, Key, KeyboardControllable};
use serde::Serialize;
use serde_json;

mod common;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct SerializableKeyEvent {
    pub physical_key: winit::keyboard::KeyCode,

    pub logical_key: winit::keyboard::Key<'static>,

    pub text: Option<&'static str>,

    pub location: winit::keyboard::KeyLocation,
    pub state: winit::event::ElementState,
    pub repeat: bool,
}

fn hacky_keyevent_from_winit(e: winit::event::KeyEvent) -> SerializableKeyEvent {
    SerializableKeyEvent {
        physical_key: e.physical_key,
        logical_key: e.logical_key,
        text: e.text,
        location: e.location,
        state: e.state,
        repeat: e.repeat,
    }
}

fn main() {
    common::run(do_input, |event, output| match event {
        Event::WindowEvent {
            event: window_event,
            ..
        } => match window_event {
            WindowEvent::KeyboardInput { event, .. } => {
                let hacky_event = hacky_keyevent_from_winit(event);
                let json = serde_json::to_string(&hacky_event).unwrap();
                writeln!(output, "{}", json).unwrap();
                output.flush().unwrap();
            }
            _ => (),
        },
        _ => (),
    });
}

fn do_input(el_proxy: EventLoopProxy<UserEvent>) {
    static TARGET_KEYS: &[Key] = &[
        Key::Alt,
        Key::Backspace,
        Key::CapsLock,
        Key::Control,
        Key::Delete,
        Key::DownArrow,
        Key::End,
        Key::Escape,
        Key::F1,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::Home,
        Key::LeftArrow,
        // Key::Meta,
        Key::Option,
        Key::PageDown,
        Key::PageUp,
        Key::Return,
        Key::RightArrow,
        Key::Shift,
        Key::Space,
        Key::Tab,
        Key::UpArrow,
        // Key::Layout('æ'),
        // Key::Layout('ø'),
        // Key::Layout('å'),
        // Key::Layout('é'),
        // Key::Layout('á'),
        // Key::Layout('ű'),
        // Key::Layout('ö'),
        // Key::Layout('ü'),
        // Key::Layout('ó'),
        // Key::Layout('ő'),
        // Key::Layout('ú'),
        // Key::Layout('ж')
    ];

    spawn(move || {
        let mut enigo = Enigo::new();
        std::thread::sleep(Duration::from_millis(100));

        for &k in TARGET_KEYS {
            enigo.key_down(k);
            std::thread::sleep(Duration::from_millis(10));
            enigo.key_up(k);
            std::thread::sleep(Duration::from_millis(10));
        }
        std::thread::sleep(Duration::from_millis(100));
        el_proxy.send_event(UserEvent::RequestStop).unwrap();
    });
}
