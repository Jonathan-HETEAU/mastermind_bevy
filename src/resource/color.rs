use bevy::prelude::{Color, FromWorld, World};

pub struct MastermindColors {
    pub clear_color: Color,
    pub case_colors: (Color, Color),
    pub pieces_colors: [(Color, Color, Color); 6],
    pub pieces_case_colors: (Color, Color),
    pub result_bad_colors: (Color, Color),
    pub result_good_colors: (Color, Color),
    pub result_case_colors: (Color, Color),
    pub secret_case_hidden_colors: (Color, Color),
}

impl FromWorld for MastermindColors {
    fn from_world(_world: &mut World) -> Self {
        MastermindColors {
            clear_color: Color::BLACK,
            case_colors: (Color::WHITE, Color::WHITE),
            pieces_case_colors: (Color::BLACK, Color::BLACK),
            pieces_colors: [
                (
                    Color::hex("36342F").unwrap(),
                    Color::hex("878377").unwrap(),
                    Color::BLACK,
                ), //BLACK
                (
                    Color::hex("F5F5E9").unwrap(),
                    Color::hex("75756F").unwrap(),
                    Color::BLACK,
                ), //WHITE
                (
                    Color::hex("F5E12C").unwrap(),
                    Color::hex("B8A921").unwrap(),
                    Color::WHITE,
                ), //YELLOW
                (
                    Color::hex("2129DB").unwrap(),
                    Color::hex("252FF5").unwrap(),
                    Color::WHITE,
                ), //BLUE
                (
                    Color::hex("F51000").unwrap(),
                    Color::hex("750800").unwrap(),
                    Color::WHITE,
                ), //RED
                (
                    Color::hex("51F516").unwrap(),
                    Color::hex("3CB510").unwrap(),
                    Color::WHITE,
                ), //GREEN
            ],
            result_bad_colors: (Color::BLACK, Color::GRAY),
            result_good_colors: (Color::WHITE, Color::BLACK),
            result_case_colors: (Color::GRAY, Color::BLACK),
            secret_case_hidden_colors: (Color::BLACK, Color::WHITE),
        }
    }
}
