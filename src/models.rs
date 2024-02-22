use glium::index::NoIndices;
use glium::{Frame, Surface, VertexBuffer};

use crate::shaders::{get_fragment_shader, get_shader_program, get_vertex_shader};
use crate::vertex::{get_vertex_array, get_vertex_buffer, Vertex};

pub struct Triangle {
    _vertex_array: Vec<Vertex>,
    indicies: NoIndices,
    vertex_buffer: VertexBuffer<Vertex>,
    _vertex_shader: String,
    _fragment_shader: String,
    shader_program: glium::Program,
}

impl Triangle {
    pub fn new(display: &glium::Display) -> Self {
        let vertex_array = get_vertex_array();
        let indicies = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let vertex_buffer = get_vertex_buffer(display, &vertex_array);
        let vertex_shader = get_vertex_shader("src/shaders/default.vert".to_string());
        let fragment_shader = get_fragment_shader("src/shaders/default.frag".to_string());
        let shader_program = get_shader_program(display, &vertex_shader, &fragment_shader);

        Triangle {
            _vertex_array: vertex_array,
            indicies,
            vertex_buffer,
            _vertex_shader: vertex_shader,
            _fragment_shader: fragment_shader,
            shader_program,
        }
    }

    pub fn draw(&self, target: &mut Frame) {
        target
            .draw(
                &self.vertex_buffer,
                self.indicies,
                &self.shader_program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
