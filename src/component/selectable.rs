use bevy::math::Vec2;

#[derive(Debug)]
pub enum SelectableShape {
    Circle(f32),
}
#[derive(Debug)]
pub struct Selectable {
    shape: SelectableShape,
    position: Vec2,
    index: i32,
}

impl Selectable {

    pub fn new(index:i32,position:Vec2 , shape:SelectableShape)-> Self {
        Selectable { shape, position, index}
    }

    pub fn is_selected(&self, position: &Vec2) -> bool {
        match self.shape {
            SelectableShape::Circle(rayon) => self.position.distance(*position) <= rayon,
        }
    }
}