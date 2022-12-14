mod open_gl;

use std::f32;
use std::time;
use std::path::Path;
use glutin;
use glutin::window;
use glutin::event;
use glutin::event_loop;
use glutin::dpi;
use glutin::monitor;
use gl;
use cgmath::{ Matrix4, InnerSpace, Vector2, vec2, Vector3, vec3 };
use open_gl::{ ShaderProgram, ViewPort, Camera };

fn main() {
    // Создание окна
    let event_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_visible(true)
        .with_inner_size(dpi::LogicalSize::new(800, 600))
        .with_min_inner_size(dpi::LogicalSize::new(400, 300))
        .with_resizable(true)
        .with_title("Rust Marching");
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // windowed_context.window().set_cursor_grab(true).unwrap();
    // windowed_context.window().set_cursor_visible(false);

    let fullscreen = window::Fullscreen::Exclusive(prompt_for_video_mode(
        &prompt_for_monitor(&event_loop)));

    // Создание контекста OpenGl
    let gl_context = windowed_context.context();
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    
    // Пути к файлам шейдеров
    let vert_filename = Path::new("Shaders/main.vert").to_str().unwrap();
    let frag_filename = Path::new("Shaders/main.frag").to_str().unwrap();

    // Загрузка и компиляция шейдеров
    let shader_loadresult = ShaderProgram::from_files(
        vert_filename, frag_filename);
    let shader_program = match shader_loadresult {
        Ok(program) => program,
        Err(err) => { println!("{}", err); return }
    };


    // // ----- Модели ------
    // let mut render_objects: Vec<RenderObject> = vec![];

    // // ----- Светильники ------
    // let mut light_objects: Vec<Light> = vec![];

    
    
    let mut view_port = ViewPort::new();
    let mut camera = Camera::new();
    camera.set_position(vec3(0.0, 0.0, -5.0));
    camera.set_direction(vec3(0.0, 0.0, 1.0));

    let now = time::Instant::now();
    let mut old_since_time = now.elapsed().as_millis();

    let mut window_is_focused = true;
    let mut is_fullscreen = false;




    let sensitivity = 0.07;
    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut delta_x = 0.0;
    let mut delta_y = 0.0;
    let mut zoom = 0.0;




    // let mut draw_mode = 0;
    // let mut is_look_at = false;

    // let mut max_normal_speed = 0.60;
    // let mut max_fast_speed = max_normal_speed * 5.00;
    // let mut current_max_speed = max_normal_speed;
    // let mut normal_speed_step = max_normal_speed / 100.0;
    // let mut fast_speed_step = max_fast_speed / 100.0;
    // let mut current_speed_step = normal_speed_step;
    // let mut speed = 0.0;
    
    let mut forward = false;
    let mut back = false;
    let mut left = false;
    let mut right = false;
    let mut up = false;
    let mut down = false;

    let mut camera_up = false;
    let mut camera_down = false;
    let mut camera_left = false;
    let mut camera_right = false;
    let mut camera_closer = false;
    let mut camera_father = false;
    let mut camera_speed = 1.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = event_loop::ControlFlow::Poll;
        match event {
            event::Event::LoopDestroyed => return,
            event::Event::WindowEvent { event, .. } => {
                match event {
                    event::WindowEvent::CloseRequested =>
                        *control_flow = event_loop::ControlFlow::Exit,
                    event::WindowEvent::Resized(physical_size) => unsafe {
                        let view_width = windowed_context.window().inner_size().width as i32;
                        let view_height = windowed_context.window().inner_size().height as i32;
                        view_port.set_position((0, 0));
                        view_port.set_size((view_width, view_height));
                    },
                    event::WindowEvent::Focused(is_focus) => 
                        window_is_focused = is_focus,
                    _ => ()
                }
            },
            event::Event::DeviceEvent { event, .. } => {
                if (window_is_focused) {
                    match event {
                        event::DeviceEvent::Key(keyboard_input) => match keyboard_input {
                            event::KeyboardInput { scancode: 1, state: event::ElementState::Released, .. } => 
                                *control_flow = event_loop::ControlFlow::Exit,
                            // event::KeyboardInput { scancode: 15, state: event::ElementState::Released, .. } => 
                            //     draw_mode = (draw_mode + 1) % 4,
                            event::KeyboardInput { scancode: 28, state: event::ElementState::Released, .. } => 
                            {
                                if !is_fullscreen {
                                    windowed_context.window().set_cursor_grab(true).unwrap();
                                    windowed_context.window().set_cursor_visible(false);
                                    windowed_context.window().set_fullscreen(Some(fullscreen.clone()));
                                    is_fullscreen = true;
                                }
                                else {
                                    windowed_context.window().set_cursor_grab(false).unwrap();
                                    windowed_context.window().set_cursor_visible(true);
                                    windowed_context.window().set_fullscreen(None);
                                    is_fullscreen = false;
                                }
                            },
                            // event::KeyboardInput { scancode: 25, state: event::ElementState::Released, ..} =>
                            // {
                            //     if camera.is_ortho() { camera.set_is_ortho(false); }
                            //     else { camera.set_is_ortho(true); }
                            // },

                            // event::KeyboardInput { scancode: 38, state: event::ElementState::Released, .. } => 
                            //     is_look_at = !is_look_at,

                            event::KeyboardInput { scancode: 17, state: event::ElementState::Released, ..} =>
                                forward = false,
                            event::KeyboardInput { scancode: 31, state: event::ElementState::Released, ..} =>
                                back = false,
                            event::KeyboardInput { scancode: 30, state: event::ElementState::Released, ..} =>
                                left = false,
                            event::KeyboardInput { scancode: 32, state: event::ElementState::Released, ..} =>
                                right = false,
                            event::KeyboardInput { scancode: 29, state: event::ElementState::Released, ..} =>
                                down = false,
                            event::KeyboardInput { scancode: 57, state: event::ElementState::Released, ..} =>
                                up = false,
                            // event::KeyboardInput { scancode: 42, state: event::ElementState::Released, ..} =>
                            //     { 
                            //         current_speed_step = normal_speed_step; 
                            //         current_max_speed = max_normal_speed;
                            //     },
                            event::KeyboardInput { scancode: 72, state: event::ElementState::Released, ..} =>
                                camera_up = false,
                            event::KeyboardInput { scancode: 80, state: event::ElementState::Released, ..} =>
                                camera_down = false,
                            event::KeyboardInput { scancode: 75, state: event::ElementState::Released, ..} =>
                                camera_left = false,
                            event::KeyboardInput { scancode: 77, state: event::ElementState::Released, ..} =>
                                camera_right = false,
                            event::KeyboardInput { scancode: 83, state: event::ElementState::Released, ..} =>
                                camera_father = false,
                            event::KeyboardInput { scancode: 82, state: event::ElementState::Released, ..} =>
                                camera_closer = false,
                            // event::KeyboardInput { scancode: 46, state: event::ElementState::Released, ..} =>
                            //     camera.set_target(camera.position()),

                            event::KeyboardInput { scancode: 17, state: event::ElementState::Pressed, ..} =>
                                forward = true,
                            event::KeyboardInput { scancode: 31, state: event::ElementState::Pressed, ..} =>
                                back = true,
                            event::KeyboardInput { scancode: 30, state: event::ElementState::Pressed, ..} =>
                                left = true,
                            event::KeyboardInput { scancode: 32, state: event::ElementState::Pressed, ..} =>
                                right = true,
                            event::KeyboardInput { scancode: 29, state: event::ElementState::Pressed, ..} =>
                                down = true,
                            event::KeyboardInput { scancode: 57, state: event::ElementState::Pressed, ..} =>
                                up = true,
                            // event::KeyboardInput { scancode: 13, state: event::ElementState::Pressed, ..} =>
                            //     {
                            //         max_normal_speed += 0.05;
                            //         max_fast_speed = max_normal_speed * 5.0;
                            //         current_max_speed = max_normal_speed;
                            //         normal_speed_step = max_normal_speed / 100.0;
                            //         fast_speed_step = max_fast_speed / 100.0;
                            //         current_speed_step = normal_speed_step;
                            //     }
                            // event::KeyboardInput { scancode: 12, state: event::ElementState::Pressed, ..} =>
                            //     {
                            //         max_normal_speed -= 0.05;
                            //         if max_normal_speed < 0.05 { max_normal_speed = 0.0 }
                            //         max_fast_speed = max_normal_speed * 5.0;
                            //         current_max_speed = max_normal_speed;
                            //         normal_speed_step = max_normal_speed / 100.0;
                            //         fast_speed_step = max_fast_speed / 100.0;
                            //         current_speed_step = normal_speed_step;
                            //     },
                            // event::KeyboardInput { scancode: 42, state: event::ElementState::Pressed, ..} =>
                            //     {
                            //         current_speed_step = fast_speed_step;
                            //         current_max_speed = max_fast_speed;
                            //     },
                            event::KeyboardInput { scancode: 72, state: event::ElementState::Pressed, ..} =>
                                camera_up = true,
                            event::KeyboardInput { scancode: 80, state: event::ElementState::Pressed, ..} =>
                                camera_down = true,
                            event::KeyboardInput { scancode: 75, state: event::ElementState::Pressed, ..} =>
                                camera_left = true,
                            event::KeyboardInput { scancode: 77, state: event::ElementState::Pressed, ..} =>
                                camera_right = true,
                            event::KeyboardInput { scancode: 83, state: event::ElementState::Pressed, ..} =>
                                camera_father = true,
                            event::KeyboardInput { scancode: 82, state: event::ElementState::Pressed, ..} =>
                                camera_closer = true,
                            // event::KeyboardInput { scancode: 78, state: event::ElementState::Pressed, ..} =>
                            //     camera_speed += 0.1,
                            // event::KeyboardInput { scancode: 74, state: event::ElementState::Pressed, ..} =>
                            //     { camera_speed -= 0.1; if camera_speed < 0.0 { camera_speed = 0.0 } },
                            // event::KeyboardInput { scancode: 55, state: event::ElementState::Pressed, ..} =>
                            //     camera.set_field_of_view(camera.field_of_view() + 0.5),
                            // event::KeyboardInput { scancode: 57397, state: event::ElementState::Pressed, ..} =>
                            //     if camera.field_of_view() > 0.5 { camera.set_field_of_view(camera.field_of_view() - 0.5) },


                            //event::KeyboardInput { scancode, state, .. } => println!("{:?} {:?}", scancode, state),
                            _ => ()
                        },

                        event::DeviceEvent::MouseMotion { delta } =>
                        {
                            delta_x = delta.0;
                            delta_y = delta.1;
                            //println!("{} {}", delta_x, delta_y);
                        },
                        event::DeviceEvent::MouseWheel { delta } => match delta {
                            event::MouseScrollDelta::LineDelta(_, y) => 
                                zoom += y / 10.0,
                            _ => (),
                        },
                        _ => ()
                    }
                }
            }
            event::Event::MainEventsCleared => {
                // Дельта времени
                let since_time = now.elapsed().as_millis();
                let delta_time = (since_time - old_since_time) as f64;
                
                if delta_time > (1000.0 / 30.0) {
                    old_since_time = since_time;
                    
                    if camera_up { delta_y = -camera_speed as f64; }
                    if camera_down { delta_y = camera_speed as f64; }
                    if camera_left { delta_x = -camera_speed as f64; }
                    if camera_right { delta_x = camera_speed as f64; }

                    let offset_x = delta_x * sensitivity * delta_time;
                    let offset_y = delta_y * sensitivity * delta_time;
                    delta_x = 0.0;
                    delta_y = 0.0;
                    yaw += offset_x as f32;
                    pitch += -5.0 * offset_y as f32;

                    if forward {
                        let matrix = Matrix4::from_translation(-camera.direction() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }
                    if back {
                        let matrix = Matrix4::from_translation(camera.direction() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }
                    if right {
                        let matrix = Matrix4::from_translation(camera.right() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }
                    if left {
                        let matrix = Matrix4::from_translation(-camera.right() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }
                    if up {
                        let matrix = Matrix4::from_translation(camera.up() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }
                    if down {
                        let matrix = Matrix4::from_translation(-camera.up() * 0.01 * delta_time as f32);
                        camera.set_position((matrix * camera.position().extend(1.0)).truncate());
                    }

                    let radians_yaw = yaw.to_radians();
                    let radians_pitch = pitch.to_radians();
                    let direct_x = radians_yaw.cos() * radians_pitch.cos();
                    let direct_y = radians_pitch.sin();
                    let direct_z = radians_yaw.sin() * radians_pitch.cos();

                    let direction = vec3(direct_x, direct_y, direct_z).normalize();
                    camera.set_direction(direction);

                    let view_port_size = vec2(
                        view_port.size().0 as f32, 
                        view_port.size().1 as f32
                    );
                    let mouse_position = vec2(yaw, pitch);
                    
                    shader_program.use_();
                    shader_program.set_uniform_vector2("screen_resolution", &view_port_size);
                    shader_program.set_uniform_vector3("camera.position", &camera.position());
                    shader_program.set_uniform_vector3("camera.direction", &camera.direction());
                    shader_program.set_uniform_float("camera.field_of_view", camera.field_of_view());


                    shader_program.set_uniform_vector2("u_mouse", &mouse_position);
                    shader_program.set_uniform_float("u_time", old_since_time as f32 / 1000.0);
                    shader_program.set_uniform_float("u_zoom", zoom);
                    
                    view_port.draw();
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => (),
        }
    });
}

fn prompt_for_monitor(event_loop: &event_loop::EventLoop<()>) -> monitor::MonitorHandle {
    event_loop.available_monitors().nth(0).unwrap()
}

fn prompt_for_video_mode(monitor: &monitor::MonitorHandle) -> monitor::VideoMode {
    monitor.video_modes().nth(0).unwrap()
}
