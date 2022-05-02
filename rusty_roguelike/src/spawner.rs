use legion::query::Or;

use crate::prelude::*;

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
