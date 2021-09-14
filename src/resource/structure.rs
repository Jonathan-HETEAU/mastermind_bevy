use bevy::{
    math::Vec3,
    prelude::{FromWorld, World},
    window::{WindowDescriptor},
};

pub struct Structure {
    pub piece_size: f32,
    pub pieces_position: Vec3,
    pub boardgame_position: Vec3,
    pub secrets_position: Vec3,
}

impl FromWorld for Structure {
    fn from_world(world: &mut World) -> Self {
        let window = world.get_resource::<WindowDescriptor>().unwrap();
        let size = (window.width, window.height);
        let piece_size = (size.0 / 6.).min(size.1 / 12.);
        let initial_positon: Vec3 = Vec3::new(
            (size.0 - (piece_size * 6.)) / 2.,
            (size.1 - (piece_size * 12.)) / 2.,
            0.,
        );
        Structure {
            piece_size: piece_size,
            pieces_position: initial_positon,
            boardgame_position: initial_positon + Vec3::new(piece_size / 2., piece_size, 0.),
            secrets_position: initial_positon + Vec3::new(piece_size / 2., piece_size * 10., 0.),
        }
    }
}
