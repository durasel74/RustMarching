use cgmath::{ Vector2, Vector3, vec3, vec2 };

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

impl Vertex {
    pub fn from_position(pos: Vector3<f32>) -> Self {
        Vertex {
            position: pos,
            normal: vec3(0.0, 0.0, 0.0),
            tex_coords: vec2(0.0, 0.0)
        }
    }
}
