use bevy::input::ElementState;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_prototype_lyon::plugin::ShapePlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;

mod component;
mod mastermind_shape_bundler;
mod resource;

mod game;
mod loading;
mod menu;
mod state;

use crate::state::AppState;
use game::GamePlugin;
use resource::{snapshots::Snapshots, structure::Structure};

use component::{mouse::MouseState, selector::Selector};

use resource::color::MastermindColors;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "MasterMind".to_string(),
            width: 300.,
            height: 600.,
            resizable: false,
            vsync: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<Snapshots>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .init_resource::<Structure>()
        .init_resource::<MastermindColors>()
        .add_state(AppState::Menu)
        .add_system(setup.system())
        .add_system_set(SystemSet::on_enter(AppState::Restart).with_system(restart.system()))
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

pub fn setup(
    mut commands: Commands,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
    window: Res<WindowDescriptor>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(
        (window.width - structure.piece_size) / 2.,
        (window.height - structure.piece_size) / 2.,
        0.,
    );
    commands.insert_resource(ClearColor(colors.clear_color));
    commands.spawn_bundle(camera);
    commands
        .spawn()
        .insert(MouseState {
            state: ElementState::Released,
        })
        .insert(Selector {
            position: Vec2::default(),
            selected: false,
        });
}

fn restart(mut state: ResMut<State<AppState>>) {
    state.set(AppState::InGame).unwrap();
}
