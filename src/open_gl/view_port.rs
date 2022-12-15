use gl;
use cgmath::{ Vector3, vec3 };
use super::{ ShaderProgram, RenderData };

pub struct ViewPort {
    position: (i32, i32),
    size: (i32, i32),
    render_data: RenderData,
}

impl ViewPort {
    pub fn new() -> Self {
        ViewPort {
            position: (0, 0),
            size: (0, 0),
            render_data: Self::create_view_port_area(),
        }
    }

    pub fn position(&self) -> &(i32, i32) { &self.position }
    pub fn set_position(&mut self, value: (i32, i32)) { self.position = value; }

    pub fn size(&self) -> &(i32, i32) { &self.size }
    pub fn set_size(&mut self, value: (i32, i32)) { self.size = value; }

    pub fn draw(&self) {
        unsafe {
            gl::Viewport(self.position.0, self.position.1, self.size.0, self.size.1);
            gl::BindVertexArray(self.render_data.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.render_data.ebo);
            gl::DrawElements(gl::TRIANGLES, self.render_data.element_count as i32,
                gl::UNSIGNED_INT, 0 as *const gl::types::GLvoid);
        }
    }

    // Создает область отрисовки OpenGL
    fn create_view_port_area() -> RenderData {
        let vertices: Vec<Vector3<f32>> = vec![
            vec3(-1.0, 1.0, 0.0),
            vec3(1.0, 1.0, 0.0),
            vec3(1.0, -1.0, 0.0),
            vec3(-1.0, -1.0, 0.0)
        ];
        let indices: Vec<u32> = vec![0, 3, 2,  0, 2, 1];
        RenderData::from_vertices(&vertices, &indices)
    }
}
