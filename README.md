# Rugia

My personal game engine written in [Rust](https://www.rust-lang.org/) using [GLFW 3](https://www.glfw.org/) and [OpenGL ES 2.0](https://www.khronos.org/registry/OpenGL-Refpages/es2.0/).

## Goals

This is mostly for playing around and having a project for learning Rust. In the future, I would like to use it for games that can run on not-so-great hardware. For this, I am testing everything regularly on my [Raspberry Pi 4 Model B](https://www.raspberrypi.org/products/raspberry-pi-4-model-b/). It seems to run pretty well. I am using OpenGL ES 2.0 so I can target [ANGLE](https://github.com/google/angle) for Windows because that seems to improve performance on my ancient Surface Pro 2 from 2014. In the future, I might write a new backend in [Vulkan](https://www.vulkan.org/) but this does not work on my Surface an the performance on my Raspberry Pi was pretty bad (the last time I checked).

## Building

For building, you need Rust, which you can get [here](https://rustup.rs/). Use `git clone https://github.com/pjuftring/rugia` in order to clone the project and run `cargo run`. This might produce some errors because of missing libraries. Continue as follows:

### Linux (Debian based)

You need to install `libglfw3-dev` and `libwebp-dev`, which you can do by typing
```
sudo apt install libglfw3-dev libwebp-dev
```
into your favorite shell. Now, everything should work fine.

### Windows

For Windows, I use the `x86_64-pc-windows-gnu` target. Everything should also work for the `msvc`-based one but I did not test that.

You need to put `glfw3dll.lib`, `glfw3.dll`, and `webp.lib` in the `rugia` directory. You can get the first two from [this](https://www.glfw.org/download.html) page (look for "64-bit Windows binaries"), and the third one from [here](), just follow the link to the *downloads repository* and download `libwebp-1.2.1-windows-x64.zip` or something like that.

If you already have them or these files live at another location, you have to modify the lines in `build.rs` commented as *Windows search path for libaries*.

### FreeBSD

You need to install `glfw` and `webp`, which you can do by typing
```
sudo pkg install glfw webp
```
into your favorite shell. Now, everything should work fine.

## Dependencies

For generating the GLFW and WebP bindings, I use [bindgen](https://crates.io/crates/bindgen), and for the OpenGL ES 2.0 bindings, I use [gl_generator](https://crates.io/crates/gl_generator).

## License

Everything is licensed under the MIT License (cf. the `LICENSE` file).
