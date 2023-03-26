use crate::prelude::*;





pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RoomDatasContainer::default())
            .insert_resource(RoomEntities::default())
            .insert_resource(RoomGenerationWatcher::default())
            .add_system(build_rooms);
    }
}





#[derive(Resource, SmartDefault, Deref, DerefMut)]
pub struct RoomGenerationWatcher (pub Watcher<PlayerRoomChange>);

pub fn build_rooms (
    mut room_generation_watcher: ResMut<RoomGenerationWatcher>,
    player_room_change: Res<PlayerRoomChange>,
    mut room_entities: ResMut<RoomEntities>,
    mut room_datas_container: ResMut<RoomDatasContainer>,
    player: Res<Player>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    if room_generation_watcher.event_has_happened(&player_room_change) {return;}

    // remove room entities
    let mut slot_positions_to_remove = vec!();
    for &slot_pos in room_entities.keys() {
        let (dx, dy) = (slot_pos.x - player.room.x, slot_pos.y - player.room.y);
        if dx * dx + dy * dy < GENERATION_PATTERN_RADIUS * GENERATION_PATTERN_RADIUS {continue;}
        slot_positions_to_remove.push(slot_pos);
    }
    for slot_pos in slot_positions_to_remove {
        remove_room_entities(&mut room_entities, slot_pos, &mut commands);
    }

    // create room entities
    for curr_offset in GENERATION_PATTERN {

        if curr_offset.x * curr_offset.x + curr_offset.y * curr_offset.y > GENERATION_PATTERN_RADIUS * GENERATION_PATTERN_RADIUS {continue;}
        let curr_pos = player.room + curr_offset;
        if room_entities.contains_key(&curr_pos) {continue;}

        generate_room_entities(&mut room_entities, &mut room_datas_container, curr_pos, &mut commands, &mut meshes, &mut materials, &asset_server);

    }

}





pub fn generate_room_entities (room_entities: &mut HashMap<IVec2, Vec<Entity>>, room_datas_container: &mut RoomDatasContainer, pos: IVec2, commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, asset_server: &AssetServer) {
    let room_data = room_datas_container.get(pos);
    let mut entities = vec!();

    // floor
    entities.push(fns::create_textured_mesh(
        Transform::from_xyz(pos.x as f32, 0., pos.y as f32),
        Mesh::from(shape::Box::new(1., 0.1, 1.)),
        "carpet.png",
        commands, meshes, materials, asset_server,
    ));

    // ceiling
    entities.push(fns::create_textured_mesh(
        Transform::from_xyz(pos.x as f32, 0.7, pos.y as f32),
        Mesh::from(shape::Box::new(1., 0.1, 1.)),
        "ceiling.png",
        commands, meshes, materials, asset_server,
    ));

    // light
    if room_data.has_light {
        let entity = commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 20.,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(pos.x as f32, 0.64, pos.y as f32),
            ..default()
        }).id();
        entities.push(entity);
    }

    // x wall
    if room_data.walls.0 {
        let wall_plane = shape::Quad::new(Vec2 {x: 1., y: 0.7});

        let mut transform = Transform::from_xyz(pos.x as f32 + 0.5, 0.35, pos.y as f32);
        transform.rotate_local_y(PI / 2.);
        entities.push(fns::create_textured_mesh(
            transform,
            Mesh::from(wall_plane.clone()),
            "wall.png",
            commands, meshes, materials, asset_server,
        ));

        let mut transform = Transform::from_xyz(pos.x as f32 + 0.5, 0.35, pos.y as f32);
        transform.rotate_local_y(PI / -2.);
        entities.push(fns::create_textured_mesh(
            transform,
            Mesh::from(wall_plane),
            "wall.png",
            commands, meshes, materials, asset_server,
        ));

    }

    // z wall
    if room_data.walls.1 {
        let wall_plane = shape::Quad::new(Vec2 {x: 1., y: 0.7});

        let transform = Transform::from_xyz(pos.x as f32, 0.35, pos.y as f32 + 0.5);
        entities.push(fns::create_textured_mesh(
            transform,
            Mesh::from(wall_plane.clone()),
            "wall.png",
            commands, meshes, materials, asset_server,
        ));

        let mut transform = Transform::from_xyz(pos.x as f32, 0.35, pos.y as f32 + 0.5);
        transform.rotate_local_y(PI);
        entities.push(fns::create_textured_mesh(
            transform,
            Mesh::from(wall_plane),
            "wall.png",
            commands, meshes, materials, asset_server,
        ));

    }

    room_entities.insert(pos, entities);

}





pub fn remove_room_entities (grid: &mut HashMap<IVec2, Vec<Entity>>, slot_pos: IVec2, commands: &mut Commands) {

    let grid_slot = grid.get(&slot_pos).expect("cannot remove slot because it is not in the grid hashmap");

    for &physical_entity in grid_slot {
        commands.entity(physical_entity).despawn();
    }

    grid.remove(&slot_pos);

}





pub const GENERATION_PATTERN_RADIUS: i32 = GENERATION_PATTERN_SIZE as i32 + 1;
pub const GENERATION_PATTERN_LEN: usize = (GENERATION_PATTERN_SIZE * 2 + 1) * (GENERATION_PATTERN_SIZE * 2 + 1);
pub const GENERATION_PATTERN: [IVec2; GENERATION_PATTERN_LEN] = generate_generation_pattern(GENERATION_PATTERN_SIZE);

// size is kinda like radius, but for a square (`size: 4` creates a 9x9 square)
pub const fn generate_generation_pattern<const N: usize> (size: usize) -> [IVec2; N] {
    let mut output = [IVec2::ZERO; N];
    let mut index = 1;
    let mut layer = 1_i32;
    while layer as usize <= size {
        // top
        let y = -layer;
        let mut x = -layer;
        while x < layer {
            output[index] = IVec2 {x, y};
            x += 1;
            index += 1;
        }
        // right
        let x = layer;
        let mut y = -layer;
        while y < layer {
            output[index] = IVec2 {x, y};
            y += 1;
            index += 1;
        }
        // bottom
        let y = layer;
        let mut x = layer;
        while x > -layer {
            output[index] = IVec2 {x, y};
            x -= 1;
            index += 1;
        }
        // left
        let x = -layer;
        let mut y = layer;
        while y > -layer {
            output[index] = IVec2 {x, y};
            y -= 1;
            index += 1;
        }
        layer += 1;
    }
    output
}
