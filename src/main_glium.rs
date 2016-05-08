extern crate cgmath;

#[macro_use]
extern crate glium;

use cgmath::prelude::*;

fn main() {
    use glium::{Surface, DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertices: () = vec![Vertex { position: [-0.5, -0.5] },
                            Vertex { position: [0.75, -0.25] },
                            Vertex { position: [0., 0.5] }];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 150

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None)
                      .unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer,
                    &indices,
                    &program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default())
              .unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            if let glium::glutin::Event::Closed = event {
                return;
            }
        }
    }
}

// fn main() {
// let mut window = glutin::WindowBuilder::new().build().unwrap();
// window.set_title("simple OpenGL example");
// let _ = unsafe { window.make_current() };
//
// println!("Pixel format of the window: {:?}", window.get_pixel_format());
//
// let context = support::load(&window);
//
// for event in window.wait_events() {
// context.draw_frame((0.0, 1.0, 0.0, 1.0));
// let _ = window.swap_buffers();
//
// match event {
// glutin::Event::Closed => break,
// _ => ()
// }
// }
// }
//
