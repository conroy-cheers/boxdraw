use crate::canvas::Vector3TranslationRotation;

use std::ops::{Add, Div, Mul, Neg};

use nalgebra::{Rotation2, Vector2, Vector3};

pub enum TransformMode {
    Point,
    Vector,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PhysicsFrame {
    Global,
    Body(Vector3<f32>),
}

impl From<Vector3<f32>> for PhysicsFrame {
    fn from(vec: Vector3<f32>) -> Self {
        PhysicsFrame::Body(vec)
    }
}

impl PhysicsFrame {
    fn rotation(&self) -> f32 {
        match self {
            PhysicsFrame::Global => 0.,
            PhysicsFrame::Body(obj) => obj.rotation(),
        }
    }

    fn rotation_mat(&self) -> Rotation2<f32> {
        Rotation2::new(self.rotation())
    }

    fn translation(&self) -> Vector2<f32> {
        match self {
            PhysicsFrame::Global => Vector2::new(0., 0.),
            PhysicsFrame::Body(obj) => obj.translation(),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct FramedVector2 {
    frame: PhysicsFrame,
    vec: Vector2<f32>,
}

impl Div<f32> for FramedVector2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            frame: self.frame,
            vec: self.vec / rhs,
        }
    }
}

impl Mul<f32> for FramedVector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            frame: self.frame,
            vec: self.vec * rhs,
        }
    }
}

impl Neg for FramedVector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            frame: self.frame,
            vec: -self.vec,
        }
    }
}

impl Add for FramedVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            frame: self.frame,
            vec: self.vec + rhs.to_frame(self.frame, TransformMode::Vector).vec,
        }
    }
}

impl FramedVector2 {
    pub fn new(frame: PhysicsFrame, vec: Vector2<f32>) -> Self {
        Self { frame, vec }
    }

    pub fn vec(&self) -> Vector2<f32> {
        self.vec
    }

    pub fn to_frame(&self, new_frame: PhysicsFrame, mode: TransformMode) -> Self {
        match &mode {
            TransformMode::Vector => {
                let new_vec = Rotation2::new(new_frame.rotation() - self.frame.rotation())
                    .inverse()
                    * self.vec;
                Self {
                    frame: new_frame,
                    vec: new_vec,
                }
            }
            TransformMode::Point => {
                let old_offset_global = self.frame.rotation_mat() * self.vec;
                let point_global = old_offset_global + self.frame.translation();
                let new_offset_global = point_global - new_frame.translation();
                let new_offset_local = new_frame.rotation_mat().inverse() * new_offset_global;
                Self {
                    frame: new_frame,
                    vec: new_offset_local,
                }
            }
        }
    }

    pub fn cross(&self, other: &Self) -> f32 {
        let other = other.to_frame(self.frame, TransformMode::Vector);
        self.vec.x * other.vec.y - self.vec.y * other.vec.x
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_framed_vec() {
        let vec = FramedVector2::new(PhysicsFrame::Global, Vector2::new(1., 1.5));
        assert_eq!(vec.frame, PhysicsFrame::Global);
        assert_eq!(vec.vec, Vector2::new(1., 1.5));
        assert_relative_eq!((vec / 2.).vec, Vector2::new(0.5, 0.75));
    }

    #[test]
    fn test_cross() {
        let vec1 = FramedVector2::new(PhysicsFrame::Global, Vector2::new(1., 0.));
        let vec2 = FramedVector2::new(PhysicsFrame::Global, Vector2::new(0., 1.));
        assert_relative_eq!(vec1.cross(&vec2), 1.);
    }

    #[test]
    fn test_frame() {
        let frame_pos = Vector3::new(1., 2., PI);
        let local = PhysicsFrame::Body(frame_pos);
        assert_eq!(local.rotation(), PI);
        assert_eq!(local.translation(), Vector2::new(1., 2.));

        let global = PhysicsFrame::Global;
        assert_eq!(global.rotation(), 0.);
        assert_eq!(global.translation(), Vector2::new(0., 0.));
    }

    #[test]
    fn test_transform() {
        let frame_pos = Vector3::new(1., 2., PI / 2.);
        let local = PhysicsFrame::Body(frame_pos);

        let vec = Vector2::new(1., 0.);
        let framed_vec = FramedVector2::new(local, vec);

        let new_frame_pos = Vector3::new(3., 1., PI);
        let new_frame = PhysicsFrame::Body(new_frame_pos);

        let new_framed_vec = framed_vec.to_frame(new_frame, TransformMode::Vector);
        assert_relative_eq!(new_framed_vec.vec, Vector2::new(0., -1.));

        let new_framed_vec = framed_vec.to_frame(new_frame, TransformMode::Point);
        assert_relative_eq!(new_framed_vec.vec, Vector2::new(2., -2.));
    }
}
