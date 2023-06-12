use nalgebra::Vector2;
use raqote::PathBuilder;
use super::util::Vector2Screenspace;

pub trait VectorPathBuilder {
    fn move_to_vec(&mut self, pos: Vector2<f32>);
    fn line_to_vec(&mut self, pos: Vector2<f32>);
}

impl VectorPathBuilder for PathBuilder {
    fn move_to_vec(&mut self, pos: Vector2<f32>) {
        self.move_to(pos.convert_screen().x, pos.convert_screen().y);
    }

    fn line_to_vec(&mut self, pos: Vector2<f32>) {
        self.line_to(pos.convert_screen().x, pos.convert_screen().y);
    }
}
