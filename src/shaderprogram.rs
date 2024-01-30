use crate::shader::{Shader, ShaderError};
use gl::types::*;

pub struct ShaderProgram {
    pub id: GLuint,
    pub shaders: Vec<Shader>
}

impl ShaderProgram {
    pub unsafe fn new(shaders: Vec<Shader>) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
            shaders: shaders.clone()
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        println!("Linking shader program...");

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            println!("Success");
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            println!("Error: {}", log);
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub fn get_shader(&mut self, name: String) -> Option<&mut Shader> {
        for shader in &mut self.shaders {
            if shader.name == name {
                return Some(shader);
            }
        }

        None
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}