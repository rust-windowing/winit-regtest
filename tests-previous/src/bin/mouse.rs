use std::{fs::File, io::Write, thread::spawn, time::Duration};

use simple_logger::SimpleLogger;
use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use serde_json;

// const INWO: dpi::PhysicalPosition<i32> = dpi::PhysicalPosition::new(10, 100);

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(dpi::PhysicalSize::new(300.0, 300.0))
        .with_position(dpi::PhysicalPosition::new(100, 100))
        .build(&event_loop)
        .unwrap();

    let window_pos = window.inner_position().unwrap();
    let outer_pos = window.outer_position().unwrap();
    let outer_size = window.outer_size();
    let inner_size = window.inner_size();

    println!("POS inner: {:?}, outer: {:?}", window_pos, outer_pos);
    println!("SIZE inner: {:?}, outer: {:?}", inner_size, outer_size);

    let mut window_bottom_left = outer_pos;
    window_bottom_left.y += outer_size.height as i32;

    do_input(window_bottom_left);

    let mut output = File::create("mouse.txt").unwrap();

    let mut counter = 5;

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
                if false {
                    counter -= 1;

                    window.set_inner_size(dpi::PhysicalSize::new(200.0, 200.0));

                    let outer_size = window.outer_size();
                    let inner_size = window.inner_size();

                    println!("POS inner: {:?}, outer: {:?}", window_pos, outer_pos);
                    println!("SIZE inner: {:?}, outer: {:?}", inner_size, outer_size);

                    let window_pos = window.inner_position().unwrap();
                    let outer_pos = window.outer_position().unwrap();
                    println!("inner: {:?}, outer: {:?}", window_pos, outer_pos);
                }
            }
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::CursorMoved { position, .. } => {
                    let json = serde_json::to_string(&position).unwrap();
                    writeln!(output, "{}", json).unwrap();
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn do_input(window_bottom_left: dpi::PhysicalPosition<i32>) {
    // return;
    spawn(move || {
        let mut enigo = Enigo::new();
        std::thread::sleep(Duration::from_millis(100));
        let offset = 10;
        // enigo.mouse_move_to(0, 0);
        for _ in 0..10 {
            enigo.mouse_move_to(window_bottom_left.x, window_bottom_left.y);
            std::thread::sleep(Duration::from_millis(100));
            enigo.mouse_move_to(window_bottom_left.x + offset, window_bottom_left.y - offset);
            std::thread::sleep(Duration::from_millis(100));
        }

        std::process::exit(0);
    });
}
