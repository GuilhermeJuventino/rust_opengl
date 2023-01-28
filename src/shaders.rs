use std::fs;

pub fn get_vertex_shader(path: String) -> String {
    fs::read_to_string(path).expect("Failed to load vertex shader")
}

pub fn get_fragment_shader(path: String) -> String {
    fs::read_to_string(path).expect("Failed to load fragment shader")
}

pub fn get_shader_program(
    display: &glium::Display,
    vertex_shader: &str,
    fragment_shader: &str,
) -> glium::Program {
    glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap()
}
