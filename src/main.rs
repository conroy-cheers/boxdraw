#![feature(div_duration)]

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::{MouseMode, Window, WindowOptions};
use physics::{PhysicsData, PhysicsObject};
use raqote::{DrawOptions, DrawTarget, Point, SolidSource, Source};
extern crate nalgebra as na;
use na::Vector2;

mod canvas;
mod parameters;
mod physics;
mod state;

use crate::canvas::Annotations;
use crate::parameters::*;

struct WindowData {
    dt: DrawTarget,
    window: Window,
    font: font_kit::font::Font,
    size: (usize, usize),
}

fn draw(
    wd: &mut WindowData,
    objs: &PhysicsData,
    annotations: &Annotations,
    delta: &std::time::Duration,
) {
    wd.dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));

    // draw all objects
    objs.draw(&mut wd.dt);

    // draw annotations
    for annotation in annotations.iter() {
        annotation.draw(&mut wd.dt);
    }

    let framerate = std::time::Duration::from_secs(1).div_duration_f32(*delta);
    let delta_string = format!("{:.2}fps", framerate);
    wd.dt.draw_text(
        &wd.font,
        36.,
        &delta_string,
        Point::new(0., 100.),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
        &DrawOptions::new(),
    );

    wd.window
        .update_with_buffer(wd.dt.get_data(), wd.size.0, wd.size.1)
        .unwrap();
}

fn main() {
    let mut window_data = WindowData {
        dt: DrawTarget::new(WIDTH as i32, HEIGHT as i32),
        window: Window::new(
            "I love rectangles",
            WIDTH,
            HEIGHT,
            WindowOptions {
                ..WindowOptions::default()
            },
        )
        .unwrap(),
        font: SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap()
            .load()
            .unwrap(),
        size: (WIDTH, HEIGHT),
    };

    let mut physics_data =
        PhysicsData::new(vec![PhysicsObject::rect(20., 100., Vector2::new(0., 0.))]);

    // Redraw the window in a loop as fast as possible
    let mut last_frame_time = std::time::Instant::now();
    while window_data.window.is_open() && !window_data.window.is_key_down(minifb::Key::Escape) {
        // Get time since last frame
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_frame_time);

        // get cursor pos if valid
        let mut annotations = Annotations::new();
        if let Some(pos) = window_data.window.get_mouse_pos(MouseMode::Clamp) {
            annotations = physics::update_physics(&mut physics_data, &delta, pos);
        }
        draw(&mut window_data, &physics_data, &annotations, &delta);

        last_frame_time = now;
    }
}
