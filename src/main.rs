use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    window::WindowMode,
};
use mastermind::Game;

use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

struct MastermindColors {
    clear_color: Color,
    case_colors: (Color, Color),
    pieces_colors: [(Color, Color, Color); 6],
    pieces_case_colors: (Color, Color),
    result_bad_colors: (Color, Color),
    result_good_colors: (Color, Color),
    result_case_colors: (Color, Color),
    secret_case_hidden_colors: (Color, Color),
}

impl FromWorld for MastermindColors {
    fn from_world(_world: &mut World) -> Self {
        MastermindColors {
            clear_color: Color::BLACK,
            case_colors: (Color::WHITE, Color::WHITE),
            pieces_case_colors:(Color::BLACK, Color::BLACK),
            pieces_colors: [
                (Color::hex("36342F").unwrap(), Color::hex("878377").unwrap() , Color::BLACK),//BLACK
                (Color::hex("F5F5E9").unwrap(), Color::hex("75756F").unwrap() , Color::BLACK),//WHITE
                (Color::hex("F5E12C").unwrap(), Color::hex("B8A921").unwrap() , Color::WHITE),//YELLOW
                (Color::hex("2129DB").unwrap(), Color::hex("252FF5").unwrap() , Color::WHITE),//BLUE
                (Color::hex("F51000").unwrap(), Color::hex("750800").unwrap() , Color::WHITE),//RED
                (Color::hex("51F516").unwrap(), Color::hex("3CB510").unwrap() , Color::WHITE),//GREEN
            ],
            result_bad_colors: (Color::BLACK, Color::GRAY),
            result_good_colors: (Color::WHITE, Color::BLACK),
            result_case_colors: (Color::GRAY, Color::BLACK ),
            secret_case_hidden_colors: (Color::BLACK, Color::WHITE),
        }
    }
}

/// This example illustrates how to create a button that changes color and text based on its
/// interaction state.
fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(WindowDescriptor {
            title: "MasterMind".to_string(),
            width: 450.,
            height: 600.,
            resizable: false,
            vsync: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<MastermindColors>()
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

fn setup(mut commands: Commands, colors: Res<MastermindColors>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_xyz(200., 275., 0.);
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

fn setup_gameboard(mut commands: Commands, colors: Res<MastermindColors>) {
    let initial_position = Vec3::new(100., 50., 0.);
    for row in 0..10 {
        for col in 0..4 {
            let local_translation =
                Vec3::new((col as f32) * 50.0, (row as f32) * 50.0, 0.) + initial_position;
            commands.spawn_bundle(MasterMindShapeBundler::build_case(
                Transform {
                    translation: local_translation,
                    ..Transform::default()
                },
                colors.case_colors,
            ));
            commands
                .spawn()
                .insert(Position { row: row, col: col })
                .insert(Selectable {
                    index: 1,
                    position: Vec2::new(local_translation.x + 25., local_translation.y + 25.),
                    shape: SelectableShape::Circle(25.),
                });
        }
        commands.spawn_bundle(MasterMindShapeBundler::build_result_case(
            Transform {
                translation: Vec3::new(200., (row as f32) * 50.0, 0.) + initial_position,
                ..Transform::default()
            },
            colors.result_case_colors,
        ));
    }
    for col in 0..4 {
        commands.spawn_bundle(MasterMindShapeBundler::build_secret_case(
            Transform {
                translation: Vec3::new((col as f32) * 50.0, 500., 0.) + initial_position,
                ..Transform::default()
            },
            colors.secret_case_hidden_colors,
        ));
    }
}

fn setup_pieces_color(mut commands: Commands, colors: Res<MastermindColors>) {
    let initial_position = Vec3::new(75., 0., 0.);
    for col in 0..6 {
        let transform = Transform {
            translation: Vec3::new((col as f32) * 50.0, 0., 0.) + initial_position,
            ..Transform::default()
        };
        commands.spawn_bundle(MasterMindShapeBundler::build_case(
            transform.clone(),
            colors.pieces_case_colors,
        ));
        commands
            .spawn_bundle(MasterMindShapeBundler::build_piece(
                transform,
                colors.pieces_colors[col as usize],
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
    fn build_case(transform: Transform, colors: (Color, Color)) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(colors.0, colors.1),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }
    fn build_result_case(transform: Transform, colors: (Color, Color)) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(colors.0, colors.1),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }

    fn build_secret_case(transform: Transform, colors: (Color, Color)) -> ShapeBundle {
        let shape = shapes::Rectangle {
            width: 50.,
            height: 50.,
            ..shapes::Rectangle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(colors.0, colors.1),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            transform,
        )
    }
    fn build_piece(transform: Transform, colors: (Color, Color, Color)) -> ShapeBundle {
        let shape = shapes::Circle {
            radius: 20.,
            ..shapes::Circle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(colors.0, colors.1),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(5.0),
            },
            transform,
        )
    }
    fn build_result(transform: Transform, colors: (Color, Color)) -> ShapeBundle {
        let shape = shapes::Circle {
            radius: 8.,
            ..shapes::Circle::default()
        };
        GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(colors.0, colors.1),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(4.0),
            },
            transform,
        )
    }
}

struct Mastermind {
    state: Game,
}

impl FromWorld for Mastermind {
    fn from_world(_world: &mut World) -> Self {
        let game = Game::new();
        Mastermind { state: game }
    }
}
struct State {
    row: usize,
    code: [Option<mastermind::Color>; 4],
}

impl FromWorld for State {
    fn from_world(_world: &mut World) -> Self {
        State {
            row: 0,
            code: [Option::None; 4],
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
    colors: Res<MastermindColors>,
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
                                colors.pieces_colors[select.piece.color.value()],
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

fn game_update(
    mut cmd: Commands,
    mut mastermind: ResMut<Mastermind>,
    mut state: ResMut<State>,
    colors: Res<MastermindColors>,
) {
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
                                    colors.result_good_colors,
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
                                    colors.result_bad_colors,
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
                            colors.pieces_colors[finish.code[col].value()],
                        ));
                }
            }
        }
    }
}
