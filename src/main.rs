extern crate glfw;
extern crate gl;

mod bubble;
mod selection;
mod quick;

use bubble::BubbleSort;
use selection::SelectionSort;

use glfw::{
    Action,
    Context,
    Key,
    OpenGlProfileHint,
    WindowEvent,
    WindowHint,
};
use rand::seq::SliceRandom;

trait Sort
{
    fn step(&mut self, array: &mut [u32]) -> bool;
    fn reset(&mut self);
    fn current_index(&self) -> u32;
}

fn main()
{
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors)
        .expect("Failed to initialise GLFW!");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(
            800, 600,
            "Bubble Sort",
            glfw::WindowMode::Windowed
        ).expect("Failed to create window!");
    window.make_current();

    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);

    gl::load_with(|s| glfw.get_proc_address_raw(s));
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    let vertices = [
        0.0, 0.0, 0.0f32,
        0.0, 1.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 1.0, 0.0,
    ];
    let indices = [
        0, 1, 2u32,
        2, 1, 3,
    ];

    let vao = unsafe {
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        vao
    };

    let vbo = unsafe {
        use std::mem;

        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            mem::size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );


        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3, gl::FLOAT, gl::FALSE,
            3 * mem::size_of::<f32>() as i32,
            0 as _
        );

        vbo
    };

    let ebo = unsafe {
        use std::mem;

        let mut ebo: u32 = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            mem::size_of_val(&indices) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        ebo
    };

    let shader = unsafe {
        use std::fs;
        use std::ffi::CString;
        use std::ptr;
        use std::str;

        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        {
            let vertex_shader_source =
                fs::read_to_string("shaders/shader.vert")
                    .expect("Failed to read shaders/shader.vert!");
            println!("==== Vertex shader\n{}", vertex_shader_source);

            let vertex_shader_source =
                CString::new(vertex_shader_source).unwrap();
            gl::ShaderSource(
                vertex_shader,
                1, &vertex_shader_source.as_ptr(), ptr::null()
            );
            gl::CompileShader(vertex_shader);

            let mut success: i32 = 0;
            gl::GetShaderiv(
                vertex_shader,
                gl::COMPILE_STATUS, &mut success
            );
            if success == 0
            {
                let mut info_log_len: i32 = 0;
                gl::GetShaderiv(
                    vertex_shader,
                    gl::INFO_LOG_LENGTH, &mut info_log_len
                );
                let info_log_len = info_log_len;

                let mut info_log: Vec<u8> =
                    Vec::with_capacity(info_log_len as usize);
                info_log.set_len(info_log_len as usize);
                gl::GetShaderInfoLog(
                    vertex_shader,
                    info_log_len,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8
                );

                let info_log = str::from_utf8(&info_log).unwrap();
                println!(
                    "==== Failed to compile vertex shader\n{}",
                    info_log
                );
            }
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        {
            let fragment_shader_source =
                fs::read_to_string("shaders/shader.frag")
                    .expect("Failed to read shaders/shader.frag!");
            println!("==== Fragment shader\n{}", fragment_shader_source);

            let fragment_shader_source =
                CString::new(fragment_shader_source).unwrap();
            gl::ShaderSource(
                fragment_shader,
                1, &fragment_shader_source.as_ptr(), ptr::null()
            );
            gl::CompileShader(fragment_shader);

            let mut success: i32 = 0;
            gl::GetShaderiv(
                fragment_shader,
                gl::COMPILE_STATUS, &mut success
            );
            if success == 0
            {
                let mut info_log_len: i32 = 0;
                gl::GetShaderiv(
                    fragment_shader,
                    gl::INFO_LOG_LENGTH, &mut info_log_len
                );
                let info_log_len = info_log_len;

                let mut info_log: Vec<u8> =
                    Vec::with_capacity(info_log_len as usize);
                info_log.set_len(info_log_len as usize);
                gl::GetShaderInfoLog(
                    fragment_shader,
                    info_log_len,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8
                );

                let info_log = str::from_utf8(&info_log).unwrap();
                println!(
                    "==== Failed to compile fragment shader\n{}",
                    info_log
                );
            }
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    };
    
    let u_gMaxCount = unsafe {
        use std::ffi::CString;
        let cstr = CString::new("gMaxCount").unwrap();
        gl::GetUniformLocation(
            shader,
            cstr.as_ptr()
        )
    };
    let u_gMaxValue = unsafe {
        use std::ffi::CString;
        let cstr = CString::new("gMaxValue").unwrap();
        gl::GetUniformLocation(
            shader,
            cstr.as_ptr()
        )
    };
    let u_gValue = unsafe {
        use std::ffi::CString;
        let cstr = CString::new("gValue").unwrap();
        gl::GetUniformLocation(
            shader,
            cstr.as_ptr()
        )
    };
    let u_gIndex = unsafe {
        use std::ffi::CString;
        let cstr = CString::new("gIndex").unwrap();
        gl::GetUniformLocation(
            shader,
            cstr.as_ptr()
        )
    };
    let u_gIndexHighlight = unsafe {
        use std::ffi::CString;
        let cstr = CString::new("gIndexHighlight").unwrap();
        gl::GetUniformLocation(
            shader,
            cstr.as_ptr()
        )
    };

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    const MAX_COUNT: u32 = 1000;
    let mut zahlen: Vec<u32> = Vec::with_capacity(MAX_COUNT as usize);
    for i in 0..1000
    {
        zahlen.push(i+1);
    }
    let mut rng = rand::thread_rng();

    let mut run_sort = false;
    let mut sorter: Box<dyn Sort> =
        Box::new(BubbleSort::new());

    const MAX_FPS: f64 = 120.0;
    const FRAME_TIME: f64 = 1.0 / MAX_FPS;
    const STEP_COUNT: u32 = 30;

    let mut prev_time = 0.0;

    while !window.should_close()
    {
        let now = glfw.get_time();
        let delta_time = now - prev_time;
        if delta_time < FRAME_TIME
        {
            continue;
        }
        prev_time = now;

        if run_sort
        {
            let mut sorted = false;
            for _ in 0..STEP_COUNT
            {
                sorted = sorter.step(&mut zahlen);
                if sorted
                {
                    break;
                }
            }

            run_sort = !sorted;
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader);
            gl::BindVertexArray(vao);
            gl::Uniform1ui(u_gMaxCount, MAX_COUNT);
            gl::Uniform1ui(u_gMaxValue, MAX_COUNT);
            gl::Uniform1ui(u_gIndexHighlight, sorter.current_index());

            for i in 0..zahlen.len()
            {
                gl::Uniform1ui(u_gValue, zahlen[i]);
                gl::Uniform1ui(u_gIndex, i as u32);

                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    0 as _
                );
            }

            gl::BindVertexArray(0);
        }

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events)
        {
            match event
            {
                WindowEvent::FramebufferSize(w, h) => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                },
                WindowEvent::Key(key, _, Action::Press, _) => {
                    if key == Key::Escape
                    {
                        window.set_should_close(true);
                    }
                    else if key == Key::S
                    {
                        zahlen.shuffle(&mut rng);
                        sorter.reset();
                        run_sort = false;
                    }
                    else if key == Key::Enter
                    {
                        run_sort = !run_sort;
                    }
                    else if key == Key::Num1
                    {
                        sorter = Box::new(BubbleSort::new());
                    }
                    else if key == Key::Num2
                    {
                        sorter = Box::new(SelectionSort::new());
                    }
                },
                _ => ()
            }
        }
    }

    unsafe {
        gl::DeleteProgram(shader);
        gl::DeleteBuffers(1, &ebo);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
