use std::{mem, ptr};
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use monolith::custom_errors::Errors;
use monolith::graphics::window::Window;
use monolith::graphics::gl_wrapper::*;

fn main() {
    let mut window = Window::new(1080, 720, "TestWindow");

    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, -0.5, -0.0,
    ];

    let indices = [0, 1, 3, 1, 2, 3];

    window.init_gl();

    let vao = monolith::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();

    ibo.store_i32_data(&indices);


    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null());

    let index_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null()
    );

    position_attribute.enable();
    index_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}

fn error_test(num: i32) -> Result<(), Errors> {
    if num == 1 {
        monolith::logger::info!("Error");
        return Err(Errors::TestError.into());
    }
    Ok(())
}