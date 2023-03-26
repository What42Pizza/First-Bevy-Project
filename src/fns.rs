use crate::prelude::*;





pub fn chance (input: f64) -> bool {
    rand::random::<f64>() < input
}





pub fn create_textured_mesh (transform: Transform, mesh: Mesh, texture_name: &str, commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>, asset_server: &AssetServer) -> Entity {

    let texture = asset_server.load(texture_name);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(texture),
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform,
        ..default()
    }).id()
}
