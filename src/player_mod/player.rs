use crate::prelude::*;
use bevy::{input::mouse::MouseMotion, window::*};





pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Player::default())
            .insert_resource(PlayerRoomChange::default())
            .add_startup_system(init_player)
            .add_system(update_camera);
    }
}





pub fn init_player (
    mut commands: Commands
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}





pub fn update_camera (
    mut player: ResMut<Player>,
    mut player_room_change: ResMut<PlayerRoomChange>,
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    timer: Res<Time>,
) {

    let mut window = window.single_mut();
    if !window.focused {
        window.cursor.visible = true;
        return;
    }
    window.cursor.visible = false;
    let pos = Vec2 {x: window.width() / 2., y: window.height() / 2.};
    window.set_cursor_position(Some(pos));

    // rotate camera
    for event in mouse_motion_events.iter() {
        player.camera_rot.x += event.delta.x / 10. * CAMERA_ROTATE_SPEED * timer.delta_seconds();
        player.camera_rot.y += event.delta.y / 10. * CAMERA_ROTATE_SPEED * timer.delta_seconds();
        player.camera_rot.y = player.camera_rot.y.clamp(PI / -2., PI / 2.);
    }

    // move camera
    let mut movement_vec = Vec2 {x: 0., y: 0.};
    if keys.pressed(KeyCode::W) {
        movement_vec.y -= 1.;
    }
    if keys.pressed(KeyCode::A) {
        movement_vec.x -= 1.;
    }
    if keys.pressed(KeyCode::S) {
        movement_vec.y += 1.;
    }
    if keys.pressed(KeyCode::D) {
        movement_vec.x += 1.;
    }
    if movement_vec.length() > 0. {

        // speed
        let offset = Vec2 {x: 0., y: -0.25,};
        movement_vec = (movement_vec - offset).normalize() * CAMERA_MOVEMENT_SPEED + offset;
        movement_vec *= timer.delta_seconds();

        // direction
        let player_rot = player.camera_rot;
        let player_rot_vec = Vec2 {x: player_rot.x.cos(), y: player_rot.x.sin()};
        movement_vec = player_rot_vec.rotate(movement_vec);

        // apply
        player.pos += movement_vec;
        let new_room = player.pos.as_ivec2();
        if new_room != player.room {
            player.room = new_room;
            player_room_change.new_event();
        }

    }

    // update camera transform
    let mut new_transform = Transform::from_xyz(player.pos.x, 0.45, player.pos.y);
    new_transform.rotate_y(player.camera_rot.x * -1.);
    new_transform.rotate_local_x(player.camera_rot.y * -1.);
    *camera_transform.single_mut() = new_transform;

}
