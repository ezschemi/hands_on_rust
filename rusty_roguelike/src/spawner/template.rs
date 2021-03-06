use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

use crate::spawner::DEFAULT_FOV_MONSTER;

const RESOURCES_FILE_PATH: &str = "resources/template.ron";

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open(RESOURCES_FILE_PATH)
            .expect(&format!("Failed to open file: {}", RESOURCES_FILE_PATH));

        from_reader(file).expect("Unable to load entity templates.")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|p| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(p, entity, &mut commands);
            }
        });

        commands.flush(ecs);
    }

    pub fn spawn_entity(
        &self,
        p: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
    ) {
        let entity = commands.push((
            p.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name {
                name: template.name.clone(),
            },
        ));

        println!("Spawning at ({}, {}): {}", p.x, p.y, template.name);

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy {});
                commands.add_component(entity, FieldOfView::new(DEFAULT_FOV_MONSTER));
                commands.add_component(entity, ChasingPlayer {});
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap(),
                        max: template.hp.unwrap(),
                    },
                );
            }
        }

        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "RevealsDungeonMap" => commands.add_component(entity, RevealsDungeonMap {}),
                    _ => {
                        println!("WARNING: dont know how to provide {}.", provides);
                    }
                });
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
    }
}
