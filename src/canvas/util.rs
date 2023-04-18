use nalgebra::{Vector2, Vector3};

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
