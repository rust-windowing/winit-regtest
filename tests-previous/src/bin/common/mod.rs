use std::fs::File;

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
pub enum UserEvent {
    RequestStop,
}

pub fn run<
    InitF: Fn(EventLoopProxy<UserEvent>),
    CallbackF: FnMut(Event<'_, UserEvent>, &mut File) + 'static,
>(
    init: InitF,
    mut callback: CallbackF,
) {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::with_user_event();

    let window = WindowBuilder::new()
        .with_title("winit test window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    init(event_loop.create_proxy());

    let exe_path = std::env::current_exe().unwrap();
    let output_filename = format!("{}.txt", exe_path.file_name().unwrap().to_string_lossy());
    let mut output = File::create(output_filename).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        // println!("{:?}", event);
        match &event {
            Event::UserEvent(UserEvent::RequestStop) => {
                *control_flow = ControlFlow::Exit;
            }
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
