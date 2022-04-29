use crate::prelude::*;

const PLAYER_SYMBOL: char = '@';

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);

        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437(PLAYER_SYMBOL),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
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

                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),

                _ => Point::zero(),
            };

            // update position
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}
