extern crate bindgen;
extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::fs::File;

pub fn main() {
    // Linking
    if cfg!(windows) {
        // Windows search path for libaries
        println!("cargo:rustc-link-search=.");
        println!("cargo:rustc-link-lib=glfw3");
        println!("cargo:rustc-link-lib=webp");
    } else { // unix
        if cfg!(target_os = "freebsd") {
            println!("cargo:rustc-link-search=/usr/local/lib");
        }
        println!("cargo:rustc-link-lib=glfw");
        println!("cargo:rustc-link-lib=webp");
    }
    
    // Rerun
    println!("cargo:rerun-if-changed=build.rs");
    
    // GLFW
    let bindings = bindgen::Builder::default()
        .header("src/visual/generated/glfw.h")
        .generate()
        .expect("Unable to generate GLFW bindings.");
    bindings
        .write_to_file("src/visual/generated/glfw.rs")
        .expect("Unable to write GLFW bindings.");
        
    // WebP
    let bindings = bindgen::Builder::default()
        .header("src/visual/generated/webp.h")
        .generate()
        .expect("Unable to generate WebP bindings.");
    bindings
        .write_to_file("src/visual/generated/webp.rs")
        .expect("Unable to write WebP bindings.");
    
    
    // OpenGL / GLES
    Registry::new(
        Api::Gles2,
        (2, 0),
        Profile::Core,
        Fallbacks::None,
        ["GL_KHR_debug", "GL_OES_vertex_array_object"],
    )
    .write_bindings(
        StructGenerator,
        &mut File::create("src/visual/generated/gl.rs")
            .expect("Unable to create file for GL bindings."),
    )
    .expect("Unable to write GL bindings.");
}
