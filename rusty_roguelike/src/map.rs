use crate::prelude::*;

// the map consists of tiles in a grid pattern.
// Each tile has a type.
// The map is represented as a vector of tiles, with NUM_TILES tiles in it.
//
// Index into the map: this will use row-first encoding. Each row will be stored together
// in x order. The next set of entries will contain the second row.
//  | 0 | 1 | 2 | 3 | 4 | 5 |
//  | 6 | 7 | 8 | 9 | 10 | 11 |
// | 12 | 13 | 14 | 15 | 16 | 17 | 18 |
// ------------------------------------> x-axis

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const WALL_SYMBOL: char = '#';
const FLOOR_SYMBOL: char = '.';

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let index = map_index(x, y);
                    match self.tiles[index] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437(FLOOR_SYMBOL),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437(WALL_SYMBOL),
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }
}
