use crate::visual::glfw::Glfw;
use crate::visual::vector::Mat4;
use std::ffi::{c_void, CString};
use std::mem::size_of;
use std::ptr::{null, null_mut};
use std::slice;

use crate::visual::generated::gl;
use crate::visual::generated::gl::types::{GLchar, GLint, GLuint};
use crate::visual::generated::gl::Gles2;

use super::webp::WebP;
use super::Model;
pub struct Gl {
    gl: Gles2,
    programs: Programs,

    vao_static: Vao,

    arrays_static: Buffer,
    elements_static: Buffer,

    camera: Mat4,
    light: Mat4,
}

impl Gl {
    pub fn new(glfw: &Glfw) -> Gl {
        let loadfn = glfw.get_loadfn();
        let gl = Gles2::load_with(loadfn);

        #[cfg(debug_assertions)]
        unsafe {
            gl.DebugMessageCallbackKHR(Some(debug_callback), null_mut());
            gl.Enable(gl::DEBUG_OUTPUT);
        }

        unsafe {
            // for textures that where width is no multiple of 4
            // otherwise, opengl does dome weird alignment stuff
            gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        let vao_static = Vao::new(&gl);
        let arrays_static = Buffer::new(&gl);
        let elements_static = Buffer::new(&gl);
        unsafe {
            gl.BindVertexArrayOES(vao_static.0);

            gl.BindBuffer(gl::ARRAY_BUFFER, arrays_static.0);
            gl.EnableVertexAttribArray(0);
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(0, 3, gl::SHORT, gl::TRUE, 12, null());
            gl.VertexAttribPointer(1, 2, gl::SHORT, gl::TRUE, 12, 8 as *const c_void);

            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, elements_static.0);
        }

        let programs = Programs::new(&gl);

        unsafe {
            gl.Enable(gl::DEPTH_TEST);
        }
        Gl {
            gl,
            programs,
            vao_static,
            arrays_static,
            elements_static,
            camera: Mat4::new(),
            light: Mat4::new(),
        }
    }
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.ClearColor(r, g, b, a);
        }
    }
    pub fn clear(&self, color_bit: bool, depth_bit: bool) {
        let buffer_bit = if color_bit { gl::COLOR_BUFFER_BIT } else { 0 }
            | if depth_bit { gl::DEPTH_BUFFER_BIT } else { 0 };
        unsafe {
            self.gl.Clear(buffer_bit);
        }
    }
    pub fn set_camera(&mut self, camera: Mat4) {
        self.camera = camera;
    }
    pub fn draw(
        &mut self,
        render_size: &(i32, i32),
        models_static: &[Model],
        models_static_dirty: bool,
    ) {
        unsafe {
            self.gl.Viewport(0, 0, render_size.0, render_size.1);
        }
        if models_static.is_empty() {
            return;
        }
        unsafe {
            self.gl.UseProgram(self.programs.default.0);
            self.gl
                .UniformMatrix4fv(self.programs.camera, 1, gl::FALSE, self.camera.as_ptr());
            self.gl
                .UniformMatrix4fv(self.programs.light, 1, gl::FALSE, self.light.as_ptr());
        }
        if models_static_dirty {
            let (arrays_data_len, elements_data_len) =
                models_static
                    .iter()
                    .fold((0, 0), |(arrays_size, elements_size), model| {
                        (
                            arrays_size + model.0.arrays.len() * 6,
                            elements_size + model.0.elements.len(),
                        )
                    });
            let mut arrays_data = Vec::with_capacity(arrays_data_len);
            let mut elements_data = Vec::with_capacity(elements_data_len);
            models_static.iter().for_each(|model| {
                let internal = &model.0;
                let offset = (arrays_data.len() / 6) as u16; // offset that must be added to the element index
                internal
                    .arrays
                    .iter()
                    .for_each(|point| arrays_data.extend_from_slice(point));
                internal
                    .elements
                    .iter()
                    .for_each(|index| elements_data.push(index + offset));
            });
            unsafe {
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.arrays_static.0);
                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (arrays_data_len * size_of::<i16>()) as isize,
                    arrays_data.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                // ELEMENTS_BUFFER should already be bound because of VAO
                self.gl
                    .BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.elements_static.0);
                self.gl.BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (elements_data_len * size_of::<u16>()) as isize,
                    elements_data.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        }
        unsafe {
            self.gl.BindVertexArrayOES(self.vao_static.0);
        }
        let mut offset = 0;
        for model in models_static {
            let count = model.0.elements.len() as i32;
            match &model.0.texture {
                Some(texture) => unsafe {
                    self.gl.ActiveTexture(gl::TEXTURE0);
                    self.gl.BindTexture(gl::TEXTURE_2D, texture.0);
                },
                None => {} // TODO: Bind dummy texture
            }
            let instances = model.0.instances.take();
            for instance in &instances {
                instance.with_spatial(|spatial| unsafe {
                    self.gl.UniformMatrix4fv(
                        self.programs.model,
                        1,
                        gl::FALSE,
                        spatial.to_mat4().as_ptr(),
                    );
                    self.gl.DrawElements(
                        gl::TRIANGLES,
                        count,
                        gl::UNSIGNED_SHORT,
                        (offset * size_of::<u16>()) as *const c_void,
                    );
                });
            }
            model.0.instances.set(instances);
            offset += count as usize;
        }
    }
    pub fn new_texture(&mut self, webp: WebP) -> Texture {
        let mut handles = [0];
        unsafe {
            self.gl.GenTextures(1, &mut handles as *mut _ as _);
            self.gl.BindTexture(gl::TEXTURE_2D, handles[0]);
            self.gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                webp.width as _,
                webp.height as _,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                webp.data.as_ptr() as _,
            );
            // only works because of the pixelstorei-command when Self is initialized
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            self.gl
                .TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        }
        Texture(handles[0])
    }
    pub fn drop_texture(&mut self, texture: &Texture) {
        unsafe {
            self.gl.DeleteTextures(1, &[texture.0] as *const _);
        }
    }
}

impl Drop for Gl {
    fn drop(&mut self) {
        Programs::drop_with_gl(&mut self.programs, &self.gl);
        Buffer::drop_with_gl(&mut self.arrays_static, &self.gl);
        Buffer::drop_with_gl(&mut self.elements_static, &self.gl);
        Vao::drop_with_gl(&mut self.vao_static, &self.gl);
    }
}

extern "system" fn debug_callback(
    _source: u32,
    _ty: u32,
    _id: u32,
    _severity: u32,
    length: i32,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    let text_slice = unsafe { slice::from_raw_parts(message as *const u8, length as usize) };
    let text = String::from_utf8(Vec::from(text_slice)).unwrap();
    println!("GL DEBUG: {}", text);
}

struct Program(u32);
struct Shader(u32);

impl Program {
    fn new(
        gl: &Gles2,
        vert_src: &'static str,
        frag_src: &'static str,
        attrib_locations: &[&'static str],
    ) -> Program {
        let handle = unsafe { gl.CreateProgram() };
        let mut vert = Shader::new(gl, vert_src, gl::VERTEX_SHADER);
        let mut frag = Shader::new(gl, frag_src, gl::FRAGMENT_SHADER);

        for (i, name) in attrib_locations.iter().enumerate() {
            let name_c = CString::new(*name).unwrap();
            unsafe {
                gl.BindAttribLocation(handle, i as GLuint, name_c.as_ptr());
            }
        }

        unsafe {
            gl.AttachShader(handle, vert.0);
            gl.AttachShader(handle, frag.0);
            gl.LinkProgram(handle);
            gl.DetachShader(handle, vert.0);
            gl.DetachShader(handle, frag.0);
        }

        Shader::drop_with_gl(&mut frag, gl);
        Shader::drop_with_gl(&mut vert, gl);

        Program(handle)
    }
    fn drop_with_gl(&mut self, gl: &Gles2) {
        unsafe {
            gl.DeleteProgram(self.0);
        }
    }
}

impl Shader {
    fn new(gl: &Gles2, src: &'static str, shader_type: u32) -> Shader {
        let handle = unsafe { gl.CreateShader(shader_type) };
        unsafe {
            gl.ShaderSource(
                handle,
                1,
                &[src.as_ptr()] as *const *const u8 as *const *const _,
                &[src.len() as GLint] as *const _,
            );
            gl.CompileShader(handle);
        }
        Shader(handle)
    }
    fn drop_with_gl(&mut self, gl: &Gles2) {
        unsafe {
            gl.DeleteShader(self.0);
        }
    }
}

struct Programs {
    default: Program,

    camera: GLint,
    light: GLint,
    model: GLint,
}

impl Programs {
    fn new(gl: &Gles2) -> Programs {
        let default = Program::new(
            gl,
            include_str!("shaders/default.vert"),
            include_str!("shaders/default.frag"),
            &["pos_in", "tex_in"],
        );
        unsafe {
            gl.UseProgram(default.0); // probably only important for setting "tex"
        }
        let camera = unsafe { gl.GetUniformLocation(default.0, "camera\0".as_ptr() as *const _) };
        let light = unsafe { gl.GetUniformLocation(default.0, "light\0".as_ptr() as *const _) };
        let model = unsafe { gl.GetUniformLocation(default.0, "model\0".as_ptr() as *const _) };

        unsafe {
            let tex = gl.GetUniformLocation(default.0, "tex\0".as_ptr() as *const _);
            gl.Uniform1i(tex, 0);
        }
        Programs {
            default,
            camera,
            light,
            model,
        }
    }
    fn drop_with_gl(&mut self, gl: &Gles2) {
        Program::drop_with_gl(&mut self.default, gl);
    }
}

struct Vao(u32);

impl Vao {
    fn new(gl: &Gles2) -> Vao {
        let mut handles = [0];
        unsafe {
            gl.GenVertexArraysOES(1, &mut handles as *mut [u32; 1] as *mut _);
        }
        Vao(handles[0])
    }
    fn drop_with_gl(&mut self, gl: &Gles2) {
        let handles = [self.0];
        unsafe {
            gl.DeleteVertexArraysOES(1, &handles as *const _);
        }
    }
}

struct Buffer(u32);

impl Buffer {
    fn new(gl: &Gles2) -> Buffer {
        let mut handles = [0];
        unsafe {
            gl.GenBuffers(1, &mut handles as *mut [u32; 1] as *mut _);
        }
        Buffer(handles[0])
    }
    fn drop_with_gl(&mut self, gl: &Gles2) {
        let handles = [self.0];
        unsafe {
            gl.DeleteBuffers(1, &handles as *const _);
        }
    }
}

pub struct Texture(u32);
