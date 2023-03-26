use crate::prelude::*;





pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FpsCount::default())
            .add_system(print_fps)
            .add_system(close);
    }
}





pub fn close (
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if (keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl)) && keys.just_pressed(KeyCode::W) {
        exit.send(AppExit);
    }
}



#[derive(Resource, SmartDefault)]
pub struct FpsCount (
    #[default(0)]
    u32,
    #[default(Instant::now())]
    Instant
);

pub fn print_fps (
    mut fps_count: ResMut<FpsCount>,
) {
    fps_count.0 += 1;
    if fps_count.1.elapsed() < Duration::SECOND {return;}
    println!("FPS: {}", fps_count.0);
    fps_count.0 = 0;
    fps_count.1 = Instant::now();
}
