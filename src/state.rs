use nalgebra::Vector3;

pub struct ObjectState {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub acc: Vector3<f32>,
    pub mass: f32,
    pub rotational_inertia: f32,
}
