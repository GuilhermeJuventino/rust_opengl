use glium::VertexBuffer;

// vertex struct
#[derive(Clone, Copy)]
pub struct Vertex {
    pub in_position: [f32; 3],
}

implement_vertex!(Vertex, in_position);

// function for creating a vertex array
pub fn get_vertex_array() -> Vec<Vertex> {
    let vertex_array = vec![
        Vertex {
            in_position: [-0.5, -0.5, 0.0],
        },
        Vertex {
            in_position: [0.5, -0.5, 0.0],
        },
        Vertex {
            in_position: [0.0, 0.5, 0.0],
        },
    ];

    vertex_array
}

pub fn get_vertex_buffer(
    display: &glium::Display,
    vertex_array: &[Vertex],
) -> VertexBuffer<Vertex> {
    glium::VertexBuffer::new(display, vertex_array).unwrap()
}
