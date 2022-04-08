#[macro_use]
extern crate glium;

mod marching_squares;
mod meta_ball;
mod shader;
mod vertex;


const BALLS: &'static[meta_ball::MetaBall; 3] = &[meta_ball::MetaBall {
    radius: 1.0,
    position: [0.5, 0.5],
}, meta_ball::MetaBall {
    radius: 1.0,
    position: [-0.35, 0.5],
}, meta_ball::MetaBall {
    radius: 0.5,
    position: [0.0, -0.5],
}];

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program = shader::build_shader(&display);

    let mut shape = vec![];

    let grid_size = 50;
    let s = 2.0 / grid_size as f32;

    let mut handles = vec![];

    for fx in 0..grid_size {
        for fy in 0..grid_size {
            handles.push(std::thread::spawn(move || {
                let x = -1.0 + 2.0 / grid_size as f32 * (fx as f32);
                let y = -1.0 + 2.0 / grid_size as f32 * (fy as f32);
                return marching_squares::march_at(x, y, s, s, BALLS);
            }));
        }
    }

    for handle in handles {
        let lines = handle.join().unwrap();
        for line in lines {
            shape.push(line[0]);
            shape.push(line[1]);
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

    event_loop.run(move |ev, _, control_flow| {
        // Render
        let mut target = display.draw();
        target.clear_color(0.01, 0.01, 0.01, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // Handle events
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_6667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => {}
            }
            _ => return,
        }
    });
}
