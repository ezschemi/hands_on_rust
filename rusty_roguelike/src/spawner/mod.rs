mod template;

use crate::prelude::*;
use template::Templates;

const DEFAULT_FOV_PLAYER: i32 = 8;
const DEFAULT_FOV_MONSTER: i32 = 5;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        // this creates a new tuple
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health {
                current: 20,
                max: 20,
            },
            Name {
                name: "Player".to_string(),
            },
            FieldOfView::new(DEFAULT_FOV_PLAYER),
            Damage(1),
        ),
    );
}

// fn ettin() -> (i32, String, FontCharType) {
//     let hitpoints = 6;
//     (hitpoints, "Ettin".to_string(), to_cp437('E'))
// }
// fn ogre() -> (i32, String, FontCharType) {
//     let hitpoints = 4;
//     (hitpoints, "Ogre".to_string(), to_cp437('O'))
// }
// fn orc() -> (i32, String, FontCharType) {
//     let hitpoints = 2;
//     (hitpoints, "Orc".to_string(), to_cp437('o'))
// }
// fn goblin() -> (i32, String, FontCharType) {
//     let hitpoints = 1;
//     (hitpoints, "Goblin".to_string(), to_cp437('g'))
// }

pub fn spawn_amulet_of_yala(ecs: &mut World, position: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name {
            name: "Amulet of Yala".to_string(),
        },
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let templates = Templates::load();
    templates.spawn_entities(ecs, rng, level, spawn_points);
}
