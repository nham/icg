extern crate cgmath;
extern crate rand;

#[macro_use]
extern crate glium;

use cgmath::prelude::*;
use cgmath::Vector2;

const NUM_POINTS: u32 = 5000;

fn generate_points(vertices: [Vector2<f32>; 3], initial: Vector2<f32>) -> Vec<Vector2<f32>> {
    // initialize points vector with arbitrary point inside the triangle
    let mut points = vec![initial];

    // generate rest of points
    for i in 1..NUM_POINTS {
        let j = rand::random::<usize>() % 3;
        let sum = points[(i - 1) as usize] + vertices[j];
        points.push(sum / 2.0)
    }
    points
}

fn main() {
    use glium::{Surface, DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    display.get_window().unwrap().set_title("sierpinski points");

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    impl Vertex {
        fn from_Vector2(x: Vector2<f32>) -> Vertex {
            Vertex { position: cgmath::conv::array2(x) }
        }
    }

    implement_vertex!(Vertex, position);

    let vertices = [Vector2::new(-0.5f32, -0.5),
                    Vector2::new(0.75f32, -0.25),
                    Vector2::new(0.0f32, 0.5)];

    let points = generate_points(vertices, Vector2::new(0.0f32, 0.25));

    let conv_vertices: Vec<_> = points.into_iter()
                                      .map(|v| Vertex::from_Vector2(v))
                                      .collect();
    let vertex_buffer = glium::VertexBuffer::new(&display, &conv_vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

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
        target.clear_color(1.0, 1.0, 1.0, 1.0);
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
