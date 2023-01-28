use crate::engine::Engine;

#[macro_use]
extern crate glium;

mod engine;
mod models;
mod shaders;
mod vertex;

fn main() {
    let engine = Engine::new();

    engine.run();
}
