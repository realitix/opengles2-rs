#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::os::raw::c_void;

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


// Types
pub type Clampf = sys::GLclampf;
pub type Bitfield = sys::GLbitfield;
pub type Int = sys::GLint;
pub type Sizei = sys::GLsizei;
pub type Enum = sys::GLenum;
pub type Void = sys::GLvoid;
pub type Uint = sys::GLuint;
pub type Char = sys::GLchar;

// Constant
pub const COLOR_BUFFER_BIT: Bitfield = sys::GL_COLOR_BUFFER_BIT;
pub const RGBA: Enum = sys::GL_RGBA;
pub const UNSIGNED_BYTE: Enum = sys::GL_UNSIGNED_BYTE;
pub const VERTEX_SHADER: Enum = sys::GL_VERTEX_SHADER;
pub const FRAGMENT_SHADER: Enum = sys::GL_FRAGMENT_SHADER;
pub const COMPILE_STATUS: Enum = sys::GL_COMPILE_STATUS;
pub const TRUE: Enum = sys::GL_TRUE;

// Functions
pub fn clear_color(red: Clampf, green: Clampf, blue: Clampf, alpha: Clampf) {
    unsafe { sys::glClearColor(red, green, blue, alpha) };
}

pub fn clear(mask: Bitfield) {
    unsafe { sys::glClear(mask) };
}

pub fn read_pixels(x: Int, y: Int, width: Sizei, height: Sizei, format: Enum, _type: Enum) -> Vec<u8> {
    let channels = 4;
    let data_size = (width * height * channels) as usize;
    let mut data: Vec<u8> = Vec::with_capacity(data_size);

    unsafe {
        sys::glReadPixels(x, y, width, height, format, _type, data[..].as_mut_ptr() as *mut c_void );
        data.set_len(data_size);
    }

    data
}

pub fn create_program() -> Uint {
    unsafe { sys::glCreateProgram() }
}

pub fn create_shader(shader_type: Enum) -> Uint {
    unsafe { sys::glCreateShader(shader_type) }
}

pub fn shader_source(shader: Uint, source: &str) {
    let lines = source.lines();
    let count = source.lines().count();
    let mut strings = Vec::<*const Char>::with_capacity(count);
    let mut lengths = Vec::<Int>::with_capacity(count);

    for line in lines {
        let string = line;
        let length = line.len();

        strings.push(string.as_bytes().as_ptr() as *const Char);
        lengths.push(length as Int);
    }

    unsafe {
        sys::glShaderSource(shader, count as Sizei, strings.as_slice().as_ptr(), lengths.as_slice().as_ptr());
    }
}

pub fn compile_shader(shader: Uint) {
    unsafe { sys::glCompileShader(shader) }
}

pub fn get_shaderiv(shader: Uint, pname: Enum) -> Int {
    let mut params = 0;
    unsafe { sys::glGetShaderiv(shader, pname, &mut params) };
    params
}

pub fn get_shader_info_log(shader: Uint) -> String {
    let max_length = 2048;
    let mut length = 0;
    let mut infolog: Char = 0;
    let result;

    unsafe {
        sys::glGetShaderInfoLog(shader, max_length, &mut length, &mut infolog);

        let mut ptr = infolog as u8;
        result = String::from_raw_parts(&mut ptr, length as usize, length as usize);
    }

    result
}

pub fn delete_shader(shader: Uint) {
    unsafe { sys::glDeleteShader(shader) }
}

pub fn attach_shader(program: Uint, shader: Uint) {
    unsafe { sys::glAttachShader(program, shader) }
}

pub fn link_program(program: Uint) {
    unsafe { sys::glLinkProgram(program) }
}