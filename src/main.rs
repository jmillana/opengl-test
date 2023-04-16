use glium::glutin::event as GlutinEvent;
use glium::{glutin, Surface};
use std::time;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = time::Instant::now() + time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            GlutinEvent::Event::WindowEvent { event, .. } => match event {
                GlutinEvent::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            GlutinEvent::Event::NewEvents(cause) => match cause {
                GlutinEvent::StartCause::ResumeTimeReached { .. } => (),
                GlutinEvent::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
    });
}
