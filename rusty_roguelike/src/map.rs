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
    pub revealed_tiles: Vec<bool>,
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
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

    fn valid_exit(&self, location: Point, delta: Point) -> Option<usize> {
        let destination = location + delta;

        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let index = self.point2d_to_index(destination);
                return Some(index);
            }
        }

        None
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        self.in_bounds(pos)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
        // the usize is a tile index, the f32 the cost to travel there
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(index);

        // check all four directions if they are valid exits for
        // anything on the given tile/position

        // west
        if let Some(index) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((index, 1.0));
        }
        // east
        if let Some(index) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((index, 1.0));
        }
        // north
        if let Some(index) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((index, 1.0));
        }
        // south
        if let Some(index) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((index, 1.0));
        }

        exits
    }

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(self.index_to_point2d(index1), self.index_to_point2d(index2))
    }

    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index as usize] != TileType::Floor
    }
}
