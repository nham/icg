extern crate cgmath;

#[macro_use]
extern crate glium;

use cgmath::prelude::*;
use cgmath::Vector2;

const NUM_DIVISIONS: u8 = 5;

fn generate_triangles(vertices: [Vector2<f32>; 3], depth: u8) -> Vec<Vector2<f32>> {
    if depth == 0 {
        vec![vertices[0], vertices[1], vertices[2]]
    } else {
        let (a, b, c) = (vertices[0], vertices[1], vertices[2]);
        let mid_ab = (vertices[0] + vertices[1])/2.0;
        let mid_ac = (vertices[0] + vertices[2])/2.0;
        let mid_bc = (vertices[1] + vertices[2])/2.0;
        let mut triangles = generate_triangles([a, mid_ab, mid_ac], depth-1);
        triangles.extend(generate_triangles([mid_ab, b, mid_bc], depth-1));
        triangles.extend(generate_triangles([mid_bc, mid_ac, c], depth-1));
        triangles
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

    let points = generate_triangles(vertices, NUM_DIVISIONS);

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
                                      .map(|v| Vertex::from_Vector2(v))
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
