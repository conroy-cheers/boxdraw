use crate::canvas::{Annotations, LineAnnotation, Rectangle, Shape, Vector3TranslationRotation};
use crate::state::ObjectState;
use nalgebra::{Rotation2, Vector2, Vector3};
use raqote::DrawTarget;

pub struct PhysicsObject {
    shape: Box<dyn Shape>,
    state: ObjectState,
}

impl PhysicsObject {
    fn get_cog(&self) -> Vector2<f32> {
        self.shape.get_cog()
    }

    pub fn rect(width: f32, height: f32, pos: Vector2<f32>) -> Self {
        Self {
            shape: Box::new(Rectangle::new(width, height)),
            state: ObjectState {
                pos: Vector3::new(pos.x, pos.y, 0.),
                vel: Vector3::zeros(),
                acc: Vector3::zeros(),
                mass: 1000000.,
                rotational_inertia: (1. / 12.)
                    * width
                    * height
                    * (width.powf(2.) + height.powf(2.)),
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
    cursor_pos: (f32, f32),
) -> Annotations {
    let mut annotations = Annotations::new();

    // Only operate on the first object
    let obj = &mut objs.objects[0];

    // Important values and when they are from
    let last_acc = obj.state.acc;
    let last_vel = obj.state.vel;
    let m = obj.state.mass;
    let i = obj.state.rotational_inertia;

    let rmat = Rotation2::new(last_vel.rotation());

    // Calculate acceleration at CoG
    let c1 = rmat * obj.get_cog();

    let lw1 = Rotation2::new(obj.state.vel.z);
    let la1 = Rotation2::new(obj.state.acc.z);
    let ac = obj.state.acc.translation() + la1 * c1 + lw1 * (lw1 * c1);

    annotations.add(LineAnnotation::new(obj.state.pos.translation(), ac));

    // Moment of inertia about pivot
    let ia = i + m * c1.norm_squared();

    // Pivot torque
    let ta = -crate::parameters::FRICTION_COEFF * last_vel.rotation();

    // Calculate new rotational acceleration
    let new_racc = (ta + c1.y * m * last_acc.x - c1.x * m * (9.81 + last_acc.y)) / ia;

    // Generate new rotational state from accel
    let dt = delta.as_secs_f32();
    let new_rvel = obj.state.vel.z + new_racc * dt;
    let new_rpos = obj.state.pos.z + new_rvel * dt;

    // Assuming all prior calculations are correct, they should (approximately) lead to the
    // joint remaining pinned. Therefore we can derive velocity/acceleration from that.
    let new_pos = Vector3::new(cursor_pos.0, cursor_pos.1, new_rpos);
    let pos_delta = new_pos - obj.state.pos;
    let new_vel = Vector3::new(pos_delta.x / dt, pos_delta.y / dt, new_rvel.clamp(-1., 1.));
    let vel_delta = new_vel - obj.state.vel;
    let new_acc = Vector3::new(
        vel_delta.x / dt,
        vel_delta.y / dt,
        new_racc.clamp(-0.1, 0.1),
    );

    annotations.add(LineAnnotation::new(
        obj.state.pos.translation(),
        new_acc.translation() * 0.0002,
    ));

    obj.state.acc = new_acc;
    obj.state.vel = obj.state.vel + new_acc * delta.as_secs_f32();
    obj.state.pos = obj.state.pos + new_vel * delta.as_secs_f32();

    annotations
}
