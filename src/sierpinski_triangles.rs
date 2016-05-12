extern crate cgmath;

#[macro_use]
extern crate glium;

use cgmath::prelude::*;
use cgmath::Vector2;
use std::iter::Extend;

const NUM_DIVISIONS: u8 = 5;
const NUM_TRIANGLES: u32 = 243; // 3^5
const NUM_POINTS: usize = (NUM_TRIANGLES as usize) * 3;

fn generate_triangles(vertices: [Vector2<f32>; 3],
                      triangle_points: &mut [Vector2<f32>],
                      depth: u8) {
    if depth == 0 {
        for i in 0..3 {
            triangle_points[i] = vertices[i];
        }
    } else {
        let (a, b, c) = (vertices[0], vertices[1], vertices[2]);
        let mid_ab = (vertices[0] + vertices[1])/2.0;
        let mid_ac = (vertices[0] + vertices[2])/2.0;
        let mid_bc = (vertices[1] + vertices[2])/2.0;
        let k = triangle_points.len() / 3;
        generate_triangles([a, mid_ab, mid_ac],
                           &mut triangle_points[..k],
                           depth-1);
        generate_triangles([mid_ab, b, mid_bc],
                           &mut triangle_points[k..(2*k)],
                           depth-1);
        generate_triangles([mid_bc, mid_ac, c],
                           &mut triangle_points[(2*k)..],
                           depth-1);
    }

}

fn generate_program<F>(display: &F) -> glium::Program where F: glium::backend::Facade {
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

    glium::Program::from_source(display,
                                vertex_shader_src,
                                fragment_shader_src,
                                None)
    .unwrap()
}

fn main() {
    use glium::{Surface, DisplayBuild};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    display.get_window().unwrap().set_title("sierpinski triangles");

    let vertices = [Vector2::new(-0.9f32, -0.7),
                    Vector2::new(0.9f32, 0.0),
                    Vector2::new(0.0f32, 0.75)];

    let mut points = [Vector2::new(0.0f32, 0.); NUM_POINTS];
    generate_triangles(vertices, &mut points[..], NUM_DIVISIONS);

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

    // convert the array of `cgmath::Vector2`s to an array of `Vertex`s
    let points_vertex: Vec<_> = points.into_iter()
                                      .map(|&v| Vertex::from_Vector2(v))
                                      .collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, &points_vertex).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
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
