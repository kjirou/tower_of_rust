use crate::models::field::Field;
use crate::screen::MapElementUpdate;
use crate::screen::ScreenUpdate;

pub fn build(field: &Field) -> ScreenUpdate {
    let map_size = (21, 13);  // width, height
    let mut map: Vec<Vec<MapElementUpdate>> = vec![];

    for map_y in 0..map_size.1 {
        let mut map_row: Vec<MapElementUpdate> = vec![];
        for map_x in 0..map_size.0 {
            // TODO: Hero 表示位置が常に Map 中央になるように調整する。
            // TODO: Field の範囲を超えた時に、何かで埋める。
            let xy = (map_x, map_y);
            let field_element = field.get_field_element(&xy);
            let symbol = field_element.get_display();
            map_row.push(MapElementUpdate {
                symbol,
                foreground: String::from(""),
                background: String::from(""),
            });
        }
        map.push(map_row);
    }

    ScreenUpdate {
        map,
    }
}
