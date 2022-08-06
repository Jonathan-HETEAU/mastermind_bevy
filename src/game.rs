use crate::component::mouse::MouseState;
use crate::component::piece::Piece;
use crate::component::position::Position;
use crate::component::select::Select;
use crate::component::selectable::{Selectable, SelectableShape};
use crate::component::selector::Selector;
use crate::mastermind_shape_bundler as MSB;
use crate::resource::button::ButtonMaterials;
use crate::resource::color::MastermindColors;
use crate::resource::mastermind::{is_all_some, some_code_to_code};
use crate::resource::snapshots::Snapshots;
use crate::resource::state::State as MState;
use crate::resource::assets::Assets as MAssets;
use crate::resource::structure::Structure;
use crate::{resource::mastermind::Mastermind, state::AppState};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(setup.system().label("Setup"))
                .with_system(draw_background.system().after("Setup"))
                .with_system(draw_pieces.system().after("Setup"))
                .with_system(draw_ui.system().after("Setup")),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(position_from_mouse.system())
                .with_system(selector_from_mouse.system().label("Selector"))
                .with_system(select.system().label("Piece").after("Selector"))
                .with_system(play_code.system().label("Code").after("Piece"))
                .with_system(button_system.system())
                .with_system(clean_selector.system().after("Code"))
                .with_system(game_update.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(clear.system()));
    }
}

pub fn setup(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut snapshots: ResMut<Snapshots>,
) {
    commands.insert_resource(Mastermind::new());
    commands.insert_resource(MState::new());
    snapshots.snap(&String::from("Game"), Vec::new());
    app_state.push(AppState::Loading).unwrap();
}

pub fn draw_ui(
    mut cmds: Commands,
    mut snapshots: ResMut<Snapshots>,
    button_materials: Res<ButtonMaterials>,
    assets: Res<MAssets>,
) {
    let entities = snapshots.get_mut_snap(&String::from("Game")).unwrap();
    entities.push(cmds.spawn_bundle(UiCameraBundle::default()).id());
    entities.push(
        cmds.spawn()
            .insert_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(75.0), Val::Px(50.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(0.0),
                        top: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: button_materials.alerte.clone(),
                ..Default::default()
            }).with_children(|parent| {
                parent.spawn().insert_bundle(TextBundle {
                    text: Text::with_section(
                        "  ...  ",
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        TextAlignment{
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(ActionButton::new(|state| state.pop().unwrap()))
            .id(),
    );
}

pub fn draw_background(
    mut commands: Commands,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
    mut snapshots: ResMut<Snapshots>,
) {
    let entities = snapshots.get_mut_snap(&String::from("Game")).unwrap();

    let initial_position = structure.boardgame_position;
    for row in 0..10 {
        for col in 0..4 {
            let local_translation = Vec3::new(
                (col as f32) * structure.piece_size,
                (row as f32) * structure.piece_size,
                0.,
            ) + initial_position;
            entities.push(
                commands
                    .spawn_bundle(MSB::build_case(
                        Transform {
                            translation: local_translation,
                            ..Transform::default()
                        },
                        colors.case_colors,
                        structure.piece_size,
                    ))
                    .id(),
            );
            entities.push(
                commands
                    .spawn()
                    .insert(Position { row: row, col: col })
                    .insert(Selectable::new(
                        Vec2::new(
                            local_translation.x + (structure.piece_size / 2.),
                            local_translation.y + (structure.piece_size / 2.),
                        ),
                        SelectableShape::Circle(structure.piece_size / 2.),
                    ))
                    .id(),
            );
        }
        entities.push(
            commands
                .spawn_bundle(MSB::build_result_case(
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
                ))
                .id(),
        );
    }
    for col in 0..4 {
        entities.push(
            commands
                .spawn_bundle(MSB::build_secret_case(
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
                ))
                .id(),
        );
    }
}

pub fn draw_pieces(
    mut commands: Commands,
    colors: Res<MastermindColors>,
    structure: Res<Structure>,
    mut snapshots: ResMut<Snapshots>,
) {
    let entities = snapshots.get_mut_snap(&String::from("Game")).unwrap();
    let initial_position = structure.pieces_position;
    for col in 0..6 {
        let transform = Transform {
            translation: Vec3::new((col as f32) * structure.piece_size, 0., 0.) + initial_position,
            ..Transform::default()
        };
        entities.push(
            commands
                .spawn_bundle(MSB::build_case(
                    transform.clone(),
                    colors.pieces_case_colors,
                    structure.piece_size,
                ))
                .id(),
        );
        entities.push(
            commands
                .spawn_bundle(MSB::build_piece(
                    transform,
                    colors.pieces_colors[col as usize],
                    structure.piece_size,
                ))
                .insert(Selectable::new(
                    Vec2::new(
                        transform.translation.x + (structure.piece_size / 2.),
                        transform.translation.y + (structure.piece_size / 2.),
                    ),
                    SelectableShape::Circle(structure.piece_size / 2.),
                ))
                .insert(Piece {
                    color: mastermind::Color::from_value(col as usize),
                })
                .id(),
        );
    }
}

fn clear(mut cmds: Commands, mut snapshots: ResMut<Snapshots>) {
    if let Some(entities) = snapshots.get_mut_snap(&String::from("Game")) {
        for entity in entities.iter() {
            cmds.entity(*entity).despawn_recursive();
        }
        entities.clear();
    }
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
    mut game: ResMut<MState>,
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
    mut state: ResMut<MState>,
    structure: Res<Structure>,
    colors: Res<MastermindColors>,
    query: Query<(Entity, &Selectable)>,
    mut snapshots: ResMut<Snapshots>,
) {
    let entities = snapshots.get_mut_snap(&String::from("Game")).unwrap();
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
                            entities.push(
                                cmd.spawn()
                                    .insert_bundle(MSB::build_result(
                                        Transform {
                                            translation: tmp,
                                            ..Transform::default()
                                        },
                                        colors.result_good_colors,
                                        structure.piece_size,
                                    ))
                                    .id(),
                            );
                            i += 1;
                        }
                        for _ in 0..tr.bad {
                            let tmp = Vec3::new(
                                (i % 2) as f32 * (structure.piece_size / 2.),
                                (i / 2) as f32 * (structure.piece_size / 2.),
                                0.,
                            ) + local_translation;
                            entities.push(
                                cmd.spawn()
                                    .insert_bundle(MSB::build_result(
                                        Transform {
                                            translation: tmp,
                                            ..Transform::default()
                                        },
                                        colors.result_bad_colors,
                                        structure.piece_size,
                                    ))
                                    .id(),
                            );
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
                    entities.push(
                        cmd.spawn()
                            .insert_bundle(MSB::build_piece(
                                Transform {
                                    translation: local_translation,
                                    ..Transform::default()
                                },
                                colors.pieces_colors[finish.code[col].value()],
                                structure.piece_size,
                            ))
                            .id(),
                    );
                }
                
            }
        }
    }
}

struct ActionButton {
    pub clicked: fn(&mut ResMut<State<AppState>>),
}

impl ActionButton {
    fn new(clicked: fn(&mut ResMut<State<AppState>>)) -> Self {
        Self { clicked }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &ActionButton,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<State<AppState>>,
    
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, action, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                (action.clicked)(&mut app_state);
            }
            Interaction::Hovered => {
                text.sections[0].value = "MENU".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.sections[0].value = "  ...  ".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}
