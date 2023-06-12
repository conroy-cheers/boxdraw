mod frame;

use frame::{FramedVector2, PhysicsFrame};

use crate::canvas::{
    Annotations, ArcAnnotation, LineAnnotation, Rectangle, Shape, Vector3TranslationRotation,
};
use crate::parameters::*;
use crate::physics::frame::TransformMode;
use crate::state::ObjectState;
use nalgebra::{Vector2, Vector3};
use raqote::DrawTarget;

fn shift_moment_of_inertia(moi: f32, m: f32, r: f32) -> f32 {
    moi + m * r.powf(2.)
}

pub struct PhysicsObject {
    shape: Box<dyn Shape>,
    state: ObjectState,
}

impl PhysicsObject {
    pub fn rect(width: f32, height: f32, pos: Vector2<f32>) -> Self {
        Self {
            shape: Box::new(Rectangle::new(width, height)),
            state: ObjectState {
                pos: Vector3::new(pos.x, pos.y, 0.),
                vel: Vector3::zeros(),
                acc: Vector3::zeros(),
                mass: 100.,
                rotational_inertia: 10.,
            },
        }
    }
}

pub struct PhysicsData {
    objects: Vec<PhysicsObject>,
}

impl PhysicsData {
    pub fn new(objects: Vec<PhysicsObject>) -> Self {
        Self { objects }
    }

    pub fn draw(&self, dt: &mut DrawTarget) {
        for obj in &self.objects {
            obj.shape.draw(dt, &obj.state);
        }
    }
}

pub(crate) fn update_physics(
    objs: &mut PhysicsData,
    delta: &std::time::Duration,
    cursor_pos: Vector2<f32>,
) -> Annotations {
    let mut annotations = Annotations::new();
    const ACCEL_SCALE: f32 = 0.001;

    // Only operate on the first object
    let obj = &mut objs.objects[0];

    // Important values and when they are from
    let last_vel = obj.state.vel;
    let m = obj.state.mass;

    let object_frame = PhysicsFrame::from(obj.state.pos);
    let pivot_pos = FramedVector2::new(object_frame, obj.shape.joint_position());

    // Force at pivot
    let pivot_pos_global = pivot_pos.to_frame(PhysicsFrame::Global, TransformMode::Point);
    let f_pull = FramedVector2::new(
        PhysicsFrame::Global,
        Vector2::new(
            (cursor_pos.x - pivot_pos_global.vec().x) * CURSOR_P - obj.state.vel.x * CURSOR_D,
            (cursor_pos.y - pivot_pos_global.vec().y) * CURSOR_P - obj.state.vel.y * CURSOR_D,
        ),
    );
    // Gravity at CoG
    let f_g = FramedVector2::new(PhysicsFrame::Global, Vector2::new(0., -9816. * m));
    // Reaction at pivot
    let f_r = -f_g;
    // Net force, global frame
    let f_net = f_pull + f_g + f_r;
    let f_net_object = f_net.to_frame(object_frame, TransformMode::Vector);

    // Moment of inertia about pivot
    // let ia = shift_moment_of_inertia(obj.shape.moment_of_inertia(m), m, pivot_pos.vec().norm());
    let ia = obj.shape.moment_of_inertia(m);

    // Inertial fudge force in local frame, applied to CoM
    let f_i = -f_net_object;

    // Pivot friction
    let t_f = -crate::parameters::FRICTION_COEFF * last_vel.rotation()
        - crate::parameters::FRICTION_COEFF_QUADRATIC * (last_vel.rotation()).powf(2.);
    // Gravity in body frame
    let local_g = f_g.to_frame(object_frame, TransformMode::Vector);
    // Net torque at pivot
    let pivot_torque = (-pivot_pos).cross(&(local_g + f_i)) + t_f;
    let angular_accel = pivot_torque / ia;

    println!("angular_accel: {}", angular_accel);
    annotations.add(ArcAnnotation::new(
        obj.state.pos.translation(),
        15.,
        obj.state.pos.rotation(),
        angular_accel * 10.,
    ));

    let new_acc = Vector3::new(f_net.vec().x / m, f_net.vec().y / m, pivot_torque / ia);
    obj.state.acc = new_acc;
    obj.state.vel = obj.state.vel + new_acc * delta.as_secs_f32();
    obj.state.pos = obj.state.pos + obj.state.vel * delta.as_secs_f32();

    annotations
}
