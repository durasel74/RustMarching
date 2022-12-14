use gl;
use gl::types::{ GLuint };
use super::{ ShaderError, ImageData };

/// Представляет косвенный объект текстуры OpenGL.
#[derive(Clone, Eq, PartialEq)]
pub struct Texture {
    pub id: GLuint,
}
impl Texture {
    /// Создает текстуру из файла с изображением.
    pub fn from_file(file_name: &str) -> Result<Self, ShaderError> {
        let image_data_result = ImageData::from_file(file_name);
        if let Err(err) = image_data_result { return Err(err) }
        let image_data = image_data_result.unwrap();
        Ok(Self::from_image_data(&image_data))
    }

    /// Создает текстуру из данных об изображении.
    pub fn from_image_data(image_data: &ImageData) -> Self {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image_data.width as i32, 
                image_data.height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 
                image_data.pixels_data.as_ptr() as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: texture_id }
    }

    /// Создает новую пустую текстуру.
    pub fn new_rgb(width: u32, height: u32, format: i32) -> Self {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, format, width as i32, height as i32, 0, 
                gl::RGBA, gl::UNSIGNED_BYTE, 0 as *const gl::types::GLvoid);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: texture_id }
    }

    /// Создает новую пустую текстуру для теста глубины и трафарета.
    pub fn new_depth_stencil(width: u32, height: u32) -> Self {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, 
                width as i32, height as i32, 0, gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8, 
                0 as *const gl::types::GLvoid);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Texture { id: texture_id }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}
