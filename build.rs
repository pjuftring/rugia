//extern crate bindgen;
//extern crate gl_generator;

//use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
//use std::fs::File;

pub fn main() {
    //println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=webp");

    /*let bindings = bindgen::Builder::default()
        .header("stuff/glfw.h")
        .generate()
        .expect("Unable to generate GLFW bindings.");
    bindings
        .write_to_file("src/visual/generated/glfw.rs")
        .expect("Unable to write GLFW bindings.");

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
    .expect("Unable to write GL bindings.");*/
}
