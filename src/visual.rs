mod generated;
mod gl;
mod glfw;
mod vector;
mod webp;

use crate::visual::vector::Vec3;
use crate::visual::webp::WebP;
use gl::Gl;
use glfw::Glfw;
use glfw::Window;
use std::cell::Cell;
use std::convert::TryInto;
use std::rc::Rc;
use vector::Mat4;

use self::gl::Texture;

#[derive(Clone)]
pub struct Model(Rc<ModelInternal>);

impl Model {
    pub fn new_instance(&self) -> ModelInstance {
        self.0.new_instance()
    }
}

#[derive(Clone)]
pub struct ModelInstance(Rc<Cell<Spatial>>);

impl ModelInstance {
    pub fn with_spatial<F>(&self, f: F)
    where
        F: FnOnce(&mut Spatial),
    {
        let mut spatial = self.0.take();
        f(&mut spatial);
        self.0.set(spatial);
    }
}

pub struct ModelInternal {
    arrays: Vec<[i16; 6]>,
    elements: Vec<u16>,

    texture: Option<Rc<Texture>>,
    instances: Cell<Vec<ModelInstance>>,
    dirty: Cell<bool>,
}

impl ModelInternal {
    fn new(arrays_src: &[u8], elements_src: &[u8], texture: Option<Rc<Texture>>) -> ModelInternal {
        let arrays_flat: Vec<i16> = arrays_src
            .chunks_exact(2)
            .map(|bytes| i16::from_le_bytes(bytes.try_into().unwrap()))
            .collect();
        let arrays = arrays_flat
            .chunks_exact(5)
            .map(|shorts| {
                // because of alignment, we have the first 4 i16 as vertex data (value 0 is for alignment, not read)
                // and 2 u16 as uv data
                [shorts[0], shorts[1], shorts[2], 0, shorts[3], shorts[4]]
            })
            .collect();
        let elements = elements_src
            .chunks_exact(2)
            .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
            .collect();

        ModelInternal {
            arrays,
            elements,
            texture,
            instances: Cell::new(Vec::new()),
            dirty: Cell::new(true),
        }
    }
    fn new_instance(&self) -> ModelInstance {
        let instance = ModelInstance(Rc::new(Cell::new(Spatial::new())));
        let mut instances = self.instances.take();
        instances.push(instance.clone());
        self.instances.set(instances);
        self.dirty.set(true);
        instance
    }
}

pub struct Visual {
    glfw: Glfw,
    gl: Gl,
    window: Window,
    render_size: (i32, i32),

    models_static: Vec<Model>,
    texture_pool: Vec<Rc<Texture>>,
}

impl Visual {
    pub fn new() -> Self {
        let glfw = Glfw::new();
        let render_size = (800, 600);
        let window = glfw.new_window(render_size.0, render_size.1, "Good day");
        window.make_current();
        let gl = Gl::new(&glfw);

        Visual {
            glfw,
            gl,
            window,
            render_size,
            models_static: Vec::new(),
            texture_pool: Vec::new(),
        }
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn clear(&mut self, r: f32, g: f32, b: f32) {
        self.gl.clear_color(r, g, b, 1.0);
        self.gl.clear(true, true);
    }
    pub fn camera(&mut self, spatial: &Spatial, near: f32, far: f32, fov: f32) {
        let aspect = (self.render_size.0 as f32) / (self.render_size.1 as f32);
        self.gl.set_camera(
            Mat4::new()
                .perspective(near, far, fov, aspect)
                .mul(spatial.to_mat4_rev()),
        );
    }
    fn housekeep_textures(&mut self) {
        let texture_pool = &mut self.texture_pool;
        let gl = &mut self.gl;
        texture_pool.retain(|texture| {
            if Rc::strong_count(texture) > 1 {
                true
            } else {
                gl.drop_texture(texture);
                false
            }
        });
        texture_pool.shrink_to_fit();
    }
    fn housekeep_models_static(&mut self) -> bool {
        // Implementation is not very optimal, but it is not intended usage to add and remove models that much during execution.
        let mut dirty = false;
        self.models_static.retain(|model| {
            let no_instances;
            let mut instances = model.0.instances.take();
            if model.0.dirty.replace(false) {
                dirty = true; // if model is dirty, i.e. has just been created
            }
            instances.retain(|instance| {
                // retain if user has reference to instance
                // otherwise, remove it but do NOT set dirty bit
                Rc::strong_count(&instance.0) > 1
            });
            no_instances = instances.is_empty();
            model.0.instances.set(instances);
            if Rc::strong_count(&model.0) > 1 || !no_instances {
                true // retain if user has reference to model or there is an instance
            } else {
                dirty = true; // otherwise, remove and set dirty bit
                false
            }
        });
        // Housekeeping of model data should not happen that often, so we can always shrink if something gets removed here
        self.models_static.shrink_to_fit();
        if dirty {
            #[cfg(debug_assertions)]
            println!("WARNING: Dirty bit set of static models.");
        }
        dirty
    }
    pub fn swap_and_poll(&mut self) {
        if self.glfw.framebuffer_size_dirty() {
            self.render_size = self.window.get_rendersize();
        }

        let dirty = self.housekeep_models_static();
        self.housekeep_textures();
        self.gl.draw(&self.render_size, &self.models_static, dirty);

        self.window.swap();
        self.glfw.poll();
    }
    pub fn load_model(
        &mut self,
        arrays_src: &[u8],
        elements_src: &[u8],
        texture_src_option: Option<&[u8]>,
    ) -> Model {
        let texture = texture_src_option.map(|texture_src| {
            let webp = WebP::load_rgb(texture_src).expect("Could not load texture");
            let texture = Rc::new(self.gl.new_texture(webp));
            self.texture_pool.push(texture.clone());
            texture
        });

        let model = Model(Rc::new(ModelInternal::new(
            arrays_src,
            elements_src,
            texture,
        )));
        self.models_static.push(model.clone());
        model
    }
}

impl Drop for Visual {
    fn drop(&mut self) {
        let texture_pool = &mut self.texture_pool;
        let gl = &mut self.gl;
        texture_pool.iter().for_each(|texture| {
            gl.drop_texture(texture);
        });
    }
}

#[derive(Debug)]
pub struct Spatial {
    xyz: [f32; 3],
    pyr: [f32; 3],
}

impl Default for Spatial {
    fn default() -> Spatial {
        Spatial::new()
    }
}

impl Spatial {
    pub fn new() -> Self {
        Spatial {
            xyz: [0.; 3],
            pyr: [0.; 3],
        }
    }
    pub fn place(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.xyz = [x, y, z];
        self
    }
    /*pub fn pitch(&mut self, pitch: f32) -> &mut Self {
        self.pyr[0] = pitch;
        self
    }*/
    pub fn yaw(&mut self, yaw: f32) -> &mut Self {
        self.pyr[1] = yaw;
        self
    }
    /*pub fn roll(&mut self, roll: f32) -> &mut Self {
        self.pyr[2] = roll;
        self
    }
    pub fn spot(&mut self, other: &Spatial) -> &mut Self {
        self.yaw(f32::atan2(
            self.xyz[0] - other.xyz[0],
            self.xyz[2] - other.xyz[2],
        ));
        self.pitch(f32::atan2(
            other.xyz[1] - self.xyz[1],
            (self.xyz[2] - other.xyz[2]).hypot(self.xyz[0] - other.xyz[0]),
        ));
        self
    }*/
    pub fn to_mat4(&self) -> Mat4 {
        Mat4::new()
            .translate(Vec3::new(self.xyz))
            .rot_y(self.pyr[1])
            .rot_x(self.pyr[0])
            .rot_z(self.pyr[2])
    }
    pub fn to_mat4_rev(&self) -> Mat4 {
        Mat4::new()
            .rot_z(-self.pyr[2])
            .rot_x(-self.pyr[0])
            .rot_y(-self.pyr[1])
            .translate(Vec3::new([-self.xyz[0], -self.xyz[1], -self.xyz[2]]))
    }
}
