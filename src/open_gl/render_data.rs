use gl;
use gl::types::{ GLint, GLuint, GLsizeiptr, GLvoid };

#[derive(Clone)]
pub struct RenderData {
    pub vbo: GLuint,
    pub vao: GLuint,
    pub ebo: GLuint,
    pub element_count: GLuint,
}
impl RenderData {
    pub fn from_vertices<T>(vertices: &Vec<T>, indices: &Vec<u32>) -> Self {
        let vbo = Self::create_vbo(vertices);
        let vao = Self::create_vao(vbo);
        let ebo = Self::create_ebo(indices);
        RenderData { vbo, vao, ebo, element_count: indices.len() as u32 }
    }

    pub fn create_vbo<T>(vertices: &Vec<T>) -> GLuint {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        vbo
    }

    pub fn create_vao(vbo: GLuint) -> GLuint {
        let mut vao: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        unsafe {
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null()
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        vao
    }

    pub fn create_ebo(indices: &Vec<u32>) -> GLuint {
        let mut ebo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        ebo
    }
}
impl Drop for RenderData {
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
         }
    }
}
