use nalgebra::{Vector2, Vector3};
use crate::parameters::HEIGHT;

pub(crate) trait Vector3TranslationRotation {
    fn translation(&self) -> Vector2<f32>;
    fn rotation(&self) -> f32;
}

impl Vector3TranslationRotation for Vector3<f32> {
    fn translation(&self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }

    fn rotation(&self) -> f32 {
        self.z
    }
}

pub(crate) trait Vector2Screenspace {
    fn convert_screen(&self) -> Vector2<f32>;
}

impl Vector2Screenspace for Vector2<f32> {
    fn convert_screen(&self) -> Vector2<f32> {
        Vector2::new(self.x, HEIGHT as f32 - self.y)
    }
}
