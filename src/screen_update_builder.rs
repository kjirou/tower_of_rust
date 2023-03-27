use crate::enums::{ColorKind};
use crate::mediators::{find_operation_target};
use crate::models::field::Field;
use crate::models::field_element::FieldElement;
use crate::models::field_object::DisplayKind;
use crate::models::game::Game;
use crate::screen_update::{MapElementUpdate, ScreenUpdate};
use crate::types::{RectangleSize, XYCoordinates};
use crate::utils::{compute_map_xy_on_field};

fn create_field_element_display(field_element: &FieldElement) -> MapElementUpdate {
    let mut symbol = '.';
    let mut foreground = ColorKind::LightBlack;
    let mut background = ColorKind::Black;

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

    // TODO: 自分の攻撃と敵の攻撃で着色する。
    // TODO: 自分の攻撃と敵の攻撃が重なった時は自分側の着色を優先する。
    if field_element.field_effects.len() > 0 {
        background = ColorKind::LightCyan;
    }

    MapElementUpdate {
        symbol,
        foreground,
        background,
    }
}

pub fn build(field: &Field, game: &Game) -> ScreenUpdate {
    let operation_target = find_operation_target(field, game);

    // Map
    let map_size: RectangleSize = (21, 13);
    let mut map: Vec<Vec<MapElementUpdate>> = vec![];
    let map_xy_on_field: Option<XYCoordinates> = match &game.operation_target_location {
        Some(operation_target_location) => Some(compute_map_xy_on_field(&map_size, &operation_target_location.0)),
        _ => None,
    };
    for y_on_map in 0..map_size.1 {
        let mut map_row: Vec<MapElementUpdate> = vec![];
        for x_on_map in 0..map_size.0 {
            let map_element_update: MapElementUpdate = match map_xy_on_field {
                Some(map_xy_on_field) => {
                    let xy_on_field = (map_xy_on_field.0 + x_on_map as i32, map_xy_on_field.1 + y_on_map as i32);
                    let field_element = field.find_field_element_by_xy(&xy_on_field);
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

    // The last key input
    let last_key_input: String = match &game.last_key_input {
        // TODO: 雑。不慮の文字が入りうる。
        Some(key_input) => format!("{:?}", key_input),
        None => String::from("None"),
    };

    // The direction of the operation target
    let direction_of_operation_target: String = if let Some(operation_target) = operation_target {
        operation_target.direction.to_string()
    } else {
        String::from("None")
    };

    ScreenUpdate {
        number_of_frames: game.number_of_frames,
        fps: game.get_fps(),
        last_key_input,
        direction_of_operation_target,
        map,
    }
}
