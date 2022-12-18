use cgmath::prelude::InnerSpace;
use cgmath::{ Matrix, Matrix4, Vector3, vec3, Vector4, vec4 };

pub struct Camera {
    position: Vector3<f32>,
    direction: Vector3<f32>,
    field_of_view: f32,
}
impl Camera {
    pub fn new() -> Self {
        Camera { 
            position: vec3(0.0, 0.0, 0.0),
            direction: vec3(0.0, 0.0, 1.0),
            field_of_view: 1.0,
        }
    }

    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn set_position(&mut self, value: Vector3<f32>) { self.position = value }

    pub fn direction(&self) -> Vector3<f32> { self.direction }
    pub fn set_direction(&mut self, value: Vector3<f32>) { self.direction = value.normalize(); }

    pub fn field_of_view(&self) -> f32 { self.field_of_view }
    pub fn set_field_of_view(&mut self, value: f32) { self.field_of_view = value; }

    pub fn right(&self) -> Vector3<f32> {
        let cross = vec3(0.0, 1.0, 0.0).cross(self.direction());
        cross.normalize()
    }
    
    pub fn up(&self) -> Vector3<f32> {
        self.direction().cross(self.right())
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let direction_matrix = Matrix4::from_cols(
            self.right().extend(0.0),
            self.up().extend(0.0),
            self.direction().extend(0.0),
            vec4(0.0, 0.0, 0.0, 1.0),
        );
        let pos = vec3(-self.position.x, -self.position.y, -self.position.z);
        let position_matrix = Matrix4::from_translation(pos);
        direction_matrix.transpose() * position_matrix
    }
}
