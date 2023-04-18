use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
extern crate nalgebra as na;
use super::pathbuilder::VectorPathBuilder;
use crate::parameters::*;
use na::{SimdPartialOrd, Vector2};

pub(crate) struct Annotations {
    annotations: Vec<Box<dyn Annotation>>,
}

impl Annotations {
    pub fn new() -> Annotations {
        Annotations { annotations: Vec::new() }
    }

    pub fn add(&mut self, annotation: impl Annotation + 'static) {
        self.annotations.push(Box::new(annotation));
    }

    pub fn iter(&self) -> std::slice::Iter<Box<dyn Annotation>> {
        self.annotations.iter()
    }
}

pub trait Annotation {
    fn draw(&self, dt: &mut DrawTarget);
}

pub(crate) struct LineAnnotation {
    start: Vector2<f32>,
    end: Vector2<f32>,
}

impl LineAnnotation {
    pub(crate) fn new(start: Vector2<f32>, vec: Vector2<f32>) -> LineAnnotation {
        let start = Vector2::new(
            start.x.simd_clamp(0., WIDTH as f32),
            start.y.simd_clamp(0., HEIGHT as f32),
        );
        LineAnnotation {
            start: start,
            end: start
                + Vector2::new(
                    vec.x.simd_clamp(0., WIDTH as f32),
                    vec.y.simd_clamp(0., HEIGHT as f32),
                ),
        }
    }
}

impl Annotation for LineAnnotation {
    fn draw(&self, dt: &mut DrawTarget) {
        let mut pb = PathBuilder::new();
        pb.move_to_vec(self.start);
        pb.line_to_vec(self.end);
        let path = pb.finish();
        dt.stroke(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
            &raqote::StrokeStyle {
                cap: raqote::LineCap::Round,
                join: raqote::LineJoin::Round,
                width: 2.,
                miter_limit: 10.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );
    }
}
