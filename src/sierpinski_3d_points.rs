extern crate cgmath;
extern crate rand;

#[macro_use]
extern crate glium;

use cgmath::prelude::*;
use cgmath::Vector3;

const NUM_POINTS: u32 = 90000;

fn generate_points(vertices: [Vector3<f32>; 4], initial: Vector3<f32>) -> Vec<Vector3<f32>> {
    // initialize points vector with arbitrary point inside the triangle
    let mut points = Vec::with_capacity(NUM_POINTS as usize);
    points.push(initial);

    // generate rest of points
    for i in 1..NUM_POINTS {
        let j = rand::random::<usize>() % 4;
        let sum = points[(i - 1) as usize] + vertices[j];
        points.push(sum / 2.0)
    }
    points
}

fn generate_program<F>(display: &F) -> glium::Program where F: glium::backend::Facade {
    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        out vec4 color;

        void main() {
            color = vec4((1.0 + position.xyz)/2.0, 1.0);
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec4 color;
        out vec4 fragColor;

        void main() {
            fragColor = color;
        }
    "#;

    let program = glium::Program::from_source(display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None);

    match program {
        Ok(p) => p,
        Err(e) => panic!("Error compiling program: {}", e),
    }
}

fn main() {
    use glium::{Surface, DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    display.get_window().unwrap().set_title("sierpinski points");

    let vertices = [Vector3::new(-1.0f32, -1.0, -1.0),
                    Vector3::new( 1.0f32, -1.0, -1.0),
                    Vector3::new( 0.0f32,  1.0, -1.0),
                    Vector3::new( 0.0f32,  0.0,  1.0)];

    let points = generate_points(vertices, Vector3::new(0.0f32, 0.0, 0.0));

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }

    impl Vertex {
        fn from_Vector3(x: Vector3<f32>) -> Vertex {
            Vertex { position: cgmath::conv::array3(x) }
        }
    }

    implement_vertex!(Vertex, position);

    // convert the array of `cgmath::Vector3`s to an array of `Vertex`s
    let points_vertex: Vec<_> = points.into_iter()
                                      .map(|v| Vertex::from_Vector3(v))
                                      .collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, &points_vertex).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let program = generate_program(&display);

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
