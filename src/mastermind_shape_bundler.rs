use bevy::prelude::{Color, Transform};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};

pub fn build_case(transform: Transform, colors: (Color, Color), size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        width: size,
        height: size,
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
pub fn build_result_case(transform: Transform, colors: (Color, Color), size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        width: size,
        height: size,
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

pub fn build_secret_case(transform: Transform, colors: (Color, Color), size: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        width: size,
        height: size,
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
pub fn build_piece(transform: Transform, colors: (Color, Color, Color), size: f32) -> ShapeBundle {
    let shape = shapes::Circle {
        radius: size * 0.4,
        ..shapes::Circle::default()
    };
    GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(colors.0, colors.1),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(size * 0.1),
        },
        transform,
    )
}
pub fn build_result(transform: Transform, colors: (Color, Color), size: f32) -> ShapeBundle {
    let shape = shapes::Circle {
        radius: size * 0.16,
        ..shapes::Circle::default()
    };
    GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(colors.0, colors.1),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(size * 0.08),
        },
        transform,
    )
}
