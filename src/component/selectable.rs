use bevy::math::Vec2;

#[derive(Debug)]
pub enum SelectableShape {
    Circle(f32),
}

pub struct Selectable {
    shape: SelectableShape,
    position: Vec2,
}

impl Selectable {
    pub fn new(position: Vec2, shape: SelectableShape) -> Self {
        Selectable {
            shape,
            position,
        }
    }

    pub fn is_selected(&self, position: &Vec2) -> bool {
        match self.shape {
            SelectableShape::Circle(rayon) => self.position.distance(*position) <= rayon,
        }
    }
}
