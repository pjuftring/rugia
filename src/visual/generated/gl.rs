
        mod __gl_imports {
            pub use std::mem;
            pub use std::marker::Send;
            pub use std::os::raw;
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum,
                                                 gltype: GLenum,
                                                 id: GLuint,
                                                 severity: GLenum,
                                                 length: GLsizei,
                                                 message: *const GLchar,
                                                 userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint,
                                                    category: GLenum,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;

}
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_POINT_SIZE_RANGE: types::GLenum = 0x846D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BITS: types::GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BITS: types::GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_KHR: types::GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_DEBUG_BIT_KHR: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_FUNCTION_KHR: types::GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_CALLBACK_USER_PARAM_KHR: types::GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_GROUP_STACK_DEPTH_KHR: types::GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_LOGGED_MESSAGES_KHR: types::GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: types::GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_KHR: types::GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_OUTPUT_SYNCHRONOUS_KHR: types::GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_HIGH_KHR: types::GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_LOW_KHR: types::GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_MEDIUM_KHR: types::GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SEVERITY_NOTIFICATION_KHR: types::GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_API_KHR: types::GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_APPLICATION_KHR: types::GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_OTHER_KHR: types::GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_SHADER_COMPILER_KHR: types::GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_THIRD_PARTY_KHR: types::GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_SOURCE_WINDOW_SYSTEM_KHR: types::GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: types::GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_ERROR_KHR: types::GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_MARKER_KHR: types::GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_OTHER_KHR: types::GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PERFORMANCE_KHR: types::GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_POP_GROUP_KHR: types::GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PORTABILITY_KHR: types::GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_PUSH_GROUP_KHR: types::GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: types::GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BITS: types::GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DISPLAY_LIST: types::GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED: types::GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: types::GLenum = 0x8CD9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GENERATE_MIPMAP_HINT: types::GLenum = 0x8192;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BITS: types::GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_FLOAT: types::GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)] pub const HIGH_INT: types::GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_FORMAT: types::GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)] pub const IMPLEMENTATION_COLOR_READ_TYPE: types::GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_FLOAT: types::GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOW_INT: types::GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE: types::GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_ALPHA: types::GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_GROUP_STACK_DEPTH_KHR: types::GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_LOGGED_MESSAGES_KHR: types::GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEBUG_MESSAGE_LENGTH_KHR: types::GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_VECTORS: types::GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LABEL_LENGTH_KHR: types::GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_VECTORS: types::GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_VECTORS: types::GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_FLOAT: types::GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)] pub const MEDIUM_INT: types::GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_SHADER_BINARY_FORMATS: types::GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_KHR: types::GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_PIPELINE_KHR: types::GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_KHR: types::GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BITS: types::GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB565: types::GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_KHR: types::GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_BINARY_FORMATS: types::GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_COMPILER: types::GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_KHR: types::GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW_KHR: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW_KHR: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BITS: types::GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING_OES: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_KHR: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;

        #[allow(dead_code, missing_copy_implementations)]
        #[derive(Clone)]
        pub struct FnPtr {
            /// The function pointer that will be used when calling the function.
            f: *const __gl_imports::raw::c_void,
            /// True if the pointer points to a real function, false if points to a `panic!` fn.
            is_loaded: bool,
        }

        impl FnPtr {
            /// Creates a `FnPtr` from a load attempt.
            fn new(ptr: *const __gl_imports::raw::c_void) -> FnPtr {
                if ptr.is_null() {
                    FnPtr {
                        f: missing_fn_panic as *const __gl_imports::raw::c_void,
                        is_loaded: false
                    }
                } else {
                    FnPtr { f: ptr, is_loaded: true }
                }
            }

            /// Returns `true` if the function has been successfully loaded.
            ///
            /// If it returns `false`, calling the corresponding function will fail.
            #[inline]
            #[allow(dead_code)]
            pub fn is_loaded(&self) -> bool {
                self.is_loaded
            }
        }
    
#[inline(never)]
        fn missing_fn_panic() -> ! {
            panic!("gles2 function was not loaded")
        }

        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone)]
        pub struct Gles2 {
pub ActiveTexture: FnPtr,
pub AttachShader: FnPtr,
pub BindAttribLocation: FnPtr,
pub BindBuffer: FnPtr,
pub BindFramebuffer: FnPtr,
pub BindRenderbuffer: FnPtr,
pub BindTexture: FnPtr,
pub BindVertexArrayOES: FnPtr,
pub BlendColor: FnPtr,
pub BlendEquation: FnPtr,
pub BlendEquationSeparate: FnPtr,
pub BlendFunc: FnPtr,
pub BlendFuncSeparate: FnPtr,
pub BufferData: FnPtr,
pub BufferSubData: FnPtr,
pub CheckFramebufferStatus: FnPtr,
pub Clear: FnPtr,
pub ClearColor: FnPtr,
pub ClearDepthf: FnPtr,
pub ClearStencil: FnPtr,
pub ColorMask: FnPtr,
pub CompileShader: FnPtr,
pub CompressedTexImage2D: FnPtr,
pub CompressedTexSubImage2D: FnPtr,
pub CopyTexImage2D: FnPtr,
pub CopyTexSubImage2D: FnPtr,
pub CreateProgram: FnPtr,
pub CreateShader: FnPtr,
pub CullFace: FnPtr,
pub DebugMessageCallback: FnPtr,
pub DebugMessageCallbackKHR: FnPtr,
pub DebugMessageControl: FnPtr,
pub DebugMessageControlKHR: FnPtr,
pub DebugMessageInsert: FnPtr,
pub DebugMessageInsertKHR: FnPtr,
pub DeleteBuffers: FnPtr,
pub DeleteFramebuffers: FnPtr,
pub DeleteProgram: FnPtr,
pub DeleteRenderbuffers: FnPtr,
pub DeleteShader: FnPtr,
pub DeleteTextures: FnPtr,
pub DeleteVertexArraysOES: FnPtr,
pub DepthFunc: FnPtr,
pub DepthMask: FnPtr,
pub DepthRangef: FnPtr,
pub DetachShader: FnPtr,
pub Disable: FnPtr,
pub DisableVertexAttribArray: FnPtr,
pub DrawArrays: FnPtr,
pub DrawElements: FnPtr,
pub Enable: FnPtr,
pub EnableVertexAttribArray: FnPtr,
pub Finish: FnPtr,
pub Flush: FnPtr,
pub FramebufferRenderbuffer: FnPtr,
pub FramebufferTexture2D: FnPtr,
pub FrontFace: FnPtr,
pub GenBuffers: FnPtr,
pub GenFramebuffers: FnPtr,
pub GenRenderbuffers: FnPtr,
pub GenTextures: FnPtr,
pub GenVertexArraysOES: FnPtr,
pub GenerateMipmap: FnPtr,
pub GetActiveAttrib: FnPtr,
pub GetActiveUniform: FnPtr,
pub GetAttachedShaders: FnPtr,
pub GetAttribLocation: FnPtr,
pub GetBooleanv: FnPtr,
pub GetBufferParameteriv: FnPtr,
pub GetDebugMessageLog: FnPtr,
pub GetDebugMessageLogKHR: FnPtr,
pub GetError: FnPtr,
pub GetFloatv: FnPtr,
pub GetFramebufferAttachmentParameteriv: FnPtr,
pub GetIntegerv: FnPtr,
pub GetObjectLabel: FnPtr,
pub GetObjectLabelKHR: FnPtr,
pub GetObjectPtrLabel: FnPtr,
pub GetObjectPtrLabelKHR: FnPtr,
pub GetPointerv: FnPtr,
pub GetPointervKHR: FnPtr,
pub GetProgramInfoLog: FnPtr,
pub GetProgramiv: FnPtr,
pub GetRenderbufferParameteriv: FnPtr,
pub GetShaderInfoLog: FnPtr,
pub GetShaderPrecisionFormat: FnPtr,
pub GetShaderSource: FnPtr,
pub GetShaderiv: FnPtr,
pub GetString: FnPtr,
pub GetTexParameterfv: FnPtr,
pub GetTexParameteriv: FnPtr,
pub GetUniformLocation: FnPtr,
pub GetUniformfv: FnPtr,
pub GetUniformiv: FnPtr,
pub GetVertexAttribPointerv: FnPtr,
pub GetVertexAttribfv: FnPtr,
pub GetVertexAttribiv: FnPtr,
pub Hint: FnPtr,
pub IsBuffer: FnPtr,
pub IsEnabled: FnPtr,
pub IsFramebuffer: FnPtr,
pub IsProgram: FnPtr,
pub IsRenderbuffer: FnPtr,
pub IsShader: FnPtr,
pub IsTexture: FnPtr,
pub IsVertexArrayOES: FnPtr,
pub LineWidth: FnPtr,
pub LinkProgram: FnPtr,
pub ObjectLabel: FnPtr,
pub ObjectLabelKHR: FnPtr,
pub ObjectPtrLabel: FnPtr,
pub ObjectPtrLabelKHR: FnPtr,
pub PixelStorei: FnPtr,
pub PolygonOffset: FnPtr,
pub PopDebugGroup: FnPtr,
pub PopDebugGroupKHR: FnPtr,
pub PushDebugGroup: FnPtr,
pub PushDebugGroupKHR: FnPtr,
pub ReadPixels: FnPtr,
pub ReleaseShaderCompiler: FnPtr,
pub RenderbufferStorage: FnPtr,
pub SampleCoverage: FnPtr,
pub Scissor: FnPtr,
pub ShaderBinary: FnPtr,
pub ShaderSource: FnPtr,
pub StencilFunc: FnPtr,
pub StencilFuncSeparate: FnPtr,
pub StencilMask: FnPtr,
pub StencilMaskSeparate: FnPtr,
pub StencilOp: FnPtr,
pub StencilOpSeparate: FnPtr,
pub TexImage2D: FnPtr,
pub TexParameterf: FnPtr,
pub TexParameterfv: FnPtr,
pub TexParameteri: FnPtr,
pub TexParameteriv: FnPtr,
pub TexSubImage2D: FnPtr,
pub Uniform1f: FnPtr,
pub Uniform1fv: FnPtr,
pub Uniform1i: FnPtr,
pub Uniform1iv: FnPtr,
pub Uniform2f: FnPtr,
pub Uniform2fv: FnPtr,
pub Uniform2i: FnPtr,
pub Uniform2iv: FnPtr,
pub Uniform3f: FnPtr,
pub Uniform3fv: FnPtr,
pub Uniform3i: FnPtr,
pub Uniform3iv: FnPtr,
pub Uniform4f: FnPtr,
pub Uniform4fv: FnPtr,
pub Uniform4i: FnPtr,
pub Uniform4iv: FnPtr,
pub UniformMatrix2fv: FnPtr,
pub UniformMatrix3fv: FnPtr,
pub UniformMatrix4fv: FnPtr,
pub UseProgram: FnPtr,
pub ValidateProgram: FnPtr,
pub VertexAttrib1f: FnPtr,
pub VertexAttrib1fv: FnPtr,
pub VertexAttrib2f: FnPtr,
pub VertexAttrib2fv: FnPtr,
pub VertexAttrib3f: FnPtr,
pub VertexAttrib3fv: FnPtr,
pub VertexAttrib4f: FnPtr,
pub VertexAttrib4fv: FnPtr,
pub VertexAttribPointer: FnPtr,
pub Viewport: FnPtr,
_priv: ()
}
impl Gles2 {
            /// Load each OpenGL symbol using a custom load function. This allows for the
            /// use of functions like `glfwGetProcAddress` or `SDL_GL_GetProcAddress`.
            ///
            /// ~~~ignore
            /// let gl = Gl::load_with(|s| glfw.get_proc_address(s));
            /// ~~~
            #[allow(dead_code, unused_variables)]
            pub fn load_with<F>(mut loadfn: F) -> Gles2 where F: FnMut(&'static str) -> *const __gl_imports::raw::c_void {
                #[inline(never)]
                fn do_metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const __gl_imports::raw::c_void,
                                 symbol: &'static str,
                                 symbols: &[&'static str])
                                 -> *const __gl_imports::raw::c_void {
                    let mut ptr = loadfn(symbol);
                    if ptr.is_null() {
                        for &sym in symbols {
                            ptr = loadfn(sym);
                            if !ptr.is_null() { break; }
                        }
                    }
                    ptr
                }
                let mut metaloadfn = |symbol: &'static str, symbols: &[&'static str]| {
                    do_metaloadfn(&mut loadfn, symbol, symbols)
                };
                Gles2 {
ActiveTexture: FnPtr::new(metaloadfn("glActiveTexture", &[])),
AttachShader: FnPtr::new(metaloadfn("glAttachShader", &[])),
BindAttribLocation: FnPtr::new(metaloadfn("glBindAttribLocation", &[])),
BindBuffer: FnPtr::new(metaloadfn("glBindBuffer", &[])),
BindFramebuffer: FnPtr::new(metaloadfn("glBindFramebuffer", &[])),
BindRenderbuffer: FnPtr::new(metaloadfn("glBindRenderbuffer", &[])),
BindTexture: FnPtr::new(metaloadfn("glBindTexture", &[])),
BindVertexArrayOES: FnPtr::new(metaloadfn("glBindVertexArrayOES", &[])),
BlendColor: FnPtr::new(metaloadfn("glBlendColor", &[])),
BlendEquation: FnPtr::new(metaloadfn("glBlendEquation", &[])),
BlendEquationSeparate: FnPtr::new(metaloadfn("glBlendEquationSeparate", &[])),
BlendFunc: FnPtr::new(metaloadfn("glBlendFunc", &[])),
BlendFuncSeparate: FnPtr::new(metaloadfn("glBlendFuncSeparate", &[])),
BufferData: FnPtr::new(metaloadfn("glBufferData", &[])),
BufferSubData: FnPtr::new(metaloadfn("glBufferSubData", &[])),
CheckFramebufferStatus: FnPtr::new(metaloadfn("glCheckFramebufferStatus", &[])),
Clear: FnPtr::new(metaloadfn("glClear", &[])),
ClearColor: FnPtr::new(metaloadfn("glClearColor", &[])),
ClearDepthf: FnPtr::new(metaloadfn("glClearDepthf", &[])),
ClearStencil: FnPtr::new(metaloadfn("glClearStencil", &[])),
ColorMask: FnPtr::new(metaloadfn("glColorMask", &[])),
CompileShader: FnPtr::new(metaloadfn("glCompileShader", &[])),
CompressedTexImage2D: FnPtr::new(metaloadfn("glCompressedTexImage2D", &[])),
CompressedTexSubImage2D: FnPtr::new(metaloadfn("glCompressedTexSubImage2D", &[])),
CopyTexImage2D: FnPtr::new(metaloadfn("glCopyTexImage2D", &[])),
CopyTexSubImage2D: FnPtr::new(metaloadfn("glCopyTexSubImage2D", &[])),
CreateProgram: FnPtr::new(metaloadfn("glCreateProgram", &[])),
CreateShader: FnPtr::new(metaloadfn("glCreateShader", &[])),
CullFace: FnPtr::new(metaloadfn("glCullFace", &[])),
DebugMessageCallback: FnPtr::new(metaloadfn("glDebugMessageCallback", &[])),
DebugMessageCallbackKHR: FnPtr::new(metaloadfn("glDebugMessageCallbackKHR", &[])),
DebugMessageControl: FnPtr::new(metaloadfn("glDebugMessageControl", &[])),
DebugMessageControlKHR: FnPtr::new(metaloadfn("glDebugMessageControlKHR", &[])),
DebugMessageInsert: FnPtr::new(metaloadfn("glDebugMessageInsert", &[])),
DebugMessageInsertKHR: FnPtr::new(metaloadfn("glDebugMessageInsertKHR", &[])),
DeleteBuffers: FnPtr::new(metaloadfn("glDeleteBuffers", &[])),
DeleteFramebuffers: FnPtr::new(metaloadfn("glDeleteFramebuffers", &[])),
DeleteProgram: FnPtr::new(metaloadfn("glDeleteProgram", &[])),
DeleteRenderbuffers: FnPtr::new(metaloadfn("glDeleteRenderbuffers", &[])),
DeleteShader: FnPtr::new(metaloadfn("glDeleteShader", &[])),
DeleteTextures: FnPtr::new(metaloadfn("glDeleteTextures", &[])),
DeleteVertexArraysOES: FnPtr::new(metaloadfn("glDeleteVertexArraysOES", &[])),
DepthFunc: FnPtr::new(metaloadfn("glDepthFunc", &[])),
DepthMask: FnPtr::new(metaloadfn("glDepthMask", &[])),
DepthRangef: FnPtr::new(metaloadfn("glDepthRangef", &[])),
DetachShader: FnPtr::new(metaloadfn("glDetachShader", &[])),
Disable: FnPtr::new(metaloadfn("glDisable", &[])),
DisableVertexAttribArray: FnPtr::new(metaloadfn("glDisableVertexAttribArray", &[])),
DrawArrays: FnPtr::new(metaloadfn("glDrawArrays", &[])),
DrawElements: FnPtr::new(metaloadfn("glDrawElements", &[])),
Enable: FnPtr::new(metaloadfn("glEnable", &[])),
EnableVertexAttribArray: FnPtr::new(metaloadfn("glEnableVertexAttribArray", &[])),
Finish: FnPtr::new(metaloadfn("glFinish", &[])),
Flush: FnPtr::new(metaloadfn("glFlush", &[])),
FramebufferRenderbuffer: FnPtr::new(metaloadfn("glFramebufferRenderbuffer", &[])),
FramebufferTexture2D: FnPtr::new(metaloadfn("glFramebufferTexture2D", &[])),
FrontFace: FnPtr::new(metaloadfn("glFrontFace", &[])),
GenBuffers: FnPtr::new(metaloadfn("glGenBuffers", &[])),
GenFramebuffers: FnPtr::new(metaloadfn("glGenFramebuffers", &[])),
GenRenderbuffers: FnPtr::new(metaloadfn("glGenRenderbuffers", &[])),
GenTextures: FnPtr::new(metaloadfn("glGenTextures", &[])),
GenVertexArraysOES: FnPtr::new(metaloadfn("glGenVertexArraysOES", &[])),
GenerateMipmap: FnPtr::new(metaloadfn("glGenerateMipmap", &[])),
GetActiveAttrib: FnPtr::new(metaloadfn("glGetActiveAttrib", &[])),
GetActiveUniform: FnPtr::new(metaloadfn("glGetActiveUniform", &[])),
GetAttachedShaders: FnPtr::new(metaloadfn("glGetAttachedShaders", &[])),
GetAttribLocation: FnPtr::new(metaloadfn("glGetAttribLocation", &[])),
GetBooleanv: FnPtr::new(metaloadfn("glGetBooleanv", &[])),
GetBufferParameteriv: FnPtr::new(metaloadfn("glGetBufferParameteriv", &[])),
GetDebugMessageLog: FnPtr::new(metaloadfn("glGetDebugMessageLog", &[])),
GetDebugMessageLogKHR: FnPtr::new(metaloadfn("glGetDebugMessageLogKHR", &[])),
GetError: FnPtr::new(metaloadfn("glGetError", &[])),
GetFloatv: FnPtr::new(metaloadfn("glGetFloatv", &[])),
GetFramebufferAttachmentParameteriv: FnPtr::new(metaloadfn("glGetFramebufferAttachmentParameteriv", &[])),
GetIntegerv: FnPtr::new(metaloadfn("glGetIntegerv", &[])),
GetObjectLabel: FnPtr::new(metaloadfn("glGetObjectLabel", &[])),
GetObjectLabelKHR: FnPtr::new(metaloadfn("glGetObjectLabelKHR", &[])),
GetObjectPtrLabel: FnPtr::new(metaloadfn("glGetObjectPtrLabel", &[])),
GetObjectPtrLabelKHR: FnPtr::new(metaloadfn("glGetObjectPtrLabelKHR", &[])),
GetPointerv: FnPtr::new(metaloadfn("glGetPointerv", &[])),
GetPointervKHR: FnPtr::new(metaloadfn("glGetPointervKHR", &[])),
GetProgramInfoLog: FnPtr::new(metaloadfn("glGetProgramInfoLog", &[])),
GetProgramiv: FnPtr::new(metaloadfn("glGetProgramiv", &[])),
GetRenderbufferParameteriv: FnPtr::new(metaloadfn("glGetRenderbufferParameteriv", &[])),
GetShaderInfoLog: FnPtr::new(metaloadfn("glGetShaderInfoLog", &[])),
GetShaderPrecisionFormat: FnPtr::new(metaloadfn("glGetShaderPrecisionFormat", &[])),
GetShaderSource: FnPtr::new(metaloadfn("glGetShaderSource", &[])),
GetShaderiv: FnPtr::new(metaloadfn("glGetShaderiv", &[])),
GetString: FnPtr::new(metaloadfn("glGetString", &[])),
GetTexParameterfv: FnPtr::new(metaloadfn("glGetTexParameterfv", &[])),
GetTexParameteriv: FnPtr::new(metaloadfn("glGetTexParameteriv", &[])),
GetUniformLocation: FnPtr::new(metaloadfn("glGetUniformLocation", &[])),
GetUniformfv: FnPtr::new(metaloadfn("glGetUniformfv", &[])),
GetUniformiv: FnPtr::new(metaloadfn("glGetUniformiv", &[])),
GetVertexAttribPointerv: FnPtr::new(metaloadfn("glGetVertexAttribPointerv", &[])),
GetVertexAttribfv: FnPtr::new(metaloadfn("glGetVertexAttribfv", &[])),
GetVertexAttribiv: FnPtr::new(metaloadfn("glGetVertexAttribiv", &[])),
Hint: FnPtr::new(metaloadfn("glHint", &[])),
IsBuffer: FnPtr::new(metaloadfn("glIsBuffer", &[])),
IsEnabled: FnPtr::new(metaloadfn("glIsEnabled", &[])),
IsFramebuffer: FnPtr::new(metaloadfn("glIsFramebuffer", &[])),
IsProgram: FnPtr::new(metaloadfn("glIsProgram", &[])),
IsRenderbuffer: FnPtr::new(metaloadfn("glIsRenderbuffer", &[])),
IsShader: FnPtr::new(metaloadfn("glIsShader", &[])),
IsTexture: FnPtr::new(metaloadfn("glIsTexture", &[])),
IsVertexArrayOES: FnPtr::new(metaloadfn("glIsVertexArrayOES", &[])),
LineWidth: FnPtr::new(metaloadfn("glLineWidth", &[])),
LinkProgram: FnPtr::new(metaloadfn("glLinkProgram", &[])),
ObjectLabel: FnPtr::new(metaloadfn("glObjectLabel", &[])),
ObjectLabelKHR: FnPtr::new(metaloadfn("glObjectLabelKHR", &[])),
ObjectPtrLabel: FnPtr::new(metaloadfn("glObjectPtrLabel", &[])),
ObjectPtrLabelKHR: FnPtr::new(metaloadfn("glObjectPtrLabelKHR", &[])),
PixelStorei: FnPtr::new(metaloadfn("glPixelStorei", &[])),
PolygonOffset: FnPtr::new(metaloadfn("glPolygonOffset", &[])),
PopDebugGroup: FnPtr::new(metaloadfn("glPopDebugGroup", &[])),
PopDebugGroupKHR: FnPtr::new(metaloadfn("glPopDebugGroupKHR", &[])),
PushDebugGroup: FnPtr::new(metaloadfn("glPushDebugGroup", &[])),
PushDebugGroupKHR: FnPtr::new(metaloadfn("glPushDebugGroupKHR", &[])),
ReadPixels: FnPtr::new(metaloadfn("glReadPixels", &[])),
ReleaseShaderCompiler: FnPtr::new(metaloadfn("glReleaseShaderCompiler", &[])),
RenderbufferStorage: FnPtr::new(metaloadfn("glRenderbufferStorage", &[])),
SampleCoverage: FnPtr::new(metaloadfn("glSampleCoverage", &[])),
Scissor: FnPtr::new(metaloadfn("glScissor", &[])),
ShaderBinary: FnPtr::new(metaloadfn("glShaderBinary", &[])),
ShaderSource: FnPtr::new(metaloadfn("glShaderSource", &[])),
StencilFunc: FnPtr::new(metaloadfn("glStencilFunc", &[])),
StencilFuncSeparate: FnPtr::new(metaloadfn("glStencilFuncSeparate", &[])),
StencilMask: FnPtr::new(metaloadfn("glStencilMask", &[])),
StencilMaskSeparate: FnPtr::new(metaloadfn("glStencilMaskSeparate", &[])),
StencilOp: FnPtr::new(metaloadfn("glStencilOp", &[])),
StencilOpSeparate: FnPtr::new(metaloadfn("glStencilOpSeparate", &[])),
TexImage2D: FnPtr::new(metaloadfn("glTexImage2D", &[])),
TexParameterf: FnPtr::new(metaloadfn("glTexParameterf", &[])),
TexParameterfv: FnPtr::new(metaloadfn("glTexParameterfv", &[])),
TexParameteri: FnPtr::new(metaloadfn("glTexParameteri", &[])),
TexParameteriv: FnPtr::new(metaloadfn("glTexParameteriv", &[])),
TexSubImage2D: FnPtr::new(metaloadfn("glTexSubImage2D", &[])),
Uniform1f: FnPtr::new(metaloadfn("glUniform1f", &[])),
Uniform1fv: FnPtr::new(metaloadfn("glUniform1fv", &[])),
Uniform1i: FnPtr::new(metaloadfn("glUniform1i", &[])),
Uniform1iv: FnPtr::new(metaloadfn("glUniform1iv", &[])),
Uniform2f: FnPtr::new(metaloadfn("glUniform2f", &[])),
Uniform2fv: FnPtr::new(metaloadfn("glUniform2fv", &[])),
Uniform2i: FnPtr::new(metaloadfn("glUniform2i", &[])),
Uniform2iv: FnPtr::new(metaloadfn("glUniform2iv", &[])),
Uniform3f: FnPtr::new(metaloadfn("glUniform3f", &[])),
Uniform3fv: FnPtr::new(metaloadfn("glUniform3fv", &[])),
Uniform3i: FnPtr::new(metaloadfn("glUniform3i", &[])),
Uniform3iv: FnPtr::new(metaloadfn("glUniform3iv", &[])),
Uniform4f: FnPtr::new(metaloadfn("glUniform4f", &[])),
Uniform4fv: FnPtr::new(metaloadfn("glUniform4fv", &[])),
Uniform4i: FnPtr::new(metaloadfn("glUniform4i", &[])),
Uniform4iv: FnPtr::new(metaloadfn("glUniform4iv", &[])),
UniformMatrix2fv: FnPtr::new(metaloadfn("glUniformMatrix2fv", &[])),
UniformMatrix3fv: FnPtr::new(metaloadfn("glUniformMatrix3fv", &[])),
UniformMatrix4fv: FnPtr::new(metaloadfn("glUniformMatrix4fv", &[])),
UseProgram: FnPtr::new(metaloadfn("glUseProgram", &[])),
ValidateProgram: FnPtr::new(metaloadfn("glValidateProgram", &[])),
VertexAttrib1f: FnPtr::new(metaloadfn("glVertexAttrib1f", &[])),
VertexAttrib1fv: FnPtr::new(metaloadfn("glVertexAttrib1fv", &[])),
VertexAttrib2f: FnPtr::new(metaloadfn("glVertexAttrib2f", &[])),
VertexAttrib2fv: FnPtr::new(metaloadfn("glVertexAttrib2fv", &[])),
VertexAttrib3f: FnPtr::new(metaloadfn("glVertexAttrib3f", &[])),
VertexAttrib3fv: FnPtr::new(metaloadfn("glVertexAttrib3fv", &[])),
VertexAttrib4f: FnPtr::new(metaloadfn("glVertexAttrib4f", &[])),
VertexAttrib4fv: FnPtr::new(metaloadfn("glVertexAttrib4fv", &[])),
VertexAttribPointer: FnPtr::new(metaloadfn("glVertexAttribPointer", &[])),
Viewport: FnPtr::new(metaloadfn("glViewport", &[])),
_priv: ()
}
        }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ActiveTexture(&self, texture: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.ActiveTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn AttachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.AttachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindAttribLocation(&self, program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, *const types::GLchar) -> ()>(self.BindAttribLocation.f)(program, index, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindBuffer(&self, target: types::GLenum, buffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindBuffer.f)(target, buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindFramebuffer(&self, target: types::GLenum, framebuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindFramebuffer.f)(target, framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindRenderbuffer(&self, target: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindRenderbuffer.f)(target, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindTexture(&self, target: types::GLenum, texture: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.BindTexture.f)(target, texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BindVertexArrayOES(&self, array: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.BindVertexArrayOES.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.BlendColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquation(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.BlendEquation.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendEquationSeparate(&self, modeRGB: types::GLenum, modeAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendEquationSeparate.f)(modeRGB, modeAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFunc(&self, sfactor: types::GLenum, dfactor: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.BlendFunc.f)(sfactor, dfactor) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BlendFuncSeparate(&self, sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.BlendFuncSeparate.f)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferData(&self, target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizeiptr, *const __gl_imports::raw::c_void, types::GLenum) -> ()>(self.BufferData.f)(target, size, data, usage) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn BufferSubData(&self, target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLintptr, types::GLsizeiptr, *const __gl_imports::raw::c_void) -> ()>(self.BufferSubData.f)(target, offset, size, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CheckFramebufferStatus(&self, target: types::GLenum) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLenum>(self.CheckFramebufferStatus.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Clear(&self, mask: types::GLbitfield) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLbitfield) -> ()>(self.Clear.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearColor(&self, red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.ClearColor.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearDepthf(&self, d: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.ClearDepthf.f)(d) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ClearStencil(&self, s: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint) -> ()>(self.ClearStencil.f)(s) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ColorMask(&self, red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean, types::GLboolean, types::GLboolean, types::GLboolean) -> ()>(self.ColorMask.f)(red, green, blue, alpha) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompileShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.CompileShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLsizei, types::GLsizei, types::GLint, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexImage2D.f)(target, level, internalformat, width, height, border, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CompressedTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.CompressedTexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, imageSize, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint) -> ()>(self.CopyTexImage2D.f)(target, level, internalformat, x, y, width, height, border) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CopyTexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.CopyTexSubImage2D.f)(target, level, xoffset, yoffset, x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateProgram(&self, ) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLuint>(self.CreateProgram.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CreateShader(&self, type_: types::GLenum) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLuint>(self.CreateShader.f)(type_) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn CullFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.CullFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageCallback(&self, callback: types::GLDEBUGPROC, userParam: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLDEBUGPROC, *const __gl_imports::raw::c_void) -> ()>(self.DebugMessageCallback.f)(callback, userParam) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageCallbackKHR(&self, callback: types::GLDEBUGPROCKHR, userParam: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLDEBUGPROCKHR, *const __gl_imports::raw::c_void) -> ()>(self.DebugMessageCallbackKHR.f)(callback, userParam) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageControl(&self, source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *const types::GLuint, types::GLboolean) -> ()>(self.DebugMessageControl.f)(source, type_, severity, count, ids, enabled) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageControlKHR(&self, source: types::GLenum, type_: types::GLenum, severity: types::GLenum, count: types::GLsizei, ids: *const types::GLuint, enabled: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLsizei, *const types::GLuint, types::GLboolean) -> ()>(self.DebugMessageControlKHR.f)(source, type_, severity, count, ids, enabled) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageInsert(&self, source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLenum, types::GLsizei, *const types::GLchar) -> ()>(self.DebugMessageInsert.f)(source, type_, id, severity, length, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DebugMessageInsertKHR(&self, source: types::GLenum, type_: types::GLenum, id: types::GLuint, severity: types::GLenum, length: types::GLsizei, buf: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLuint, types::GLenum, types::GLsizei, *const types::GLchar) -> ()>(self.DebugMessageInsertKHR.f)(source, type_, id, severity, length, buf) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteBuffers(&self, n: types::GLsizei, buffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteFramebuffers(&self, n: types::GLsizei, framebuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DeleteProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteRenderbuffers(&self, n: types::GLsizei, renderbuffers: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteShader(&self, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DeleteShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteTextures(&self, n: types::GLsizei, textures: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DeleteVertexArraysOES(&self, n: types::GLsizei, arrays: *const types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint) -> ()>(self.DeleteVertexArraysOES.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthFunc(&self, func: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.DepthFunc.f)(func) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthMask(&self, flag: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLboolean) -> ()>(self.DepthMask.f)(flag) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DepthRangef(&self, n: types::GLfloat, f: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.DepthRangef.f)(n, f) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DetachShader(&self, program: types::GLuint, shader: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint) -> ()>(self.DetachShader.f)(program, shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Disable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Disable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DisableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.DisableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawArrays(&self, mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLsizei) -> ()>(self.DrawArrays.f)(mode, first, count) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn DrawElements(&self, mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLsizei, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.DrawElements.f)(mode, count, type_, indices) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Enable(&self, cap: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.Enable.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn EnableVertexAttribArray(&self, index: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.EnableVertexAttribArray.f)(index) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Finish(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Finish.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Flush(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.Flush.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferRenderbuffer(&self, target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint) -> ()>(self.FramebufferRenderbuffer.f)(target, attachment, renderbuffertarget, renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FramebufferTexture2D(&self, target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLuint, types::GLint) -> ()>(self.FramebufferTexture2D.f)(target, attachment, textarget, texture, level) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn FrontFace(&self, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.FrontFace.f)(mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenBuffers(&self, n: types::GLsizei, buffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenBuffers.f)(n, buffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenFramebuffers(&self, n: types::GLsizei, framebuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenFramebuffers.f)(n, framebuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenRenderbuffers(&self, n: types::GLsizei, renderbuffers: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenRenderbuffers.f)(n, renderbuffers) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenTextures(&self, n: types::GLsizei, textures: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenTextures.f)(n, textures) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenVertexArraysOES(&self, n: types::GLsizei, arrays: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *mut types::GLuint) -> ()>(self.GenVertexArraysOES.f)(n, arrays) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GenerateMipmap(&self, target: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> ()>(self.GenerateMipmap.f)(target) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveAttrib(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(self.GetActiveAttrib.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetActiveUniform(&self, program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLint, *mut types::GLenum, *mut types::GLchar) -> ()>(self.GetActiveUniform.f)(program, index, bufSize, length, size, type_, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttachedShaders(&self, program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLuint) -> ()>(self.GetAttachedShaders.f)(program, maxCount, count, shaders) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetAttribLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetAttribLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBooleanv(&self, pname: types::GLenum, data: *mut types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLboolean) -> ()>(self.GetBooleanv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetBufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetBufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDebugMessageLog(&self, count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLenum, *mut types::GLenum, *mut types::GLuint, *mut types::GLenum, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(self.GetDebugMessageLog.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetDebugMessageLogKHR(&self, count: types::GLuint, bufSize: types::GLsizei, sources: *mut types::GLenum, types: *mut types::GLenum, ids: *mut types::GLuint, severities: *mut types::GLenum, lengths: *mut types::GLsizei, messageLog: *mut types::GLchar) -> types::GLuint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLenum, *mut types::GLenum, *mut types::GLuint, *mut types::GLenum, *mut types::GLsizei, *mut types::GLchar) -> types::GLuint>(self.GetDebugMessageLogKHR.f)(count, bufSize, sources, types, ids, severities, lengths, messageLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetError(&self, ) -> types::GLenum { __gl_imports::mem::transmute::<_, extern "system" fn() -> types::GLenum>(self.GetError.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFloatv(&self, pname: types::GLenum, data: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLfloat) -> ()>(self.GetFloatv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetFramebufferAttachmentParameteriv(&self, target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetFramebufferAttachmentParameteriv.f)(target, attachment, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetIntegerv(&self, pname: types::GLenum, data: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *mut types::GLint) -> ()>(self.GetIntegerv.f)(pname, data) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectLabel(&self, identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectLabel.f)(identifier, name, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectLabelKHR(&self, identifier: types::GLenum, name: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectLabelKHR.f)(identifier, name, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectPtrLabel.f)(ptr, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetObjectPtrLabelKHR(&self, ptr: *const __gl_imports::raw::c_void, bufSize: types::GLsizei, length: *mut types::GLsizei, label: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetObjectPtrLabelKHR.f)(ptr, bufSize, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetPointerv(&self, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetPointerv.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetPointervKHR(&self, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetPointervKHR.f)(pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramInfoLog(&self, program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetProgramInfoLog.f)(program, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetProgramiv(&self, program: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetProgramiv.f)(program, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetRenderbufferParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetRenderbufferParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderInfoLog(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetShaderInfoLog.f)(shader, bufSize, length, infoLog) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderPrecisionFormat(&self, shadertype: types::GLenum, precisiontype: types::GLenum, range: *mut types::GLint, precision: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint, *mut types::GLint) -> ()>(self.GetShaderPrecisionFormat.f)(shadertype, precisiontype, range, precision) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderSource(&self, shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *mut types::GLsizei, *mut types::GLchar) -> ()>(self.GetShaderSource.f)(shader, bufSize, length, source) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetShaderiv(&self, shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetShaderiv.f)(shader, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetString(&self, name: types::GLenum) -> *const types::GLubyte { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> *const types::GLubyte>(self.GetString.f)(name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLfloat) -> ()>(self.GetTexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetTexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *mut types::GLint) -> ()>(self.GetTexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformLocation(&self, program: types::GLuint, name: *const types::GLchar) -> types::GLint { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLchar) -> types::GLint>(self.GetUniformLocation.f)(program, name) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformfv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLfloat) -> ()>(self.GetUniformfv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetUniformiv(&self, program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, *mut types::GLint) -> ()>(self.GetUniformiv.f)(program, location, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribPointerv(&self, index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *const *mut __gl_imports::raw::c_void) -> ()>(self.GetVertexAttribPointerv.f)(index, pname, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribfv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLfloat) -> ()>(self.GetVertexAttribfv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn GetVertexAttribiv(&self, index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLenum, *mut types::GLint) -> ()>(self.GetVertexAttribiv.f)(index, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Hint(&self, target: types::GLenum, mode: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum) -> ()>(self.Hint.f)(target, mode) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsBuffer(&self, buffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsBuffer.f)(buffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsEnabled(&self, cap: types::GLenum) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum) -> types::GLboolean>(self.IsEnabled.f)(cap) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsFramebuffer(&self, framebuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsFramebuffer.f)(framebuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsProgram(&self, program: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsRenderbuffer(&self, renderbuffer: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsRenderbuffer.f)(renderbuffer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsShader(&self, shader: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsShader.f)(shader) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsTexture(&self, texture: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsTexture.f)(texture) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn IsVertexArrayOES(&self, array: types::GLuint) -> types::GLboolean { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> types::GLboolean>(self.IsVertexArrayOES.f)(array) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LineWidth(&self, width: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat) -> ()>(self.LineWidth.f)(width) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn LinkProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.LinkProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectLabel(&self, identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectLabel.f)(identifier, name, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectLabelKHR(&self, identifier: types::GLenum, name: types::GLuint, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectLabelKHR.f)(identifier, name, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectPtrLabel(&self, ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectPtrLabel.f)(ptr, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ObjectPtrLabelKHR(&self, ptr: *const __gl_imports::raw::c_void, length: types::GLsizei, label: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(*const __gl_imports::raw::c_void, types::GLsizei, *const types::GLchar) -> ()>(self.ObjectPtrLabelKHR.f)(ptr, length, label) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PixelStorei(&self, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint) -> ()>(self.PixelStorei.f)(pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PolygonOffset(&self, factor: types::GLfloat, units: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLfloat) -> ()>(self.PolygonOffset.f)(factor, units) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PopDebugGroup(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopDebugGroup.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PopDebugGroupKHR(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.PopDebugGroupKHR.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PushDebugGroup(&self, source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.PushDebugGroup.f)(source, id, length, message) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn PushDebugGroupKHR(&self, source: types::GLenum, id: types::GLuint, length: types::GLsizei, message: *const types::GLchar) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint, types::GLsizei, *const types::GLchar) -> ()>(self.PushDebugGroupKHR.f)(source, id, length, message) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReadPixels(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *mut __gl_imports::raw::c_void) -> ()>(self.ReadPixels.f)(x, y, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ReleaseShaderCompiler(&self, ) -> () { __gl_imports::mem::transmute::<_, extern "system" fn() -> ()>(self.ReleaseShaderCompiler.f)() }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn RenderbufferStorage(&self, target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLsizei, types::GLsizei) -> ()>(self.RenderbufferStorage.f)(target, internalformat, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn SampleCoverage(&self, value: types::GLfloat, invert: types::GLboolean) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLfloat, types::GLboolean) -> ()>(self.SampleCoverage.f)(value, invert) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Scissor(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Scissor.f)(x, y, width, height) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderBinary(&self, count: types::GLsizei, shaders: *const types::GLuint, binaryformat: types::GLenum, binary: *const __gl_imports::raw::c_void, length: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLsizei, *const types::GLuint, types::GLenum, *const __gl_imports::raw::c_void, types::GLsizei) -> ()>(self.ShaderBinary.f)(count, shaders, binaryformat, binary, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ShaderSource(&self, shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLsizei, *const *const types::GLchar, *const types::GLint) -> ()>(self.ShaderSource.f)(shader, count, string, length) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFunc(&self, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFunc.f)(func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilFuncSeparate(&self, face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint, types::GLuint) -> ()>(self.StencilFuncSeparate.f)(face, func, ref_, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMask(&self, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.StencilMask.f)(mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilMaskSeparate(&self, face: types::GLenum, mask: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLuint) -> ()>(self.StencilMaskSeparate.f)(face, mask) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOp(&self, fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOp.f)(fail, zfail, zpass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn StencilOpSeparate(&self, face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLenum, types::GLenum) -> ()>(self.StencilOpSeparate.f)(face, sfail, dpfail, dppass) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexImage2D(&self, target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLint, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexImage2D.f)(target, level, internalformat, width, height, border, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterf(&self, target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLfloat) -> ()>(self.TexParameterf.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameterfv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLfloat) -> ()>(self.TexParameterfv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteri(&self, target: types::GLenum, pname: types::GLenum, param: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, types::GLint) -> ()>(self.TexParameteri.f)(target, pname, param) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexParameteriv(&self, target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLenum, *const types::GLint) -> ()>(self.TexParameteriv.f)(target, pname, params) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn TexSubImage2D(&self, target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLenum, types::GLint, types::GLint, types::GLint, types::GLsizei, types::GLsizei, types::GLenum, types::GLenum, *const __gl_imports::raw::c_void) -> ()>(self.TexSubImage2D.f)(target, level, xoffset, yoffset, width, height, format, type_, pixels) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1f(&self, location: types::GLint, v0: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat) -> ()>(self.Uniform1f.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform1fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1i(&self, location: types::GLint, v0: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint) -> ()>(self.Uniform1i.f)(location, v0) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform1iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform1iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat) -> ()>(self.Uniform2f.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform2fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform2i.f)(location, v0, v1) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform2iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform2iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Uniform3f.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform3fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform3i.f)(location, v0, v1, v2) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform3iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform3iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4f(&self, location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.Uniform4f.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4fv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLfloat) -> ()>(self.Uniform4fv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4i(&self, location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLint, types::GLint, types::GLint) -> ()>(self.Uniform4i.f)(location, v0, v1, v2, v3) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Uniform4iv(&self, location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, *const types::GLint) -> ()>(self.Uniform4iv.f)(location, count, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix2fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix2fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix3fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix3fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UniformMatrix4fv(&self, location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLsizei, types::GLboolean, *const types::GLfloat) -> ()>(self.UniformMatrix4fv.f)(location, count, transpose, value) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn UseProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.UseProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn ValidateProgram(&self, program: types::GLuint) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint) -> ()>(self.ValidateProgram.f)(program) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1f(&self, index: types::GLuint, x: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat) -> ()>(self.VertexAttrib1f.f)(index, x) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib1fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib1fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib2f.f)(index, x, y) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib2fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib2fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib3f.f)(index, x, y, z) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib3fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib3fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4f(&self, index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLfloat, types::GLfloat, types::GLfloat, types::GLfloat) -> ()>(self.VertexAttrib4f.f)(index, x, y, z, w) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttrib4fv(&self, index: types::GLuint, v: *const types::GLfloat) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, *const types::GLfloat) -> ()>(self.VertexAttrib4fv.f)(index, v) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn VertexAttribPointer(&self, index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLuint, types::GLint, types::GLenum, types::GLboolean, types::GLsizei, *const __gl_imports::raw::c_void) -> ()>(self.VertexAttribPointer.f)(index, size, type_, normalized, stride, pointer) }
#[allow(non_snake_case, unused_variables, dead_code)]
            #[inline] pub unsafe fn Viewport(&self, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> () { __gl_imports::mem::transmute::<_, extern "system" fn(types::GLint, types::GLint, types::GLsizei, types::GLsizei) -> ()>(self.Viewport.f)(x, y, width, height) }
}

        unsafe impl __gl_imports::Send for Gles2 {}
