use crate::prelude::*;





#[derive(Resource, SmartDefault)]
pub struct Player {
    #[default(Vec2::ZERO)]  pub pos: Vec2,
    #[default(IVec2::ZERO)] pub room: IVec2,
    #[default(Vec2::ZERO)]  pub camera_rot: Vec2,
}



#[derive(Resource, SmartDefault)]
pub struct PlayerRoomChange (
    #[default(u8::MAX)] pub u8,
);

impl EventSource for PlayerRoomChange {
    fn get_id (&self) -> u8 {
        self.0
    }
    fn get_id_mut (&mut self) -> &mut u8 {
        &mut self.0
    }
}
