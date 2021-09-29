use crate::enums::{ColorKind};
use crate::models::field::Field;
use crate::models::game::Game;
use crate::screen_update::{MapElementUpdate, ScreenUpdate};
use crate::types::{RectangleSize, XYCoordinates};
use crate::utils::{translate_rectangle_on_field};

pub fn build(field: &Field, game: &Game) -> ScreenUpdate {
    let map_size: RectangleSize = (21, 13);
    let mut map: Vec<Vec<MapElementUpdate>> = vec![];
    let map_xy: Option<XYCoordinates> = match &game.operation_target {
        Some(operation_target) => Some(translate_rectangle_on_field(&map_size, &operation_target.0)),
        _ => None,
    };
    for map_y in 0..map_size.1 {
        let mut map_row: Vec<MapElementUpdate> = vec![];
        for map_x in 0..map_size.0 {
            let map_element_update: MapElementUpdate = match map_xy {
                Some(map_xy) => {
                    let xy = (map_xy.0 + map_x as i32, map_xy.1 + map_y as i32);
                    let field_element = field.find_field_element_by_xy(&xy);
                    match field_element {
                        Some(field_element) => MapElementUpdate {
                            symbol: field_element.get_display(),
                            foreground: ColorKind::White,
                            background: ColorKind::Black,
                        },
                        _ => MapElementUpdate {
                            symbol: 'X',
                            foreground: ColorKind::White,
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
        map,
    }
}
