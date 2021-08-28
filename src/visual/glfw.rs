use std::ffi::{c_void, CString};
use std::os::raw::c_int;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::visual::generated::glfw;

static FRAMEBUFFER_SIZE_DIRTY: AtomicBool = AtomicBool::new(true);

struct GlfwInner;
pub struct Glfw(Rc<GlfwInner>);

pub struct Window {
    _glfw: Glfw,
    handle: *mut glfw::GLFWwindow,
}

impl Glfw {
    pub fn new() -> Self {
        unsafe {
            glfw::glfwInit();
            glfw::glfwWindowHint(glfw::GLFW_CLIENT_API as i32, glfw::GLFW_OPENGL_API as i32);
            /*glfw::glfwWindowHint(
                glfw::GLFW_CONTEXT_CREATION_API as i32,
                glfw::GLFW_NATIVE_CONTEXT_API as i32,
            );*/
            glfw::glfwWindowHint(glfw::GLFW_CONTEXT_VERSION_MAJOR as i32, 3);
            glfw::glfwWindowHint(glfw::GLFW_CONTEXT_VERSION_MINOR as i32, 0);
            glfw::glfwWindowHint(glfw::GLFW_DOUBLEBUFFER as i32, glfw::GLFW_TRUE as i32);

            glfw::glfwWindowHint(
                glfw::GLFW_OPENGL_FORWARD_COMPAT as i32,
                glfw::GLFW_TRUE as i32,
            );
            /*glfw::glfwWindowHint(
                glfw::GLFW_OPENGL_PROFILE as i32,
                glfw::GLFW_OPENGL_CORE_PROFILE as i32,
            );*/
        }
        #[cfg(debug_assertions)]
        unsafe {
            glfw::glfwWindowHint(
                glfw::GLFW_OPENGL_DEBUG_CONTEXT as i32,
                glfw::GLFW_TRUE as i32,
            );
        }
        Glfw(Rc::new(GlfwInner))
    }
    extern "C" fn framebuffer_size_callback(_window: *mut glfw::GLFWwindow, _: c_int, _: c_int) {
        FRAMEBUFFER_SIZE_DIRTY.store(true, Ordering::Relaxed);
    }
    pub fn framebuffer_size_dirty(&self) -> bool {
        FRAMEBUFFER_SIZE_DIRTY.swap(false, Ordering::Relaxed)
    }
    pub fn new_window(&self, width: i32, height: i32, title: &'static str) -> Window {
        let title_c = CString::new(title).expect("String conversion failed.");
        let handle = unsafe {
            glfw::glfwCreateWindow(width, height, title_c.as_ptr(), null_mut(), null_mut())
        };
        unsafe {
            glfw::glfwSetFramebufferSizeCallback(handle, Some(Glfw::framebuffer_size_callback));
        }
        Window {
            _glfw: self.clone(),
            handle,
        }
    }
    pub fn poll(&self) {
        unsafe {
            glfw::glfwPollEvents();
        }
    }
    pub fn get_loadfn(&self) -> impl FnMut(&'static str) -> *const c_void {
        |proc| {
            let proc_c = CString::new(proc).expect("String conversion failed.");
            unsafe {
                match glfw::glfwGetProcAddress(proc_c.as_ptr()) {
                    Some(address) => address as _,
                    None => null(),
                }
            }
        }
    }
}

impl Clone for Glfw {
    fn clone(&self) -> Self {
        Glfw(self.0.clone())
    }
}

impl Drop for GlfwInner {
    fn drop(&mut self) {
        unsafe {
            glfw::glfwTerminate();
        }
    }
}

impl Window {
    pub fn should_close(&self) -> bool {
        unsafe { glfw::glfwWindowShouldClose(self.handle) == glfw::GLFW_TRUE as c_int }
    }
    pub fn swap(&self) {
        unsafe {
            glfw::glfwSwapBuffers(self.handle);
        }
    }
    pub fn make_current(&self) {
        unsafe {
            glfw::glfwMakeContextCurrent(self.handle);
        }
    }
    pub fn get_rendersize(&self) -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            glfw::glfwGetFramebufferSize(self.handle, &mut width, &mut height);
        }
        (width, height)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            glfw::glfwDestroyWindow(self.handle);
        }
    }
}
