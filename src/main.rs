use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event;
use winit::event::Event;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
// Scene 1:
// Draw a circle
// Draw a point
// Simulate the point bouncing around the circle

// Scene 2:
// Draw a circle
// Draw two points
// Simulate the points bouncing around the circle

// Scene 3:
// Simulate a point for each starting pixel in the circle
// Color each point according to its point in the circle

const WINDOW_TITLE: &str = "Phase Space Visualizer";
const SCREEN_WIDTH: u32 = 300;
const SCREEN_HEIGHT: u32 = 200;
const PIXEL_SIZE: u32 = 6;

const CLEAR_COLOR: [u8; 4] = [0, 0, 0, 255];
const CIRCLE_COLOR: [u8; 4] = [255, 255, 255, 255];
const POINT_COLOR: [u8; 4] = [255, 0, 0, 255];

const CIRCLE_RADIUS: u32 = (SCREEN_HEIGHT / 2) - 1;
const CIRCLE_RADIUS_SQUARED: u32 = CIRCLE_RADIUS.pow(2);
const CIRCLE_CENTER_X: u32 = SCREEN_WIDTH / 2;
const CIRCLE_CENTER_Y: u32 = SCREEN_HEIGHT / 2;

const G: f64 = 9.8;
const TIME_STEP: f64 = 0.05;

fn main() {
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
    let mut pixels = Pixels::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SurfaceTexture::new(
            window.inner_size().width,
            window.inner_size().height,
            &window,
        ),
    )
    .unwrap();

    let mut point_position_x = CIRCLE_CENTER_X as f64;
    let mut point_position_y = CIRCLE_CENTER_Y as f64;
    let mut point_velocity_x = 2.0;
    let mut point_velocity_y = 0.0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            window_id: _,
            event: event::WindowEvent::CloseRequested,
        } => {
            control_flow.set_exit();
        }
        Event::MainEventsCleared => {
            // Update point
            let point_acceleration_x = 0.0;
            let point_acceleration_y = G;

            point_velocity_x += point_acceleration_x * TIME_STEP;
            point_velocity_y += point_acceleration_y * TIME_STEP;

            point_position_x += point_velocity_x * TIME_STEP;
            point_position_y += point_velocity_y * TIME_STEP;

            let distance_squared = (point_position_x - CIRCLE_CENTER_X as f64).powi(2)
                + (point_position_y - CIRCLE_CENTER_Y as f64).powi(2);
            if distance_squared >= CIRCLE_RADIUS_SQUARED as f64 {
                let distance = distance_squared.sqrt();
                let (reflection_x, reflection_y) = reflect(
                    point_velocity_x,
                    point_velocity_y,
                    (CIRCLE_CENTER_X as f64 - point_position_x) / distance,
                    (CIRCLE_CENTER_Y as f64 - point_position_y) / distance,
                );

                point_velocity_x = reflection_x;
                point_velocity_y = reflection_y;

                point_position_x = CIRCLE_CENTER_X as f64
                    + CIRCLE_RADIUS as f64
                        * ((point_position_x - CIRCLE_CENTER_X as f64) / distance);
                point_position_y = CIRCLE_CENTER_Y as f64
                    + CIRCLE_RADIUS as f64
                        * ((point_position_y - CIRCLE_CENTER_Y as f64) / distance);
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

            // Draw point
            let point_x = point_position_x.round() as i32;
            let point_y = point_position_y.round() as i32;
            if point_x >= 0
                && point_x < SCREEN_WIDTH as i32
                && point_y >= 0
                && point_y < SCREEN_HEIGHT as i32
            {
                let index = (point_y as u32 * SCREEN_WIDTH + point_x as u32) as usize * 4;
                frame[index..index + 4].copy_from_slice(&POINT_COLOR);
            }

            pixels.render().unwrap();
        }
        _ => (),
    });
}

fn reflect(v_x: f64, v_y: f64, n_x: f64, n_y: f64) -> (f64, f64) {
    let dot_product = v_x * n_x + v_y * n_y;
    let r_x = v_x - 2.0 * dot_product * n_x;
    let r_y = v_y - 2.0 * dot_product * n_y;
    (r_x, r_y)
}
