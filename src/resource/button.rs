use bevy::prelude::*;

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub alerte: Handle<ColorMaterial>,
    //pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            alerte: materials.add(Color::hex("750800").unwrap().into()),
            //pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
