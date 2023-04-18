use super::{Vector3TranslationRotation, VectorPathBuilder};
use crate::state::ObjectState;
use nalgebra::{Rotation2, Vector2};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub trait Shape {
    fn get_cog(&self) -> Vector2<f32>;
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
    fn get_cog(&self) -> Vector2<f32> {
        Vector2::new(self.width / 2., self.height / 2.)
    }

    fn draw(&self, dt: &mut DrawTarget, state: &ObjectState) {
        let rmat = Rotation2::new(state.pos.rotation());
        let width_vec = Vector2::new(self.width, 0.);
        let height_vec = Vector2::new(0., self.height);
        // Use rotation matrix to transform height/width vectors
        let width = rmat * width_vec;
        let height = rmat * height_vec;

        let mut pb = PathBuilder::new();
        pb.move_to_vec(state.pos.translation());
        pb.line_to_vec(state.pos.translation() + width);
        pb.line_to_vec(state.pos.translation() + width + height);
        pb.line_to_vec(state.pos.translation() + height);
        pb.close();
        let path = pb.finish();
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
            &DrawOptions::new(),
        );
    }
}
