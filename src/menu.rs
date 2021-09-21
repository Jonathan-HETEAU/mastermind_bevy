use bevy::prelude::*;

use crate::{
    loading::AssetsLoading,
    resource::{button::ButtonMaterials, snapshots::Snapshots},
    state::AppState,
};
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::Menu).with_system(setup.system().label("Setup")),
            )
            .add_system_set(SystemSet::on_pause(AppState::Menu).with_system(clear.system()))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(clear.system()))
            .add_system_set(SystemSet::on_resume(AppState::Menu).with_system(resume.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(button_system.system()),
            );
    }
}

struct AssetsMenu {
    font: Handle<Font>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut app_state: ResMut<State<AppState>>,
) {
    info!("menu::setup");
    let assets_menu = AssetsMenu {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    };
    let mut loader = AssetsLoading::new();
    loader.add(assets_menu.font.clone_untyped());
    commands.insert_resource(loader);
    commands.insert_resource(assets_menu);
    app_state.push(AppState::Loading).unwrap();
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                app_state.push(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                text.sections[0].value = "PLAY".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "play".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn clear(mut cmds: Commands, mut snapshots: ResMut<Snapshots>) {
    info!("menu::clean");
    if let Some(entities) = snapshots.get_mut_snap(&String::from("Menu")) {
        for entity in entities.iter() {
            cmds.entity(*entity).despawn_recursive();
        }
        entities.clear();
    }
}

fn resume(
    mut cmds: Commands,
    mut snapshots: ResMut<Snapshots>,
    button_materials: Res<ButtonMaterials>,
    assets: Res<AssetsMenu>,
) {
    info!("menu::resume");
    let mut entities: Vec<Entity> = Vec::new();
    entities.push(cmds.spawn_bundle(UiCameraBundle::default()).id());
    entities.push(
        cmds.spawn()
            .insert_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn().insert_bundle(TextBundle {
                    text: Text::with_section(
                        "play",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
            })
            .id(),
    );
    snapshots.snap(&String::from("Menu"), entities);
}
