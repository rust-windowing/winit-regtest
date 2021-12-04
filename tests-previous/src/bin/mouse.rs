use std::{
    fs::File,
    io::Write,
    sync::{atomic::AtomicBool, Mutex},
    thread::spawn,
    time::Duration,
};

use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use once_cell::sync::Lazy;
use serde_json;
use simple_logger::SimpleLogger;
use winit::{
    dpi,
    event::{Event, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop, EventLoopProxy},
    window::WindowBuilder,
};

// This just my guess at what could the largest decoration be in physical pixels
const MAX_DECORATION_SIZE: i32 = 150;
const TEST_CURSOR_OFFSET: i32 = 10;

static MOUSE_POS_READY: AtomicBool = AtomicBool::new(false);
static WINDOW_RELATIVE_POS: Lazy<Mutex<Option<dpi::PhysicalPosition<i32>>>> =
    Lazy::new(|| Mutex::new(None));

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    // Adding two to give enough space for the cursor (we place the cursor at MAX_DECORATION_SIZE+1 from the edge)
    let window_dimension = TEST_CURSOR_OFFSET + MAX_DECORATION_SIZE + 2;
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(dpi::PhysicalSize::new(window_dimension, window_dimension))
        .with_position(dpi::PhysicalPosition::new(100, 100))
        .build(&event_loop)
        .unwrap();

    let inner_pos = window.inner_position().unwrap();
    let outer_pos = window.outer_position().unwrap();
    let outer_size = window.outer_size();
    let inner_size = window.inner_size();

    println!("POS inner: {:?}, outer: {:?}", inner_pos, outer_pos);
    println!("SIZE inner: {:?}, outer: {:?}", inner_size, outer_size);

    // let mut window_bottom_left = outer_pos;
    // window_bottom_left.y += outer_size.height as i32;

    do_input(inner_pos, event_loop.create_proxy());

    let mut output = File::create("mouse.txt").unwrap();

    let mut counter = 5;

    let mut last_mouse_pos = dpi::PhysicalPosition::new(0, 0);
    let mut allow_recording = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // println!("{:?}", event);
        match event {
            Event::UserEvent(()) => {
                allow_recording = true;
                if MOUSE_POS_READY.load(std::sync::atomic::Ordering::SeqCst) {
                    *WINDOW_RELATIVE_POS.lock().unwrap() = Some(last_mouse_pos);
                }
            }
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

                    println!("POS inner: {:?}, outer: {:?}", inner_pos, outer_pos);
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
                    last_mouse_pos = dpi::PhysicalPosition::new(
                        position.x.round() as i32,
                        position.y.round() as i32,
                    );
                    if MOUSE_POS_READY.load(std::sync::atomic::Ordering::SeqCst) {
                        *WINDOW_RELATIVE_POS.lock().unwrap() = Some(last_mouse_pos);
                    }
                    if allow_recording {
                        let json = serde_json::to_string(&last_mouse_pos).unwrap();
                        writeln!(output, "{}", json).unwrap();
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}

fn do_input(window_top_right: dpi::PhysicalPosition<i32>, el_proxy: EventLoopProxy<()>) {
    // return;
    spawn(move || {
        let mut enigo = Enigo::new();
        std::thread::sleep(Duration::from_millis(100));

        let relative_base_x = MAX_DECORATION_SIZE + 1;
        let relative_base_y = MAX_DECORATION_SIZE + 1;

        let abs_base_x = window_top_right.x + relative_base_x;
        let abs_base_y = window_top_right.y + relative_base_y;
        let offset = TEST_CURSOR_OFFSET;
        // enigo.mouse_move_to(0, 0);
        for _ in 0..2 {
            enigo.mouse_move_to(abs_base_x + offset, abs_base_y + offset);
            std::thread::sleep(Duration::from_millis(100));
            enigo.mouse_move_to(abs_base_x, abs_base_y);
            std::thread::sleep(Duration::from_millis(100));
        }
        MOUSE_POS_READY.store(true, std::sync::atomic::Ordering::SeqCst);
        // Wake up the event loop
        el_proxy.send_event(());

        // Poll until we get the mouse pos from the window
        let relative_received_pos;
        loop {
            let value = WINDOW_RELATIVE_POS.lock().unwrap().clone();
            if let Some(pos) = value {
                relative_received_pos = pos;
                break;
            } else {
                continue;
            }
        }

        let diff_x = relative_base_x - relative_received_pos.x;
        let diff_y = relative_base_y - relative_received_pos.y;
        let abs_corrected_x = abs_base_x + diff_x;
        let abs_corrected_y = abs_base_y + diff_y;

        enigo.mouse_move_to(abs_corrected_x + offset, abs_corrected_y + offset);
        std::thread::sleep(Duration::from_millis(100));
        enigo.mouse_move_to(abs_corrected_x, abs_corrected_y + offset);
        std::thread::sleep(Duration::from_millis(100));
        enigo.mouse_move_to(abs_corrected_x + offset, abs_corrected_y);
        std::thread::sleep(Duration::from_millis(100));
        enigo.mouse_move_to(abs_corrected_x, abs_corrected_y);
        std::thread::sleep(Duration::from_millis(100));

        std::process::exit(0);
    });
}
