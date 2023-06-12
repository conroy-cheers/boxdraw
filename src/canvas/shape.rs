use super::{Vector3TranslationRotation, VectorPathBuilder};
use crate::state::ObjectState;
use nalgebra::{Rotation2, Vector2};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub trait Shape {
    /// Vector from CoM to joint, in the body frame.
    fn joint_position(&self) -> Vector2<f32>;

    /// Moment of inertia about the shape's center of mass.
    /// Assumes uniform density.
    fn moment_of_inertia(&self, m: f32) -> f32;

    /// Draw the shape. Rotation is applied around the shape's center of mass.
    fn draw(&self, dt: &mut DrawTarget, state: &ObjectState);
}

pub struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn joint_position(&self) -> Vector2<f32> {
        Vector2::new(-self.width / 2., self.height / 2.)
    }

    fn moment_of_inertia(&self, m: f32) -> f32 {
        let b = self.width;
        let h = self.height;
        m * (b * h * (b.powf(2.) + h.powf(2.)) / 12.) / 100.
    }

    fn draw(&self, dt: &mut DrawTarget, state: &ObjectState) {
        let rmat = Rotation2::new(state.pos.rotation());
        let width_vec = Vector2::new(self.width, 0.);
        let height_vec = Vector2::new(0., self.height);
        // Use rotation matrix to transform height/width vectors
        let width = rmat * width_vec;
        let height = rmat * height_vec;

        let mut pb = PathBuilder::new();
        let start_pos = state.pos.translation() - width/2. - height/2.;
        pb.move_to_vec(start_pos);
        pb.line_to_vec(start_pos + width);
        pb.line_to_vec(start_pos + width + height);
        pb.line_to_vec(start_pos + height);
        pb.close();
        let path = pb.finish();
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
            &DrawOptions::new(),
        );
    }
}
