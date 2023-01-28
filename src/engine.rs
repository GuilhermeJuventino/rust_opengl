use glium::*;

use crate::models::Triangle;

// graphics engine struct
pub struct Engine {
    pub event_loop: glutin::event_loop::EventLoop<()>,
    pub window_builder: glutin::window::WindowBuilder,
    pub context_builder: glutin::ContextBuilder<'static, glutin::NotCurrent>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            event_loop: glutin::event_loop::EventLoop::new(),
            window_builder: glutin::window::WindowBuilder::new(),
            context_builder: glutin::ContextBuilder::new(),
        }
    }

    pub fn run(self) {
        // creating the display
        let display =
            glium::Display::new(self.window_builder, self.context_builder, &self.event_loop)
                .unwrap();

        let mut objects: Vec<Triangle> = Vec::new();

        let triangle = Triangle::new(&display);
        objects.push(triangle);

        // running the main event loop
        self.event_loop.run(move |ev, _, control_flow| {
            // drawing into the display
            draw(&display, &objects);

            // getting the time until next frame
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            // checking events
            check_events(ev, control_flow);
        })
    }
}

fn check_events(ev: glutin::event::Event<()>, control_flow: &mut glutin::event_loop::ControlFlow) {
    if let glutin::event::Event::WindowEvent { event, .. } = ev {
        match event {
            // checking if window should close
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            _ => (),
        }
    }
}

fn draw(display: &glium::Display, objects: &[Triangle]) {
    // drawing to the target display
    let mut target = display.draw();

    // clearing the screen after each frame with a blue color
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    for object in objects.iter() {
        object.draw(&mut target);
    }

    // swap the buffers and destroy previous frame
    target.finish().unwrap();
}
