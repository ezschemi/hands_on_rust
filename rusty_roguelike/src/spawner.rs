use legion::query::Or;

use crate::prelude::*;

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
        ),
    );
}

fn ettin() -> (i32, String, FontCharType) {
    let hitpoints = 6;
    (hitpoints, "Ettin".to_string(), to_cp437('E'))
}
fn ogre() -> (i32, String, FontCharType) {
    let hitpoints = 4;
    (hitpoints, "Ogre".to_string(), to_cp437('O'))
}
fn orc() -> (i32, String, FontCharType) {
    let hitpoints = 2;
    (hitpoints, "Orc".to_string(), to_cp437('o'))
}
fn goblin() -> (i32, String, FontCharType) {
    let hitpoints = 1;
    (hitpoints, "Goblin".to_string(), to_cp437('g'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=6 => goblin(),
        6..=7 => orc(),
        7..=8 => ogre(),
        _ => ettin(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name { name },
        FieldOfView::new(DEFAULT_FOV_MONSTER),
    ));
}

pub fn spawn_monster_ettin(ecs: &mut World, pos: Point) {
    let (hp, name, glyph) = ettin();

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        ChasingPlayer {},
        Health {
            current: hp,
            max: hp,
        },
        Name { name },
        FieldOfView::new(DEFAULT_FOV_MONSTER),
    ));
}

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

pub fn spawn_healing_potion(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('!'),
        },
        Name {
            name: "Basic Healing Potion".to_string(),
        },
        ProvidesHealing { amount: 6 },
    ));
}

pub fn spawn_map_revealer(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('{'),
        },
        Name {
            name: "Dungeon Map".to_string(),
        },
        RevealsDungeonMap,
    ));
}

pub fn spawn_entity(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let roll = rng.roll_dice(1, 6);
    match roll {
        1 => spawn_healing_potion(ecs, pos),
        2 => spawn_map_revealer(ecs, pos),
        _ => spawn_monster(ecs, rng, pos),
    }
}
