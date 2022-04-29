use crate::prelude::*;

const PLAYER_SYMBOL: char = '@';

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437(PLAYER_SYMBOL),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                // colemak-dh layout

                //  left
                VirtualKeyCode::R => Point::new(-1, 0),
                // right
                VirtualKeyCode::T => Point::new(1, 0),
                // up
                VirtualKeyCode::F => Point::new(0, -1),
                // down
                VirtualKeyCode::S => Point::new(0, -1),

                _ => Point::zero(),
            };

            // update position
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
