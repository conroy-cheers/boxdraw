#![feature(div_duration)]

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::{MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
extern crate nalgebra as na;
use na::{Rotation2, Rotation3, Transform2, Translation2, Vector2, Vector3, VectorView2};

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

struct WindowData {
    dt: DrawTarget,
    window: Window,
    font: font_kit::font::Font,
    size: (usize, usize),
}

trait Shape {
    fn get_cog(&self) -> Vector2<f32>;
    fn draw(&self, dt: &mut DrawTarget, state: &ObjectState);
}

struct Rectangle {
    width: f32,
    height: f32,
}

trait VectorPathBuilder {
    fn move_to_vec(&mut self, pos: Vector2<f32>);
    fn line_to_vec(&mut self, pos: Vector2<f32>);
}

impl VectorPathBuilder for PathBuilder {
    fn move_to_vec(&mut self, pos: Vector2<f32>) {
        self.move_to(pos.x, pos.y);
    }

    fn line_to_vec(&mut self, pos: Vector2<f32>) {
        self.line_to(pos.x, pos.y);
    }
}

impl Shape for Rectangle {
    fn get_cog(&self) -> Vector2<f32> {
        Vector2::new(self.width / 2., self.height / 2.)
    }

    fn draw(&self, dt: &mut DrawTarget, state: &ObjectState) {
        let rmat = Rotation2::new(state.angle());
        let width_vec = Vector2::new(self.width, 0.);
        let height_vec = Vector2::new(0., self.height);
        // Use rotation matrix to transform height/width vectors
        let width = rmat * width_vec;
        let height = rmat * height_vec;

        let mut pb = PathBuilder::new();
        pb.move_to_vec(state.origin());
        pb.line_to_vec(state.origin() + width);
        pb.line_to_vec(state.origin() + width + height);
        pb.line_to_vec(state.origin() + height);
        pb.close();
        let path = pb.finish();
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
            &DrawOptions::new(),
        );
    }
}

struct ObjectState {
    pos: Vector3<f32>,
    vel: Vector3<f32>,
    acc: Vector3<f32>,
    mass: f32,
    rotational_inertia: f32,
}

impl ObjectState {
    fn origin(&self) -> Vector2<f32> {
        Vector2::new(self.pos.x, self.pos.y)
    }

    fn angle(&self) -> f32 {
        self.pos.z
    }
}

struct PhysicsObject {
    shape: Box<dyn Shape>,
    state: ObjectState,
}

impl PhysicsObject {
    fn get_cog(&self) -> Vector2<f32> {
        self.shape.get_cog()
    }

    fn rotation(&self) -> Rotation2<f32> {
        Rotation2::new(self.state.pos.z)
    }

    fn translation(&self) -> Translation2<f32> {
        Translation2::new(self.state.pos.x, self.state.pos.y)
    }

    fn rect(width: f32, height: f32, pos: Vector2<f32>) -> Self {
        Self {
            shape: Box::new(Rectangle { width, height }),
            state: ObjectState {
                pos: Vector3::new(pos.x, pos.y, 0.),
                vel: Vector3::zeros(),
                acc: Vector3::zeros(),
                mass: 1000.,
                rotational_inertia: 1000.,
            },
        }
    }
}

struct PhysicsData {
    objects: Vec<PhysicsObject>,
}

fn update_physics(objs: &mut PhysicsData, delta: &std::time::Duration, cursor_pos: (f32, f32)) {
    // Only operate on the first object
    let obj = &mut objs.objects[0];

    // Calculate motion of pivot point
    let new_pivot_pos = Vector2::new(cursor_pos.0, cursor_pos.1);
    let new_pivot_vel = (new_pivot_pos - obj.state.pos) / delta.as_secs_f32();
    let new_pivot_acc = (new_pivot_vel - obj.state.vel) / delta.as_secs_f32();

    // Transform pivot motion to CoG motion
    let origin_to_cog_global = obj.rotation() * obj.get_cog();
    let acc_matrix = Rotation2::new(obj.state.r_acc);
    let vel_matrix = Rotation2::new(obj.state.r_vel);
    let cog_accel_global = new_pivot_acc
        + acc_matrix * origin_to_cog_global
        + vel_matrix * (vel_matrix * origin_to_cog_global);
    let cog_net_force_global = obj.state.mass * cog_accel_global;

    let pivot_torque = obj.state.r_vel * -5.0;
    let reaction_force = obj.state.rotational_inertia * acc_matrix;

    // Get gravity force on CoG
    let gravity_force = Vector2::new(0., 9.81 * obj.state.mass);
    // Translate from global into object frame
    let gravity_force_local = obj.rotation().inverse() * gravity_force;

    // Get reaction force at origin
    let origin_force_local = net_force - gravity_force_local;
    // Vector from CoG to origin
    let cog_to_origin = -obj.get_cog();
    // Calculate torque on CoG
    let torque = -origin_force_local.x * cog_to_origin.y + origin_force_local.y * cog_to_origin.x;
    // Add joint friction
    let torque = torque - obj.state.r_vel * 2000.0;

    // Calculate rotational acceleration from torque
    let r_acc = torque / obj.state.rotational_inertia;
    // Update rotational velocity
    obj.state.r_vel += r_acc * delta.as_secs_f32();
    // Update rotational position
    obj.state.rot += obj.state.r_vel * delta.as_secs_f32();

    obj.state.pos = new_pos;
    obj.state.vel = new_vel;
}

fn draw(wd: &mut WindowData, objs: &PhysicsData, delta: &std::time::Duration) {
    wd.dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));

    // draw all objects
    for obj in &objs.objects {
        obj.shape.draw(&mut wd.dt, &obj.state);
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
            "Raqote",
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

    let mut physics_data = PhysicsData {
        objects: vec![PhysicsObject::rect(20., 100., Vector2::new(0., 0.))],
    };

    // Redraw the window in a loop as fast as possible
    let mut last_frame_time = std::time::Instant::now();
    while window_data.window.is_open() && !window_data.window.is_key_down(minifb::Key::Escape) {
        // Get time since last frame
        let now = std::time::Instant::now();
        let delta = now.duration_since(last_frame_time);

        // get cursor pos if valid
        if let Some(pos) = window_data.window.get_mouse_pos(MouseMode::Clamp) {
            update_physics(&mut physics_data, &delta, pos);
        }
        draw(&mut window_data, &physics_data, &delta);

        last_frame_time = now;
    }
}
