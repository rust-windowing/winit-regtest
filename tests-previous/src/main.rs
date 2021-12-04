use std::{fs::File, io::Write, thread::spawn, time::Duration};

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use enigo::{Enigo, Key, KeyboardControllable};
use serde_json;

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    do_input();

    let mut output = File::create("output.txt").unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // println!("{:?}", event);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: window_event,
                ..
            } => {
                // match window_event {
                //     WindowEvent::KeyboardInput {
                //         input,
                //         ..
                //     } => {
                //         let json = serde_json::to_string(&input).unwrap();
                //         writeln!(output, "{}", json).unwrap();
                //     }
                //     _ => ()
                // }
            }
            _ => (),
        }
    });
}

fn do_input() {
    spawn(move || {
        let mut enigo = Enigo::new();
        std::thread::sleep(Duration::from_millis(100));
        enigo.key_down(Key::F2);
        std::thread::sleep(Duration::from_millis(10));
        enigo.key_up(Key::F2);
        std::thread::sleep(Duration::from_millis(100));
        std::process::exit(0);
    });
}
