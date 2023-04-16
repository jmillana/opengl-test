use glium::glutin::event as GlutinEvent;
use glium::{glutin, implement_vertex, Surface};
use image;
use std::io::Cursor;
use std::time;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texture_coords: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coords);

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Load textures
    let image = image::load(
        Cursor::new(&include_bytes!("../assets/dog.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.to_vec(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    // Build the triangle
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        texture_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        texture_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        texture_coords: [1.0, 0.0],
    };

    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 texture_coords;
        out vec2 v_texture_coords;

        uniform mat4 matrix;

        void main() {
            v_texture_coords = texture_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_texture_coords;
        out vec4 color;

        uniform sampler2D object_texture;

        void main() {
            color = texture(object_texture, v_texture_coords);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut t: f32 = -0.5;

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

        // Update animation time
        t += 0.02;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = glium::uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32],
            ],
            texture: &texture,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
