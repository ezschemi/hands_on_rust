use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(FieldOfView)]
#[read_component(Player)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut positions = <(Entity, &Point, &Name)>::query();

    let mut fovs = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fovs.iter(ecs).nth(0).unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos))
        .for_each(|(entity, _, name)| {
            // the tooltip layer is 4x larger, so multiply the mouse position by
            // 4 to get the screen position for the tooltip layer.
            let screen_pos = *mouse_pos * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{}: {} hp", &name.name, health.current)
                } else {
                    name.name.clone()
                };
            draw_batch.print(screen_pos, &display);
        });

    draw_batch.submit(10100).expect("Batch Error");
}
