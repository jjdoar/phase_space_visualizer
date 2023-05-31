use pixels::{Pixels, SurfaceTexture};
use std::env;
use winit::dpi::LogicalSize;
use winit::event;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowBuilder;

const DEFAULT_SCENE: &str = "1";

const WINDOW_TITLE: &str = "Phase Space Visualizer";
const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;
const PIXEL_SIZE: u32 = 3;

const CLEAR_COLOR: [u8; 4] = [0, 0, 0, 255];
const CIRCLE_COLOR: [u8; 4] = [100, 100, 100, 255];

const CIRCLE_RADIUS: u32 = (SCREEN_HEIGHT / 2) - 1;
const CIRCLE_RADIUS_SQUARED: u32 = CIRCLE_RADIUS.pow(2);
const CIRCLE_CENTER_X: u32 = SCREEN_WIDTH / 2;
const CIRCLE_CENTER_Y: u32 = SCREEN_HEIGHT / 2;

const G: f64 = 9.8;
const TIME_STEP: f64 = 0.1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene: u32 = args
        .get(1)
        .unwrap_or(&DEFAULT_SCENE.to_string())
        .parse()
        .unwrap_or(1);

    match scene {
        1 => scene_1(),
        2 => scene_2(),
        3 => scene_3(),
        4 => scene_4(),
        5 => scene_5(),
        _ => scene_1(),
    }
}

fn reflect(v_x: f64, v_y: f64, n_x: f64, n_y: f64) -> (f64, f64) {
    let dot_product = v_x * n_x + v_y * n_y;
    let r_x = v_x - 2.0 * dot_product * n_x;
    let r_y = v_y - 2.0 * dot_product * n_y;
    (r_x, r_y)
}

fn point_update(p_x: f64, p_y: f64, v_x: f64, v_y: f64) -> (f64, f64, f64, f64) {
    let acceleration_x = 0.0;
    let acceleration_y = G;

    let velocity_x = v_x + acceleration_x * TIME_STEP;
    let velocity_y = v_y + acceleration_y * TIME_STEP;

    let position_x = p_x + velocity_x * TIME_STEP;
    let position_y = p_y + velocity_y * TIME_STEP;

    let distance_squared = (position_x - CIRCLE_CENTER_X as f64).powi(2)
        + (position_y - CIRCLE_CENTER_Y as f64).powi(2);
    if distance_squared >= CIRCLE_RADIUS_SQUARED as f64 {
        let distance = distance_squared.sqrt();
        let (velocity_x, velocity_y) = reflect(
            velocity_x,
            velocity_y,
            (CIRCLE_CENTER_X as f64 - position_x) / distance,
            (CIRCLE_CENTER_Y as f64 - position_y) / distance,
        );

        let position_x = CIRCLE_CENTER_X as f64
            + CIRCLE_RADIUS as f64 * ((position_x - CIRCLE_CENTER_X as f64) / distance);
        let position_y = CIRCLE_CENTER_Y as f64
            + CIRCLE_RADIUS as f64 * ((position_y - CIRCLE_CENTER_Y as f64) / distance);

        return (position_x, position_y, velocity_x, velocity_y);
    }

    (position_x, position_y, velocity_x, velocity_y)
}

fn map_to_range(x: f64, from_low: f64, from_high: f64, to_low: f64, to_high: f64) -> f64 {
    (x - from_low) / (from_high - from_low) * (to_high - to_low) + to_low
}

fn init() -> (EventLoop<()>, Window, Pixels) {
    let event_loop = EventLoop::new();
    let window_size = LogicalSize::new(
        (SCREEN_WIDTH * PIXEL_SIZE) as f64,
        (SCREEN_HEIGHT * PIXEL_SIZE) as f64,
    );
    let window = WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(window_size)
        .with_min_inner_size(window_size)
        .with_max_inner_size(window_size)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let pixels = Pixels::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
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
    let (event_loop, window, mut pixels) = init();

    let mut point_position_x = CIRCLE_CENTER_X as f64;
    let mut point_position_y = CIRCLE_CENTER_Y as f64;
    let mut point_velocity_x = 2.0;
    let mut point_velocity_y = 0.0;
    let point_color = [255, 0, 0, 255];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            (
                point_position_x,
                point_position_y,
                point_velocity_x,
                point_velocity_y,
            ) = point_update(
                point_position_x,
                point_position_y,
                point_velocity_x,
                point_velocity_y,
            );

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // clear
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&CLEAR_COLOR);
            }

            // draw circle
            for row in 0..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let x = col as i32 - CIRCLE_CENTER_X as i32;
                    let y = row as i32 - CIRCLE_CENTER_Y as i32;
                    let distance_squared = x.pow(2) + y.pow(2);

                    if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                        continue;
                    }

                    let index = (row * SCREEN_WIDTH + col) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&CIRCLE_COLOR);
                }
            }

            // Draw point
            let point_x = point_position_x.round() as i32;
            let point_y = point_position_y.round() as i32;
            if point_x >= 0
                && point_x < SCREEN_WIDTH as i32
                && point_y >= 0
                && point_y < SCREEN_HEIGHT as i32
            {
                let index = (point_y as u32 * SCREEN_WIDTH + point_x as u32) as usize * 4;
                frame[index..index + 4].copy_from_slice(&point_color);
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}

fn scene_2() {
    let (event_loop, window, mut pixels) = init();

    let num_points = 2;
    let mut point_position_x: Vec<f64> = vec![CIRCLE_CENTER_X as f64; num_points];
    let mut point_position_y: Vec<f64> = vec![CIRCLE_CENTER_Y as f64; num_points];
    let mut point_velocity_x: Vec<f64> = vec![0.5, 0.4];
    let mut point_velocity_y: Vec<f64> = vec![0.0; num_points];
    let point_color: Vec<[u8; 4]> = vec![[255, 0, 0, 255], [0, 255, 0, 255]];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for i in 0..num_points {
                (
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                ) = point_update(
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                );
            }

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // clear
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&CLEAR_COLOR);
            }

            // draw circle
            for row in 0..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let x = col as i32 - CIRCLE_CENTER_X as i32;
                    let y = row as i32 - CIRCLE_CENTER_Y as i32;
                    let distance_squared = x.pow(2) + y.pow(2);

                    if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                        continue;
                    }

                    let index = (row * SCREEN_WIDTH + col) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&CIRCLE_COLOR);
                }
            }

            // Draw points
            for i in 0..num_points {
                let point_x = point_position_x[i].round() as i32;
                let point_y = point_position_y[i].round() as i32;
                if point_x >= 0
                    && point_x < SCREEN_WIDTH as i32
                    && point_y >= 0
                    && point_y < SCREEN_HEIGHT as i32
                {
                    let index = (point_y as u32 * SCREEN_WIDTH + point_x as u32) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&point_color[i]);
                }
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}

fn scene_3() {
    let (event_loop, window, mut pixels) = init();

    let mut point_position_x = CIRCLE_CENTER_X as f64;
    let mut point_position_y = CIRCLE_CENTER_Y as f64;
    let mut point_velocity_x = 2.0;
    let mut point_velocity_y = 0.0;
    let point_color = [255, 0, 0, 255];

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            (
                point_position_x,
                point_position_y,
                point_velocity_x,
                point_velocity_y,
            ) = point_update(
                point_position_x,
                point_position_y,
                point_velocity_x,
                point_velocity_y,
            );

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // clear
            let clear_g = map_to_range(
                point_position_x,
                CIRCLE_CENTER_X as f64 - CIRCLE_RADIUS as f64,
                CIRCLE_CENTER_X as f64 + CIRCLE_RADIUS as f64,
                0.0,
                255.0,
            )
            .round() as u8;
            let clear_b = map_to_range(
                point_position_y,
                CIRCLE_CENTER_Y as f64 - CIRCLE_RADIUS as f64,
                CIRCLE_CENTER_Y as f64 + CIRCLE_RADIUS as f64,
                0.0,
                255.0,
            )
            .round() as u8;
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&[0, clear_g, clear_b, 255]);
            }

            // draw circle
            for row in 0..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let x = col as i32 - CIRCLE_CENTER_X as i32;
                    let y = row as i32 - CIRCLE_CENTER_Y as i32;
                    let distance_squared = x.pow(2) + y.pow(2);

                    if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                        continue;
                    }

                    let index = (row * SCREEN_WIDTH + col) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&CIRCLE_COLOR);
                }
            }

            // Draw point
            let point_x = point_position_x.round() as i32;
            let point_y = point_position_y.round() as i32;
            if point_x >= 0
                && point_x < SCREEN_WIDTH as i32
                && point_y >= 0
                && point_y < SCREEN_HEIGHT as i32
            {
                let index = (point_y as u32 * SCREEN_WIDTH + point_x as u32) as usize * 4;
                frame[index..index + 4].copy_from_slice(&point_color);
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}

fn scene_4() {
    let (event_loop, window, mut pixels) = init();

    let mut num_points = 0;
    let mut point_position_x: Vec<f64> = Vec::new();
    let mut point_position_y: Vec<f64> = Vec::new();
    let mut point_velocity_x: Vec<f64> = Vec::new();
    let mut point_velocity_y: Vec<f64> = Vec::new();
    let point_color = [255, 0, 0, 255];

    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let x = col as i32 - CIRCLE_CENTER_X as i32;
            let y = row as i32 - CIRCLE_CENTER_Y as i32;
            let distance_squared = x.pow(2) + y.pow(2);

            if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                continue;
            }

            num_points += 1;
            point_position_x.push(col as f64);
            point_position_y.push(row as f64);
            point_velocity_x.push(0.0);
            point_velocity_y.push(0.0);
        }
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for i in 0..num_points {
                (
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                ) = point_update(
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                );
            }

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // clear
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&CLEAR_COLOR);
            }

            // draw circle
            for row in 0..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let x = col as i32 - CIRCLE_CENTER_X as i32;
                    let y = row as i32 - CIRCLE_CENTER_Y as i32;
                    let distance_squared = x.pow(2) + y.pow(2);

                    if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                        continue;
                    }

                    let index = (row * SCREEN_WIDTH + col) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&CIRCLE_COLOR);
                }
            }

            // Draw points
            for i in 0..num_points {
                let point_x = point_position_x[i].round() as i32;
                let point_y = point_position_y[i].round() as i32;
                if point_x >= 0
                    && point_x < SCREEN_WIDTH as i32
                    && point_y >= 0
                    && point_y < SCREEN_HEIGHT as i32
                {
                    let index = (point_y as u32 * SCREEN_WIDTH + point_x as u32) as usize * 4;
                    frame[index..index + 4].copy_from_slice(&point_color);
                }
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}

fn scene_5() {
    let (event_loop, window, mut pixels) = init();

    let mut num_points = 0;
    let mut point_initial_position_x: Vec<u32> = Vec::new();
    let mut point_initial_position_y: Vec<u32> = Vec::new();
    let mut point_position_x: Vec<f64> = Vec::new();
    let mut point_position_y: Vec<f64> = Vec::new();
    let mut point_velocity_x: Vec<f64> = Vec::new();
    let mut point_velocity_y: Vec<f64> = Vec::new();

    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let x = col as i32 - CIRCLE_CENTER_X as i32;
            let y = row as i32 - CIRCLE_CENTER_Y as i32;
            let distance_squared = x.pow(2) + y.pow(2);

            if distance_squared > CIRCLE_RADIUS_SQUARED as i32 {
                continue;
            }

            num_points += 1;
            point_initial_position_x.push(col);
            point_initial_position_y.push(row);
            point_position_x.push(col as f64);
            point_position_y.push(row as f64);
            point_velocity_x.push(0.0);
            point_velocity_y.push(0.0);
        }
    }

    let num_points = num_points;
    let point_initial_position_x = point_initial_position_x;
    let point_initial_position_y = point_initial_position_y;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            for i in 0..num_points {
                (
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                ) = point_update(
                    point_position_x[i],
                    point_position_y[i],
                    point_velocity_x[i],
                    point_velocity_y[i],
                );
            }

            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            let frame = pixels.frame_mut();

            // clear
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(&CLEAR_COLOR);
            }

            // Draw points
            for i in 0..num_points {
                let point_color_x = map_to_range(
                    point_position_x[i],
                    CIRCLE_CENTER_X as f64 - CIRCLE_RADIUS as f64,
                    CIRCLE_CENTER_X as f64 + CIRCLE_RADIUS as f64,
                    0.0,
                    255.0,
                )
                .round() as u8;
                let point_color_y = map_to_range(
                    point_position_y[i],
                    CIRCLE_CENTER_Y as f64 - CIRCLE_RADIUS as f64,
                    CIRCLE_CENTER_Y as f64 + CIRCLE_RADIUS as f64,
                    0.0,
                    255.0,
                )
                .round() as u8;

                let index = (point_initial_position_y[i] * SCREEN_WIDTH
                    + point_initial_position_x[i]) as usize
                    * 4;
                frame[index..index + 4].copy_from_slice(&[0, point_color_x, point_color_y, 255]);
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}
