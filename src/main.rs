use pixels::{Pixels, SurfaceTexture};
use std::env;
use std::ops::Range;
use winit::{
    dpi::LogicalSize,
    event,
    event::Event,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const SCREEN_WIDTH: usize = 400;
const SCREEN_HEIGHT: usize = SCREEN_WIDTH;
const PIXEL_SIZE: usize = 3;

const G: f64 = 9.8;
const TIME_STEP: f64 = 0.1;

const CLEAR_COLOR: [u8; 4] = [0, 0, 0, 255];
const ARENA_COLOR: [u8; 4] = [100, 100, 100, 255];
const BALL_COLOR: [u8; 4] = [255, 255, 255, 255];

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let default_scene = "1".to_string();
    let scene_str = args.get(1).unwrap_or(&default_scene).as_str();

    match scene_str {
        "1" => scene_1(),
        "2" => scene_2(),
        "3" => scene_3(),
        "4" => scene_4(),
        "5" => scene_5(),
        _ => scene_1(),
    }
}

fn map_to_range(num: f64, from: &Range<f64>, to: &Range<f64>) -> f64 {
    (num - from.start) / (from.end - from.start) * (to.end - to.start) + to.start
}

fn clear_frame(color: &[u8; 4], frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(color);
    }
}

fn draw_circle(circle: &Circle, color: &[u8; 4], frame: &mut [u8]) {
    let row_start = (circle.center.y - circle.radius).round().max(0.0) as usize;
    let row_end = (circle.center.y + circle.radius)
        .ceil()
        .min(SCREEN_HEIGHT as f64) as usize;
    let col_start = (circle.center.x - circle.radius).floor().max(0.0) as usize;
    let col_end = (circle.center.x + circle.radius)
        .ceil()
        .min(SCREEN_WIDTH as f64) as usize;

    let mut pixel_count = 0;
    for row in row_start..row_end {
        for col in col_start..col_end {
            let distance_squared =
                (col as f64 - circle.center.x).powi(2) + (row as f64 - circle.center.y).powi(2);

            if distance_squared < circle.radius_squared {
                let index = (row * SCREEN_WIDTH + col) * 4;
                frame[index..index + 4].copy_from_slice(color);
                pixel_count += 1;
            }
        }
    }

    if pixel_count == 0 {
        let x = circle.center.x.round() as i32;
        let y = circle.center.y.round() as i32;

        if x >= 0 && x < SCREEN_WIDTH as i32 && y >= 0 && y < SCREEN_HEIGHT as i32 {
            let index = (y as usize * SCREEN_WIDTH + x as usize) * 4;
            frame[index..index + 4].copy_from_slice(color);
        }
    }
}

fn set_pixel(x: usize, y: usize, color: &[u8; 4], frame: &mut [u8]) {
    if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
        return;
    }

    let index = (y * SCREEN_WIDTH + x) * 4;
    frame[index..index + 4].copy_from_slice(color);
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn dot_product(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn reflect(&self, normal: &Vec2) -> Vec2 {
        let dot_product = self.dot_product(normal);
        Vec2 {
            x: self.x - 2.0 * dot_product * normal.x,
            y: self.y - 2.0 * dot_product * normal.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    center: Vec2,
    radius: f64,
    radius_squared: f64,
}

impl Circle {
    fn new(center: Vec2, radius: f64) -> Self {
        Self {
            center,
            radius,
            radius_squared: radius.powi(2),
        }
    }
}

struct BallSimulation {
    arena: Circle,
    ball: Circle,
    velocity: Vec2,
    initial_position: Vec2,
}

impl BallSimulation {
    fn new(arena: Circle, ball: Circle, velocity: Vec2) -> Self {
        Self {
            arena,
            ball,
            velocity,
            initial_position: ball.center,
        }
    }

    fn update(&mut self, acceleration: &Vec2, time_step: f64) {
        self.velocity.x += acceleration.x * time_step;
        self.velocity.y += acceleration.y * time_step;

        self.ball.center.x += self.velocity.x * time_step;
        self.ball.center.y += self.velocity.y * time_step;

        let distance_squared = (self.ball.center.x - self.arena.center.x).powi(2)
            + (self.ball.center.y - self.arena.center.y).powi(2);
        let arena_boundary_squared = (self.arena.radius - self.ball.radius).powi(2);

        if distance_squared > arena_boundary_squared {
            let distance = distance_squared.sqrt();
            let normal = Vec2 {
                x: (self.ball.center.x - self.arena.center.x) / distance,
                y: (self.ball.center.y - self.arena.center.y) / distance,
            };

            self.velocity = self.velocity.reflect(&normal);

            self.ball.center.x =
                self.arena.center.x + (self.arena.radius - self.ball.radius) * normal.x;
            self.ball.center.y =
                self.arena.center.y + (self.arena.radius - self.ball.radius) * normal.y;
        }
    }
}

fn initialize_scene(window_title: &str) -> (EventLoop<()>, Window, Pixels) {
    let event_loop = EventLoop::new();
    let window_size = LogicalSize::new(
        (SCREEN_WIDTH * PIXEL_SIZE) as f64,
        (SCREEN_HEIGHT * PIXEL_SIZE) as f64,
    );
    let window = WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(window_size)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let pixels = Pixels::new(
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
        SurfaceTexture::new(
            window.inner_size().width,
            window.inner_size().height,
            &window,
        ),
    )
    .unwrap();

    (event_loop, window, pixels)
}

fn scene_1() {
    println!("Running Scene 1L Single Ball");
    let (event_loop, window, mut pixels) = initialize_scene("Single Ball");

    let arena = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 2.0,
    );
    let ball = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 100.0,
    );
    let velocity = Vec2 { x: 10.0, y: 0.0 };
    let mut simulation = BallSimulation::new(arena, ball, velocity);
    let acceleration = Vec2 { x: 0.0, y: G };

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            simulation.update(&acceleration, TIME_STEP);
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(&CLEAR_COLOR, frame);
            draw_circle(&simulation.arena, &ARENA_COLOR, frame);
            draw_circle(&simulation.ball, &BALL_COLOR, frame);

            pixels.render().unwrap();
        }
        _ => {}
    });
}

fn scene_2() {
    println!("Running Scene 2: Chaotic System With 10 Balls");
    let (event_loop, window, mut pixels) = initialize_scene("Chaotic System With 10 Balls");

    let num_balls = 10;
    let arena = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 2.0,
    );
    let mut simulations = Vec::with_capacity(num_balls);
    for i in 0..num_balls {
        let ball = Circle::new(
            Vec2 {
                x: SCREEN_WIDTH as f64 / 2.0,
                y: SCREEN_HEIGHT as f64 / 2.0,
            },
            SCREEN_WIDTH as f64 / 100.0,
        );
        let velocity = Vec2 {
            x: (i as f64 / num_balls as f64),
            y: 0.0,
        };
        let simulation = BallSimulation::new(arena, ball, velocity);
        simulations.push(simulation);
    }
    let acceleration = Vec2 { x: 0.0, y: G };

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for simulation in simulations.iter_mut() {
                simulation.update(&acceleration, TIME_STEP);
            }
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(&CLEAR_COLOR, frame);
            draw_circle(&arena, &ARENA_COLOR, frame);
            for simulation in simulations.iter() {
                draw_circle(&simulation.ball, &BALL_COLOR, frame);
            }

            pixels.render().unwrap();
        }
        _ => {}
    });
}

fn scene_3() {
    println!("Running Scene 3: Ball Per Pixel");
    let (event_loop, window, mut pixels) = initialize_scene("Ball Per Pixel");

    let arena = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 2.0,
    );

    let mut simulations = Vec::new();
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let x = col as f64;
            let y = row as f64;

            let distance_squared = (x - arena.center.x).powi(2) + (y - arena.center.y).powi(2);

            if distance_squared < arena.radius_squared {
                let ball = Circle::new(Vec2 { x, y }, 1.0);
                let simulation = BallSimulation::new(arena, ball, Vec2 { x: 0.0, y: 0.0 });
                simulations.push(simulation);
            }
        }
    }

    let acceleration = Vec2 { x: 0.0, y: G };

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for simulation in simulations.iter_mut() {
                simulation.update(&acceleration, TIME_STEP);
            }
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(&CLEAR_COLOR, frame);
            draw_circle(&arena, &ARENA_COLOR, frame);
            for simulation in simulations.iter() {
                draw_circle(&simulation.ball, &BALL_COLOR, frame);
            }

            pixels.render().unwrap();
        }
        _ => {}
    });
}

fn scene_4() {
    println!("Running Scene 4: Position Phase Space");
    let (event_loop, window, mut pixels) = initialize_scene("Position Phase Space");

    let arena = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 2.0,
    );

    let mut simulations = Vec::new();
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let x = col as f64;
            let y = row as f64;

            let distance_squared = (x - arena.center.x).powi(2) + (y - arena.center.y).powi(2);

            if distance_squared < arena.radius_squared {
                let ball = Circle::new(Vec2 { x, y }, 1.0);
                let simulation = BallSimulation::new(arena, ball, Vec2 { x: 0.0, y: 0.0 });
                simulations.push(simulation);
            }
        }
    }

    let acceleration = Vec2 { x: 0.0, y: G };

    let color_r_range = arena.center.x - arena.radius..arena.center.x + arena.radius;
    let color_g_range = arena.center.y - arena.radius..arena.center.y + arena.radius;
    let u8_range = 0.0..255.0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for simulation in simulations.iter_mut() {
                simulation.update(&acceleration, TIME_STEP);
            }
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(&CLEAR_COLOR, frame);
            for simulation in simulations.iter() {
                let x = simulation.initial_position.x.round() as usize;
                let y = simulation.initial_position.y.round() as usize;

                let color_r =
                    map_to_range(simulation.ball.center.x, &color_r_range, &u8_range).round() as u8;
                let color_g =
                    map_to_range(simulation.ball.center.y, &color_g_range, &u8_range).round() as u8;

                set_pixel(x, y, &[color_r, color_g, 0, 255], frame);
            }

            pixels.render().unwrap();
        }
        _ => {}
    });
}

fn scene_5() {
    println!("Running Scene 5: Velocity Phase Space");
    let (event_loop, window, mut pixels) = initialize_scene("Velocity Phase Space");

    let arena = Circle::new(
        Vec2 {
            x: SCREEN_WIDTH as f64 / 2.0,
            y: SCREEN_HEIGHT as f64 / 2.0,
        },
        SCREEN_WIDTH as f64 / 2.0,
    );

    let mut simulations = Vec::new();
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let x = col as f64;
            let y = row as f64;

            let distance_squared = (x - arena.center.x).powi(2) + (y - arena.center.y).powi(2);

            if distance_squared < arena.radius_squared {
                let ball = Circle::new(Vec2 { x, y }, 1.0);
                let simulation = BallSimulation::new(arena, ball, Vec2 { x: 0.0, y: 0.0 });
                simulations.push(simulation);
            }
        }
    }

    let acceleration = Vec2 { x: 0.0, y: G };

    let max_velocity = SCREEN_WIDTH as f64 / 10.0 * 2.5; // Magic number used to set color range
    let color_g_range = 0.0..max_velocity;
    let color_b_range = 0.0..max_velocity;
    let u8_range = 100.0..255.0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for simulation in simulations.iter_mut() {
                simulation.update(&acceleration, TIME_STEP);
            }

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            clear_frame(&CLEAR_COLOR, frame);
            for simulation in simulations.iter() {
                let x = simulation.initial_position.x.round() as usize;
                let y = simulation.initial_position.y.round() as usize;

                let color_g =
                    map_to_range(simulation.velocity.x, &color_g_range, &u8_range).round() as u8;
                let color_b =
                    map_to_range(simulation.velocity.y, &color_b_range, &u8_range).round() as u8;

                set_pixel(x, y, &[0, color_g, color_b, 255], frame);
            }

            pixels.render().unwrap();
        }
        _ => {}
    });
}
