use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    window::WindowMode,
};
use mastermind::Game;

use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(WindowDescriptor {
            title: "MasterMind".to_string(),
            width: 450.,
            height: 600.,
            resizable: false,
            vsync: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<Mastermind>()
        .init_resource::<State>()
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

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(200., 275., 0.);
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

fn setup_gameboard(mut commands: Commands) {
    let initial_position = Vec3::new(100., 50., 0.);
    for row in 0..10 {
        for col in 0..4 {
            let local_translation =
                Vec3::new((col as f32) * 50.0, (row as f32) * 50.0, 0.) + initial_position;
            commands.spawn_bundle(MasterMindShapeBundler::build_case(Transform {
                translation: local_translation,
                ..Transform::default()
            }));
            commands
                .spawn()
                .insert(Position { row: row, col: col })
                .insert(Selectable {
                    index: 1,
                    position: Vec2::new(local_translation.x + 25., local_translation.y + 25.),
                    shape: SelectableShape::Circle(25.),
                });
        }
        commands.spawn_bundle(MasterMindShapeBundler::build_result_case(Transform {
            translation: Vec3::new(200., (row as f32) * 50.0, 0.) + initial_position,
            ..Transform::default()
        }));
    }
    for col in 0..4 {
        commands.spawn_bundle(MasterMindShapeBundler::build_secret_case(Transform {
            translation: Vec3::new((col as f32) * 50.0, 500., 0.) + initial_position,
            ..Transform::default()
        }));
    }
}

fn setup_pieces_color(mut commands: Commands, state: Res<State>) {
    let initial_position = Vec3::new(75., 0., 0.);
    for col in 0..6 {
        let transform = Transform {
            translation: Vec3::new((col as f32) * 50.0, 0., 0.) + initial_position,
            ..Transform::default()
        };
        commands.spawn_bundle(MasterMindShapeBundler::build_case(transform.clone()));
        commands
            .spawn_bundle(MasterMindShapeBundler::build_piece(
                transform,
                state.colors[col as usize],
            ))
            .insert(Selectable {
                index: 1,
                position: Vec2::new(transform.translation.x + 25., transform.translation.y + 25.),
                shape: SelectableShape::Circle(25.),
            })
            .insert(Piece {
                color: mastermind::Color::from_value(col as usize),
            });
    }
}

struct MasterMindShapeBundler;

impl MasterMindShapeBundler {
    fn build_case(transform: Transform) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::MAROON, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }
    fn build_result_case(transform: Transform) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::BLACK, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }

    fn build_secret_case(transform: Transform) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::BLACK, Color::YELLOW),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }
    fn build_piece(transform: Transform, color: Color) -> ShapeBundle {
        let shape = shapes::Circle {
            radius: 20.,
            ..shapes::Circle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(color, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            transform,
        )
    }
    fn build_result(transform: Transform, color: Color) -> ShapeBundle {
        let shape = shapes::Circle {
            radius: 10.,
            ..shapes::Circle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(color, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(0.0),
            },
            transform,
        )
    }
}

struct Mastermind {
    state: Game,
}

struct State {
    row: usize,
    code: [Option<mastermind::Color>; 4],
    colors: [Color; 6],
}

impl FromWorld for Mastermind {
    fn from_world(_world: &mut World) -> Self {
        let game = Game::new();
        Mastermind { state: game }
    }
}

impl FromWorld for State {
    fn from_world(_world: &mut World) -> Self {
        State {
            row: 0,
            code: [Option::None; 4],
            colors: [
                Color::BLACK,
                Color::WHITE,
                Color::YELLOW,
                Color::BLUE,
                Color::RED,
                Color::GREEN,
            ],
        }
    }
}

struct MouseState {
    state: ElementState,
}

#[derive(Debug)]
struct Selector {
    position: Vec2,
    selected: bool,
}

struct Select {
    piece: Piece,
}

struct Position {
    row: u32,
    col: u32,
}

#[derive(Debug)]
enum SelectableShape {
    Circle(f32),
}
#[derive(Debug)]
pub struct Selectable {
    shape: SelectableShape,
    position: Vec2,
    index: i32,
}

impl Selectable {
    pub fn is_selected(&self, position: &Vec2) -> bool {
        match self.shape {
            SelectableShape::Circle(rayon) => self.position.distance(*position) <= rayon,
        }
    }
}

#[derive(Clone, Copy)]
struct Piece {
    color: mastermind::Color,
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
                        let initial_position = Vec3::new(100., 50., 0.);
                        let local_translation = Vec3::new(
                            (position.col as f32) * 50.0,
                            (position.row as f32) * 50.0,
                            0.,
                        ) + initial_position;
                        cmd.entity(entity)
                            .insert_bundle(MasterMindShapeBundler::build_piece(
                                Transform {
                                    translation: local_translation,
                                    ..Transform::default()
                                },
                                game.colors[select.piece.color.value()],
                            ));
                    }
                    selector.selected = false;
                }
            });
        }
    });
}

fn is_all_some(tab: &[Option<mastermind::Color>; 4]) -> bool {
    let mut bool = true;
    for code in tab.iter() {
        bool &= code.is_some();
    }
    bool
}

fn some_code_to_code(tab: &[Option<mastermind::Color>; 4]) -> mastermind::Code {
    let mut code: [mastermind::Color; 4] = [
        mastermind::Color::Black,
        mastermind::Color::Black,
        mastermind::Color::Black,
        mastermind::Color::Black,
    ];
    for i in 0..4 {
        if let Some(color) = &tab[i] {
            code[i] = color.clone();
        }
    }
    code as mastermind::Code
}

fn game_update(mut cmd: Commands, mut mastermind: ResMut<Mastermind>, mut state: ResMut<State>) {
    if is_all_some(&state.code) {
        match mastermind.state.play(some_code_to_code(&state.code)) {
            mastermind::State::Playable(playable) => {
                if state.row != playable.tries.len() {
                    if let Some(tr) = playable.tries.last() {
                        let initial_position = Vec3::new(100. - 12.5, 50. - 12.5, 0.);
                        let local_translation =
                            Vec3::new(200., state.row as f32 * 50.0, 0.) + initial_position;
                        let mut i = 0;
                        for _ in 0..tr.good {
                            let tmp = Vec3::new((i % 2) as f32 * 25., (i / 2) as f32 * 25.0, 0.)
                                + local_translation;
                            cmd.spawn()
                                .insert_bundle(MasterMindShapeBundler::build_result(
                                    Transform {
                                        translation: tmp,
                                        ..Transform::default()
                                    },
                                    Color::GREEN,
                                ));
                            i += 1;
                        }
                        for _ in 0..tr.bad {
                            let tmp = Vec3::new((i % 2) as f32 * 25., (i / 2) as f32 * 25.0, 0.)
                                + local_translation;
                            cmd.spawn()
                                .insert_bundle(MasterMindShapeBundler::build_result(
                                    Transform {
                                        translation: tmp,
                                        ..Transform::default()
                                    },
                                    Color::YELLOW,
                                ));
                            i += 1;
                        }
                    }

                    state.row = playable.tries.len();
                    state.code = [Option::None; 4];
                }
            }
            mastermind::State::Finish(finish) => {
                for col in 0..4 {
                    let initial_position = Vec3::new(100., 50., 0.);
                    let local_translation =
                        Vec3::new((col as f32) * 50.0, 500., 0.) + initial_position;
                    cmd.spawn()
                        .insert_bundle(MasterMindShapeBundler::build_piece(
                            Transform {
                                translation: local_translation,
                                ..Transform::default()
                            },
                            state.colors[finish.code[col].value()],
                        ));
                }
            }
        }
    }
}
