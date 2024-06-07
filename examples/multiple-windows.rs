//! # Example: Input Handling
//!
//! This example allows you to move a red circle to the location of a click on the simulator
//! screen, or move the circle using the arrow keys. Although input handling is not a part of the
//! embedded-graphics API, the simulator can be used to emulate input controls in order to
//! represent more complex UI systems such as touch screens.

extern crate embedded_graphics;
extern crate embedded_graphics_simulator;

use std::time::Duration;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettings, SimulatorDisplay, SimulatorEvent, Window,
};

const BACKGROUND_COLOR: Rgb888 = Rgb888::BLACK;
const FOREGROUND_COLOR: Rgb888 = Rgb888::RED;
const KEYBOARD_DELTA: i32 = 20;

fn move_circle(
    display: &mut SimulatorDisplay<Rgb888>,
    old_center: Point,
    new_center: Point,
) -> Result<(), core::convert::Infallible> {
    // Clear old circle
    Circle::with_center(old_center, 200)
        .into_styled(PrimitiveStyle::with_fill(BACKGROUND_COLOR))
        .draw(display)?;

    // Draw circle at new location
    Circle::with_center(new_center, 200)
        .into_styled(PrimitiveStyle::with_fill(FOREGROUND_COLOR))
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 480));
    let mut window = Window::with_video_subsystem(
        "Click to move circle",
        &OutputSettings::default(),
        video_subsystem.clone(),
    );
    let mut window2 = Window::with_video_subsystem(
        "Click to move circle",
        &OutputSettings::default(),
        video_subsystem,
    );

    let mut position = Point::new(200, 200);
    Circle::with_center(position, 200)
        .into_styled(PrimitiveStyle::with_fill(FOREGROUND_COLOR))
        .draw(&mut display)?;

    'running: loop {
        window.update(&display);
        window2.update(&display);

        for event in event_pump
            .poll_iter()
            .filter_map(|e| window.sdl_to_simulator_event(e))
        {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let delta = match keycode {
                        Keycode::Left => Point::new(-KEYBOARD_DELTA, 0),
                        Keycode::Right => Point::new(KEYBOARD_DELTA, 0),
                        Keycode::Up => Point::new(0, -KEYBOARD_DELTA),
                        Keycode::Down => Point::new(0, KEYBOARD_DELTA),
                        _ => Point::zero(),
                    };
                    let new_position = position + delta;
                    move_circle(&mut display, position, new_position)?;
                    position = new_position;
                }
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    move_circle(&mut display, position, point)?;
                    position = point;
                }
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
