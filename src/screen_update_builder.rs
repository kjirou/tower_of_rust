use crate::enums::{ColorKind};
use crate::models::field::Field;
use crate::models::field_element::FieldElement;
use crate::models::field_object::DisplayKind;
use crate::models::game::Game;
use crate::screen_update::{MapElementUpdate, ScreenUpdate};
use crate::types::{RectangleSize, XYCoordinates};
use crate::utils::{translate_rectangle_on_field};

fn create_field_element_display(field_element: &FieldElement) -> MapElementUpdate {
    let mut symbol = '.';
    let mut foreground = ColorKind::LightBlack;
    let background = ColorKind::Black;

    if field_element.field_objects.len() > 0 {
        let first = &field_element.field_objects[0];
        match first.display_kind {
            DisplayKind::Hero => {
                symbol = '@';
                foreground = ColorKind::Magenta;
            },
            DisplayKind::Wall => {
                symbol = '#';
                foreground = ColorKind::LightBlack;
            },
        }
    }

    MapElementUpdate {
        symbol,
        foreground,
        background,
    }
}

pub fn build(field: &Field, game: &Game) -> ScreenUpdate {
    // Last Key Input
    let last_key_input: String = match &game.last_key_input {
        // TODO: 雑。不慮の文字が入りうる。
        Some(key_input) => format!("{:?}", key_input),
        None => String::from("None"),
    };

    // Map
    let map_size: RectangleSize = (21, 13);
    let mut map: Vec<Vec<MapElementUpdate>> = vec![];
    let hero_xy: Option<XYCoordinates> = match &game.operation_target {
        Some(operation_target) => Some(translate_rectangle_on_field(&map_size, &operation_target.0)),
        _ => None,
    };
    for map_y in 0..map_size.1 {
        let mut map_row: Vec<MapElementUpdate> = vec![];
        for map_x in 0..map_size.0 {
            let map_element_update: MapElementUpdate = match hero_xy {
                Some(hero_xy) => {
                    let xy = (hero_xy.0 + map_x as i32, hero_xy.1 + map_y as i32);
                    let field_element = field.find_field_element_by_xy(&xy);
                    match field_element {
                        Some(field_element) => create_field_element_display(field_element),
                        _ => MapElementUpdate {
                            symbol: '#',
                            foreground: ColorKind::LightBlack,
                            background: ColorKind::Black,
                        },
                    }
                },
                _ => {
                    MapElementUpdate {
                        symbol: '?',
                        foreground: ColorKind::White,
                        background: ColorKind::Black,
                    }
                },
            };
            map_row.push(map_element_update);
        }
        map.push(map_row);
    }

    ScreenUpdate {
        last_key_input,
        map,
    }
}
