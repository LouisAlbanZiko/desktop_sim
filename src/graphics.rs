/*
pub mod vertex {
    pub trait Vertex {
        fn gen_layout() -> Vec<Attribute>;
    }

    pub struct Attribute {
        gl_type: u32,
        gl_count: u32
    }

    pub struct VertexBuffer<T: Sized> {
        data: Vec<T>,
        gl_id: u32,
        size: u32,
        layout: Vec<Attribute>
    }

    impl<T> VertexBuffer<T> where T : Sized, T: Vertex {
        pub fn new(data: Vec<T>) -> Self {
            let mut buffer: VertexBuffer<T> = VertexBuffer { data: data, gl_id: 0, size: std::mem::size_of(), layout: T::gen_layout() };

            unsafe {
                gl::GenBuffers(1, &mut buffer.gl_id as *mut u32);
                gl::BindBuffer(gl::ARRAY_BUFFER, buffer.gl_id);
                //gl::BufferData(gl::ARRAY_BUFFER, );
            }

            buffer
        }
    }

}*/

use gl::types::*;
use crate::array2d::*;

pub struct Rect {
	vertices: [GLfloat; 4 * 4],
	indices: [GLuint; 6],
	vertex_buffer: GLuint,
	index_buffer: GLuint
}

fn map<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Div<Output=T> + std::ops::Mul<Output=T> + Copy>(start1: T, end1: T, start2: T, end2: T, value: T) -> T {
	((value - start1) / (end1 - start1)) * (end2 - start2) + start2
}

impl Rect {
	pub fn new(x: f32, y: f32, w: f32, h: f32, x_w: f32, y_w: f32, w_w: f32, h_w: f32) -> Self {
		println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);
		let mut rect = Rect {
			vertices: [
				map(x_w, x_w + w_w, -1.0, 1.0, x), 		map(y_w, y_w + h_w, 1.0, -1.0, y), 		0.0, 1.0,
				map(x_w, x_w + w_w, -1.0, 1.0, x + w),		map(y_w, y_w + h_w, 1.0, -1.0, y), 		1.0, 1.0,
				map(x_w, x_w + w_w, -1.0, 1.0, x + w), 	map(y_w, y_w + h_w, 1.0, -1.0, y + h),		1.0, 0.0,
				map(x_w, x_w + w_w, -1.0, 1.0, x), 		map(y_w, y_w + h_w, 1.0, -1.0, y + h), 	0.0, 0.0
			],
			indices: [
				0, 1, 2,
				2, 3, 0
			],
			vertex_buffer: 0,
			index_buffer: 0
		};
		unsafe {
			gl::GenBuffers(1, &mut rect.vertex_buffer as *mut u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, rect.vertex_buffer);
			println!("size: {}", (std::mem::size_of::<GLfloat>() * 4 * 4) as isize);
			gl::BufferData(gl::ARRAY_BUFFER, (std::mem::size_of::<GLfloat>() * 4 * 4) as isize, rect.vertices.as_ptr() as *const _, gl::STATIC_DRAW);
		}

		unsafe {
			let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;
			gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * std::mem::size_of::<GLfloat>() as GLsizei) as *const _);
			gl::EnableVertexAttribArray(1);
		}

		unsafe {
			gl::GenBuffers(1, &mut rect.index_buffer as *mut u32);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, rect.index_buffer);
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (std::mem::size_of::<GLuint>() * 6) as isize, rect.indices.as_ptr() as *const _, gl::STATIC_DRAW);
		}
		
		rect
	}

	pub fn draw(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);

			let stride = 4 * std::mem::size_of::<GLfloat>() as GLsizei;
			gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
			gl::EnableVertexAttribArray(0);
			gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * std::mem::size_of::<GLfloat>() as GLsizei) as *const _);
			gl::EnableVertexAttribArray(1);

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
			
			gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
		}
	}
}

pub mod shader {

	extern crate nalgebra_glm as glm;

    use gl::types::*;
    use std::ffi::CString;

    pub struct Shader {
        gl_id: gl::types::GLuint,
    }

    impl Shader {
        pub fn bind(&self) {
            unsafe { gl::UseProgram(self.gl_id) };
        }

        /*pub fn setUniform<T: Sized>(&self, name: &str, data: &T) {
            let location =
                unsafe { gl::GetUniformLocation(self.gl_id, name.as_ptr() as *const GLchar) };
        }*/

		#[allow(dead_code)]
        pub fn set_uniform4fv(&self, name: &str, data: &glm::Mat4) {
            let location =
                unsafe { gl::GetUniformLocation(self.gl_id, name.as_ptr() as *const GLchar) };
            unsafe {
                gl::UniformMatrix4fv(
                    location,
                    1,
                    gl::FALSE,
                    std::ptr::addr_of!(data) as *const f32,
                )
            }
        }
    }

    pub fn load(shader_name: &str) -> Shader {
        let vertex_shader_source: String =
            std::fs::read_to_string(format!("shaders/{}.vert", shader_name)).unwrap();
        let fragment_shader_source: String =
            std::fs::read_to_string(format!("shaders/{}.frag", shader_name)).unwrap();

        // Compile and link the shaders
        let vertex_shader = compile_shader(&vertex_shader_source[..], gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(&fragment_shader_source[..], gl::FRAGMENT_SHADER);
        let shader_program = link_program(vertex_shader, fragment_shader);

        Shader {
            gl_id: shader_program,
        }
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(shader_type);

            let c_str = CString::new(source.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success: GLint = 1;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len: GLint = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error: CString = CString::from_vec_unchecked(buffer);

                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
                println!("Shader compilation failed: {}", error.to_string_lossy());
            }

            shader
        }
    }

    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
        unsafe {
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success: GLint = 1;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut len: GLint = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

                let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
                buffer.extend([b' '].iter().cycle().take(len as usize));
                let error: CString = CString::from_vec_unchecked(buffer);

                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
                println!("Shader program linking failed: {}", error.to_string_lossy());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program
        }
    }
}

#[derive(Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8
}

impl Clone for Color {
	fn clone(&self) -> Self {
		Color {
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a
		}
	}
}

pub struct Texture {
	pub width: i32,
	pub height: i32,
	gl_id: GLuint
}

impl Texture {
	pub fn new(width: i32, height: i32) -> Texture {
		let mut gl_id: GLuint = 0;
		unsafe {
            gl::GenTextures(1, std::ptr::addr_of_mut!(gl_id));
            gl::BindTexture(gl::TEXTURE_2D, gl_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                0 as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

		Texture {
			width: width,
			height: height,
			gl_id: gl_id
		}
	}

    pub fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
		}
	}

    pub fn set_data(&self, data: &Array2D<Color>) {
		self.bind();
        unsafe{
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
				self.width,
                self.height,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
		}
    }
}

#[allow(dead_code)]
pub struct FrameBuffer {
	pub texture: Texture,
    gl_id: GLuint
}

#[allow(dead_code)]
impl FrameBuffer {
    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_id);
            gl::Viewport(0, 0, self.texture.width, self.texture.height);
        }
    }

    pub fn new(width: i32, height: i32) -> Result<FrameBuffer, String> {
        let mut frame_buffer_name: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, std::ptr::addr_of_mut!(frame_buffer_name));
            gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer_name);
        }

		let texture: Texture = Texture::new(width, height);

        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.gl_id, 0);

            let draw_buffers: [GLenum; 1] = [gl::COLOR_ATTACHMENT0];
            gl::DrawBuffers(1, draw_buffers.as_ptr());
        }

        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err("Failed to create frame buffer.".to_owned());
            }
        }

        Ok(FrameBuffer {
            gl_id: frame_buffer_name,
            texture: texture
        })
    }

    pub fn default(width: i32, height: i32) -> FrameBuffer {
        FrameBuffer {
            gl_id: 0,
            texture: Texture {
				width: width,
				height: height,
				gl_id: 0
			}
        }
    }
}
