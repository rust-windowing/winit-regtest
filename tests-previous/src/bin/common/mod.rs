
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::fs::File;

pub fn run<InitF: Fn(), CallbackF: FnMut(Event<'_, ()>, &mut File) + 'static>(
    init: InitF, mut callback: CallbackF
) {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("winit test window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    init();

    let exe_path = std::env::current_exe().unwrap();
    let mut output_filename = exe_path.file_name().unwrap().to_string_lossy().into_owned();
    output_filename.push_str(".txt");
    let mut output = File::create(output_filename).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // println!("{:?}", event);
        match &event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if *window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
        callback(event, &mut output);
    });
}
