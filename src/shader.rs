use gl::types::*;
use std::ffi::{CString, NulError};
use std::ptr;
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::uniform::{UniformManager, UniformValue};

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

#[derive(Clone)]
pub struct Shader {
    pub id: GLuint,
    pub name: String,
    uniform_manager: UniformManager
}

impl Shader {
    pub unsafe fn new(name: String, source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;

        let shader = Self {
            name,
            id: gl::CreateShader(shader_type),
            uniform_manager: UniformManager::new()
        };

        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());

        println!("Compiling shader...");
        gl::CompileShader(shader.id);

        //Check for compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        if success == 1 {
            println!("Success");
            Ok(shader)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            println!("Error: {}", log);
            Err(ShaderError::CompilationError(log))
        }
    }

    pub fn add_uniform(&mut self, key: String, value: UniformValue) {
        self.uniform_manager.add(key, value);
    }

    pub fn update_uniform_value(&mut self, key: String, new_value: UniformValue) {
        self.uniform_manager.update_value(key, new_value);
    }

    fn apply_uniform(&self, program_id: GLuint, key: String) {
        let location = unsafe {
            gl::GetUniformLocation(program_id, CString::new(key.clone()).unwrap().as_ptr())
        };

        if(location == -1) {
            println!("Uniform {} not found in shader", key.clone());
        } else {
            match self.uniform_manager.get_value(&key) {
                UniformValue::Float(value) => unsafe {
                    gl::Uniform1f(location, value as GLfloat);
                },
                UniformValue::Int(value) => unsafe {
                    gl::Uniform1i(location, value as GLint);
                },
                UniformValue::Mat4(value) => unsafe {
                    gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
                },
                UniformValue::Array_F(value) => unsafe {
                    gl::Uniform1fv(location, value.len() as GLsizei, value.as_ptr());
                },
                UniformValue::Vec2(value) => unsafe {
                    gl::Uniform2f(location, value.x, value.y);
                },
                UniformValue::Bool(value) => unsafe {
                    gl::Uniform1i(location, value as GLint);
                }
            }
        }
    }

    pub fn apply_uniforms(&self, program_id: GLuint) {
        for uniform in &self.uniform_manager.uniforms {
            self.apply_uniform(program_id, uniform.get_key());
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}