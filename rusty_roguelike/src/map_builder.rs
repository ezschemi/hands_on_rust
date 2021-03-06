use crate::prelude::*;

const NUM_ROOMS: usize = 5;

pub struct MapBuilder {
    pub map: Map,         // copy of the original map to work with
    pub rooms: Vec<Rect>, // list of rooms that will be added to the map
    pub spawn_locations: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    fn fill(&mut self, tile: TileType) {
        // like in C, the * dereferences, to write to the variable
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    // rooms should not overlap and create NUM_ROOMS rooms
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // for_each runs the provide closure on every x/y-coordinate inside the rectangle it represents
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let index = map_index(p.x, p.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..max(y1, y2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..max(x1, x2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            // randomly dig corridors
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            spawn_locations: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0,
        );

        // the player spawns in the first room, skip it
        mb.rooms.iter().skip(1).for_each(|r| {
            mb.spawn_locations.push(r.center());
        });

        const UNREACHABLE: &f32 = &f32::MAX;

        // look for the most distant tile from the player's position
        mb.amulet_start = mb.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        );

        mb
    }
}
