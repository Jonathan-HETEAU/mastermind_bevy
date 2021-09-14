use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    math::{Vec2, Vec3},
    prelude::{
        App, ClearColor, Commands, Entity, EventReader, IntoSystem, MouseButton, Msaa,
        OrthographicCameraBundle, ParallelSystemDescriptorCoercion, Query, Res, ResMut, Transform,
    },
    window::{CursorMoved, WindowDescriptor, WindowMode},
    DefaultPlugins,
};

use bevy_prototype_lyon::{entity::ShapeBundle, plugin::ShapePlugin};

mod component;
mod mastermind_shape_bundler;
mod resource;

use mastermind_shape_bundler as MSB;

use component::{
    mouse::MouseState, piece::Piece, position::Position, select::Select, selectable::*,
    selector::Selector,
};

use resource::mastermind::Mastermind;
use resource::state::State;
use resource::{
    color::MastermindColors,
    mastermind::{is_all_some, some_code_to_code},
    structure::Structure,
};

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.
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
        .init_resource::<MastermindColors>()
        .init_resource::<Mastermind>()
        .init_resource::<State>()
        .init_resource::<Structure>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_gameboard.system())
        .add_startup_system(setup_pieces_color.system())
        .add_system(position_from_mouse.system())
        .add_system(selector_from_mouse.system().label("Selector"))
        .add_system(select.system().label("Piece").after("Selector"))
        .add_system(play_code.system().label("Code").after("Piece"))
        .add_system(clean_selector.system().after("Code"))
        .add_system(game_update.system())
        .run();
}

fn position_from_mouse(mut mouse_pos: EventReader<CursorMoved>, query: Query<&mut Selector>) {
    for event in mouse_pos.iter() {
        query.for_each_mut(|mut selector| {
            selector.position = event.position;
        });
    }
}

fn clean_selector(query: Query<&mut Selector>) {
    query.for_each_mut(|mut selector| selector.selected = false);
}

fn selector_from_mouse(
    mut mouse_button: EventReader<MouseButtonInput>,
    query: Query<(&mut MouseState, &mut Selector)>,
) {
    for event in mouse_button.iter() {
        query.for_each_mut(|(mut mouse_state, mut selector)| {
            match (event.button, mouse_state.state, event.state) {
                (MouseButton::Left, ElementState::Pressed, ElementState::Released) => {
                    selector.selected = true;
                    mouse_state.state = ElementState::Released;
                }
                (MouseButton::Left, ElementState::Released, ElementState::Pressed) => {
                    selector.selected = false;
                    mouse_state.state = ElementState::Pressed;
                }
                _ => (),
            }
        });
    }
}

fn select(
    mut cmd: Commands,
    selector: Query<(Entity, &mut Selector)>,
    query: Query<(&Selectable, &Piece)>,
) {
    selector.for_each_mut(|(entity, mut selector)| {
        if selector.selected {
            for (selectable, piece) in query.iter() {
                if selectable.is_selected(&selector.position) {
                    cmd.entity(entity).insert(Select {
                        piece: piece.clone(),
                    });
                    selector.selected = false;
                    break;
                }
            }
        }
    });
}

fn play_code(
    mut cmd: Commands,
    mut game: ResMut<State>,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
    squery: Query<(&Select, &mut Selector)>,
    query: Query<(Entity, &Selectable, &Position)>,
) {
    squery.for_each_mut(|(select, mut selector)| {
        if selector.selected {
            query.for_each_mut(|(entity, selectable, position)| {
                if selectable.is_selected(&selector.position) {
                    if position.row == game.row as u32 {
                        game.code[position.col as usize] = match game.code[position.col as usize] {
                            Some(_) => {
                                cmd.entity(entity).remove_bundle::<ShapeBundle>();
                                Some(select.piece.color.clone())
                            }
                            None => Some(select.piece.color.clone()),
                        };
                        let local_translation = Vec3::new(
                            (position.col as f32) * structure.piece_size,
                            (position.row as f32) * structure.piece_size,
                            0.,
                        ) + structure.boardgame_position;
                        cmd.entity(entity).insert_bundle(MSB::build_piece(
                            Transform {
                                translation: local_translation,
                                ..Transform::default()
                            },
                            colors.pieces_colors[select.piece.color.value()],
                            structure.piece_size,
                        ));
                    }
                    selector.selected = false;
                }
            });
        }
    });
}

fn game_update(
    mut cmd: Commands,
    mut mastermind: ResMut<Mastermind>,
    mut state: ResMut<State>,
    structure: Res<Structure>,
    colors: Res<MastermindColors>,
    query: Query<(Entity, &Selectable)>,
) {
    if is_all_some(&state.code) {
        match mastermind.state.play(some_code_to_code(&state.code)) {
            mastermind::State::Playable(playable) => {
                if state.row != playable.tries.len() {
                    if let Some(tr) = playable.tries.last() {
                        let initial_position = structure.boardgame_position
                            + Vec3::new(
                                -(structure.piece_size / 4.),
                                -(structure.piece_size / 4.),
                                0.,
                            );
                        let local_translation = Vec3::new(
                            structure.piece_size * 4.,
                            state.row as f32 * structure.piece_size,
                            0.,
                        ) + initial_position;
                        let mut i = 0;
                        for _ in 0..tr.good {
                            let tmp = Vec3::new(
                                (i % 2) as f32 * (structure.piece_size / 2.),
                                (i / 2) as f32 * (structure.piece_size / 2.),
                                0.,
                            ) + local_translation;
                            cmd.spawn().insert_bundle(MSB::build_result(
                                Transform {
                                    translation: tmp,
                                    ..Transform::default()
                                },
                                colors.result_good_colors,
                                structure.piece_size,
                            ));
                            i += 1;
                        }
                        for _ in 0..tr.bad {
                            let tmp = Vec3::new(
                                (i % 2) as f32 * (structure.piece_size / 2.),
                                (i / 2) as f32 * (structure.piece_size / 2.),
                                0.,
                            ) + local_translation;
                            cmd.spawn().insert_bundle(MSB::build_result(
                                Transform {
                                    translation: tmp,
                                    ..Transform::default()
                                },
                                colors.result_bad_colors,
                                structure.piece_size,
                            ));
                            i += 1;
                        }
                    }

                    state.row = playable.tries.len();
                    state.code = [Option::None; 4];
                }
            }
            mastermind::State::Finish(finish) => {
                query.for_each(|(entity, _)| {
                    cmd.entity(entity).remove::<Selectable>();
                });
                for col in 0..4 {
                    let initial_position = structure.boardgame_position;
                    let local_translation = Vec3::new(
                        (col as f32) * structure.piece_size,
                        structure.piece_size * 10.,
                        0.,
                    ) + initial_position;
                    cmd.spawn().insert_bundle(MSB::build_piece(
                        Transform {
                            translation: local_translation,
                            ..Transform::default()
                        },
                        colors.pieces_colors[finish.code[col].value()],
                        structure.piece_size,
                    ));
                }
            }
        }
    }
}

pub fn setup(mut commands: Commands, colors: Res<MastermindColors> , structure: Res<Structure>, window : Res<WindowDescriptor>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz((window.width- structure.piece_size) / 2. , (window.height - structure.piece_size) /2.  , 0.);
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

pub fn setup_gameboard(
    mut commands: Commands,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
) {
    let initial_position = structure.boardgame_position;
    for row in 0..10 {
        for col in 0..4 {
            let local_translation = Vec3::new(
                (col as f32) * structure.piece_size,
                (row as f32) * structure.piece_size,
                0.,
            ) + initial_position;
            commands.spawn_bundle(MSB::build_case(
                Transform {
                    translation: local_translation,
                    ..Transform::default()
                },
                colors.case_colors,
                structure.piece_size,
            ));
            commands
                .spawn()
                .insert(Position { row: row, col: col })
                .insert(Selectable::new(
                    1,
                    Vec2::new(
                        local_translation.x + (structure.piece_size / 2.),
                        local_translation.y + (structure.piece_size / 2.),
                    ),
                    SelectableShape::Circle(structure.piece_size / 2.),
                ));
        }
        commands.spawn_bundle(MSB::build_result_case(
            Transform {
                translation: Vec3::new(
                    structure.piece_size * 4.,
                    (row as f32) * structure.piece_size,
                    0.,
                ) + initial_position,
                ..Transform::default()
            },
            colors.result_case_colors,
            structure.piece_size,
        ));
    }
    for col in 0..4 {
        commands.spawn_bundle(MSB::build_secret_case(
            Transform {
                translation: Vec3::new(
                    (col as f32) * structure.piece_size,
                    structure.piece_size * 10.,
                    0.,
                ) + initial_position,
                ..Transform::default()
            },
            colors.secret_case_hidden_colors,
            structure.piece_size,
        ));
    }
}

pub fn setup_pieces_color(
    mut commands: Commands,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
) {
    let initial_position = structure.pieces_position;
    for col in 0..6 {
        let transform = Transform {
            translation: Vec3::new((col as f32) * structure.piece_size, 0., 0.) + initial_position,
            ..Transform::default()
        };
        commands.spawn_bundle(MSB::build_case(
            transform.clone(),
            colors.pieces_case_colors,
            structure.piece_size,
        ));
        commands
            .spawn_bundle(MSB::build_piece(
                transform,
                colors.pieces_colors[col as usize],
                structure.piece_size,
            ))
            .insert(Selectable::new(
                1,
                Vec2::new(
                    transform.translation.x + (structure.piece_size / 2.),
                    transform.translation.y + (structure.piece_size / 2.),
                ),
                SelectableShape::Circle(structure.piece_size / 2.),
            ))
            .insert(Piece {
                color: mastermind::Color::from_value(col as usize),
            });
    }
}
